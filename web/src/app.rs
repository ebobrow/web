use std::{
    future::Future,
    io,
    net::{SocketAddr, ToSocketAddrs},
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{
    endpoint::{Endpoint, Method},
    request::Request,
    response::Response,
};

pub struct App {
    addr: SocketAddr,
    endpoints: Vec<Endpoint>,
}

impl App {
    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            endpoints: Vec::new(),
        })
    }

    // TODO: Use macros instead, like:
    // #[web::get("/")]
    // async fn home() { /* ... */ }
    pub fn get<T>(&mut self, route: impl ToString, handler: fn(Request) -> T)
    where
        T: Future<Output = Response> + 'static,
    {
        self.endpoints
            .push(Endpoint::new(route.to_string(), Method::GET, handler));
    }
    // pub fn put(&mut self, route: impl ToString, handler: Cb) {
    //     self.endpoints
    //         .push(Endpoint::new(route.to_string(), Method::PUT, handler));
    // }
    // pub fn post(&mut self, route: impl ToString, handler: Cb) {
    //     self.endpoints
    //         .push(Endpoint::new(route.to_string(), Method::POST, handler));
    // }

    #[tokio::main]
    pub async fn listen(self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();

        let endpoints = Arc::new(Mutex::new(self.endpoints));
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let endpoints = endpoints.clone();
            tokio::spawn(async move {
                App::handle_connection(endpoints, socket).await;
            });
        }
    }

    async fn handle_connection(endpoints: Arc<Mutex<Vec<Endpoint>>>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let request = Request::new(&buffer);

        let response = {
            let routes = endpoints.lock().unwrap();
            (routes.iter().find(|r| r.matches(&request)).unwrap().cb)(request)
            // routes.iter().find(|r| r.matches(&request)).unwrap().cb
        };
        stream
            .write(response.await.format_for_response().as_bytes())
            .await
            .unwrap();
        stream.flush().await.unwrap();
    }
}
