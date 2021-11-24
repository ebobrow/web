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
    route::Route,
};

macro_rules! add_endpoint {
    ($name:ident, $method:path) => {
        pub fn $name(&mut self, route: impl ToString, handler: fn(&Request, &mut Response) -> ()) {
            self.endpoints
                .push(Endpoint::new(Route::from(route), $method, handler));
        }
    };
}

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

    add_endpoint!(get, Method::GET);
    add_endpoint!(head, Method::HEAD);
    add_endpoint!(post, Method::POST);
    add_endpoint!(put, Method::PUT);
    add_endpoint!(delete, Method::DELETE);
    add_endpoint!(connect, Method::CONNECT);
    add_endpoint!(options, Method::OPTIONS);
    add_endpoint!(trace, Method::TRACE);
    add_endpoint!(patch, Method::PATCH);

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
            let mut response = Response::default();
            let routes = endpoints.lock().unwrap();
            routes
                .iter()
                .filter(|r| r.matches(&request))
                .for_each(|r| (r.cb)(&request, &mut response));
            response
        };
        stream
            .write(response.format_for_response().as_bytes())
            .await
            .unwrap();
        stream.flush().await.unwrap();
    }
}
