use std::{
    future::Future,
    io,
    net::{SocketAddr, ToSocketAddrs},
    pin::Pin,
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{route::Route, Request, Response};

#[derive(PartialEq, Clone, Debug)]
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET
    /// should only retrieve data.
    GET,

    /// The POST method submits an entity to the specified resource, often causing a change in
    /// state or side effects on the server.
    POST,

    /// The PUT method replaces all current representations of the target resource with the request
    /// payload.
    PUT,

    /// The DELETE method deletes the specified resource.
    DELETE,

    /// The TRACE method performs a message loop-back test along the path to the target resource.
    TRACE,

    /// The PATCH method applies partial modifications to a resource.
    PATCH,
}

type Cfg = Box<dyn Fn(Runtime) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send>;
fn make_cfg<T>(f: fn(Runtime) -> T) -> Cfg
where
    T: Future<Output = ()> + Send + 'static,
{
    Box::new(move |rt| Box::pin(f(rt)))
}

type Handler =
    Box<dyn Fn(Request, Response) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;
fn make_handler<T>(f: fn(Request, Response) -> T) -> Handler
where
    T: Future<Output = Response> + Send + 'static,
{
    Box::new(move |req, res| Box::pin(f(req, res)))
}

macro_rules! add_endpoint {
    ($name:ident, $method:path) => {
        pub fn $name<T>(&mut self, route: impl ToString, handler: fn(Request, Response) -> T)
        where
            T: Future<Output = Response> + Send + 'static,
        {
            self.endpoint(route, make_handler(handler), $method);
        }
    };
}

pub struct App {
    addr: SocketAddr,
    cfg: Cfg,
}

// For lack of a better name
pub struct Runtime {
    stream: TcpStream,
    logging: Option<Box<dyn Fn(&Request) + Send>>,
    identified: bool,
    request: Request,
    response: Pin<Box<(dyn Future<Output = Response> + Send + 'static)>>,
}

impl Runtime {
    async fn run(mut stream: TcpStream, cfg: Arc<Mutex<Cfg>>) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let rt = Runtime {
            stream,
            logging: None,
            identified: false,
            request: Request::new(&buffer),
            response: Box::pin(Response::default_async()),
        };

        let fut = {
            let cfg = cfg.lock().unwrap();
            cfg(rt)
        };
        fut.await;
    }

    pub async fn listen(&mut self) {
        if !self.identified {
            self.log_route(); // TODO: Can we do something special knowing it's 404?
        }

        let res = std::mem::replace(&mut self.response, Box::pin(Response::default_async()));
        self.stream
            .write(res.await.format_for_response().as_bytes())
            .await
            .unwrap();
        self.stream.flush().await.unwrap();
    }

    // Bad naming again
    fn log_route(&self) {
        if let Some(logger) = &self.logging {
            logger(&self.request);
        }
    }

    fn endpoint(&mut self, route: impl ToString, handler: Handler, method: Method) {
        let route = Route::from(route);

        if route == self.request.route && method == self.request.method {
            self.identified = true;
            self.log_route();

            let mut req = self.request.clone();
            req.populate_params(&route);
            let res = std::mem::replace(&mut self.response, Box::pin(Response::default_async()));
            self.response = Box::pin((|| async move { (handler)(req, res.await).await })());
        }
    }
    add_endpoint!(get, Method::GET);
    add_endpoint!(post, Method::POST);
    add_endpoint!(put, Method::PUT);
    add_endpoint!(delete, Method::DELETE);
    add_endpoint!(trace, Method::TRACE);
    add_endpoint!(patch, Method::PATCH);

    pub fn log(&mut self) {
        self.logging = Some(Box::new(|req| {
            println!("{:?}", req);
        }));
    }

    pub fn log_with(&mut self, logger: fn(&Request)) {
        self.logging = Some(Box::new(logger));
    }
}

impl App {
    #[tokio::main]
    pub async fn new<A: ToSocketAddrs, T>(addr: A, cfg: fn(Runtime) -> T) -> io::Result<()>
    where
        T: Future<Output = ()> + Send + 'static,
    {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        let app = Self {
            addr,
            cfg: make_cfg(cfg),
        };

        let listener = TcpListener::bind(app.addr).await?;

        let cfg = Arc::new(Mutex::new(app.cfg));
        loop {
            let (socket, _) = listener.accept().await?;
            let cfg = cfg.clone();
            tokio::spawn(async move {
                Runtime::run(socket, cfg).await;
            });
        }
    }
}
