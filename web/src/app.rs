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

use crate::{
    endpoint::{Endpoint, Method},
    route::Route,
    Request, Response,
};

pub struct App {
    addr: SocketAddr,
    cfg: Cb,
}

// For lack of a better name
pub struct Runtime {
    stream: TcpStream,
    logging: bool,
}

impl Runtime {
    async fn run(stream: TcpStream, cfg: Arc<Mutex<Cb>>) {
        let rt = Runtime {
            stream,
            logging: false,
        };
        let fut = {
            let cfg = cfg.lock().unwrap();
            cfg(rt)
        };
        fut.await;
    }

    async fn endpoint(
        &mut self,
        route: impl ToString,
        handler: fn(&Request, &mut Response) -> (),
        method: Method,
    ) {
        let mut buffer = [0; 1024];
        self.stream.read(&mut buffer).await.unwrap();
        let mut request = Request::new(&buffer);
        let endpoint = Endpoint::new(Route::from(route), method, handler); // TODO: is endpoint obsolete?

        if self.logging {
            println!("{:?}", request);
        }

        let response = {
            let mut response = Response::default();
            if endpoint.matches(&request) {
                response.status(200);
                request.populate_params(&endpoint.route);
                (handler)(&request, &mut response)
            }
            response
        };
        self.stream
            .write(response.format_for_response().as_bytes())
            .await
            .unwrap();
        self.stream.flush().await.unwrap();
    }
    pub async fn get(&mut self, route: impl ToString, handler: fn(&Request, &mut Response) -> ()) {
        self.endpoint(route, handler, Method::GET).await;
    }
    pub async fn post(&mut self, route: impl ToString, handler: fn(&Request, &mut Response) -> ()) {
        self.endpoint(route, handler, Method::POST).await;
    }
    pub async fn put(&mut self, route: impl ToString, handler: fn(&Request, &mut Response) -> ()) {
        self.endpoint(route, handler, Method::PUT).await;
    }
    pub async fn delete(
        &mut self,
        route: impl ToString,
        handler: fn(&Request, &mut Response) -> (),
    ) {
        self.endpoint(route, handler, Method::DELETE).await;
    }
    pub async fn trace(
        &mut self,
        route: impl ToString,
        handler: fn(&Request, &mut Response) -> (),
    ) {
        self.endpoint(route, handler, Method::TRACE).await;
    }
    pub async fn patch(
        &mut self,
        route: impl ToString,
        handler: fn(&Request, &mut Response) -> (),
    ) {
        self.endpoint(route, handler, Method::PATCH).await;
    }

    // TODO: Pass custom function or something?
    pub fn log(&mut self) {
        self.logging = true;
    }
}

type Cb = Box<dyn Fn(Runtime) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send>;

fn make_cb<T>(f: fn(Runtime) -> T) -> Cb
where
    T: Future<Output = ()> + Send + 'static,
{
    Box::new(move |rt| Box::pin(f(rt)))
}

impl App {
    pub fn new<A: ToSocketAddrs, T>(addr: A, cfg: fn(Runtime) -> T) -> io::Result<Self>
    where
        T: Future<Output = ()> + Send + 'static,
    {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            cfg: make_cb(cfg),
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
