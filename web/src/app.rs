use std::{
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

    // TODO: macro generated methods
    pub fn get(&mut self, route: impl ToString, handler: fn(Request, &mut Response) -> ()) {
        self.endpoints
            .push(Endpoint::new(route.to_string(), Method::GET, handler));
    }
    pub fn put(&mut self, route: impl ToString, handler: fn(Request, &mut Response) -> ()) {
        self.endpoints
            .push(Endpoint::new(route.to_string(), Method::PUT, handler));
    }
    pub fn post(&mut self, route: impl ToString, handler: fn(Request, &mut Response) -> ()) {
        self.endpoints
            .push(Endpoint::new(route.to_string(), Method::POST, handler));
    }

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
            let mut response = Response::new();
            let routes = endpoints.lock().unwrap();
            (routes
                .iter()
                .find(|r| r.matches(&request))
                .unwrap_or(&Default::default())
                .cb)(request, &mut response);
            response
        };
        stream
            .write(response.format_for_response().as_bytes())
            .await
            .unwrap();
        stream.flush().await.unwrap();
    }
}
