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

type Handler = Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;
fn make_handler<T>(f: fn(Request) -> T) -> Handler
where
    T: Future<Output = Response> + Send + 'static,
{
    Box::new(move |req| Box::pin(f(req)))
}

macro_rules! add_endpoint {
    ($name:ident, $method:path) => {
        pub async fn $name<T>(&mut self, route: impl ToString, handler: fn(Request) -> T)
        where
            T: Future<Output = Response> + Send + 'static,
        {
            self.endpoint(route, make_handler(handler), $method).await;
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
    request: Request,
}

impl Runtime {
    async fn run(mut stream: TcpStream, cfg: Arc<Mutex<Cfg>>) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let rt = Runtime {
            stream,
            logging: None,
            request: Request::new(&buffer),
        };
        let fut = {
            let cfg = cfg.lock().unwrap();
            cfg(rt)
        };
        fut.await;
    }

    async fn endpoint(&mut self, route: impl ToString, handler: Handler, method: Method) {
        let route = Route::from(route);

        if route == self.request.route && method == self.request.method {
            if let Some(logger) = &self.logging {
                logger(&self.request);
            }

            let mut req = self.request.clone();
            req.populate_params(&route);
            let response = (handler)(req).await;
            self.stream
                .write(response.format_for_response().as_bytes())
                .await
                .unwrap();
            self.stream.flush().await.unwrap();
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
    pub fn new<A: ToSocketAddrs, T>(addr: A, cfg: fn(Runtime) -> T) -> io::Result<Self>
    where
        T: Future<Output = ()> + Send + 'static,
    {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            cfg: make_cfg(cfg),
        })
    }

    #[tokio::main]
    pub async fn listen(self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;

        let cfg = Arc::new(Mutex::new(self.cfg));
        loop {
            let (socket, _) = listener.accept().await?;
            let cfg = cfg.clone();
            tokio::spawn(async move {
                Runtime::run(socket, cfg).await;
            });
        }
    }
}
