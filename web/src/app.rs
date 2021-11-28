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
    route::Route,
    Request, Response,
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
    logging: bool,
}

impl App {
    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            endpoints: Vec::new(),
            logging: false,
        })
    }

    add_endpoint!(get, Method::GET);
    add_endpoint!(post, Method::POST);
    add_endpoint!(put, Method::PUT);
    add_endpoint!(delete, Method::DELETE);
    add_endpoint!(trace, Method::TRACE);
    add_endpoint!(patch, Method::PATCH);

    pub fn log(&mut self) {
        self.logging = true;
    }

    #[tokio::main]
    pub async fn listen(self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr).await?;

        // let endpoints = Arc::new(Mutex::new(self.endpoints));
        let slf = Arc::new(Mutex::new(self));
        loop {
            let (socket, _) = listener.accept().await?;
            // let endpoints = endpoints.clone();
            let slf = slf.clone();
            tokio::spawn(async move {
                App::handle_connection(slf, socket).await;
            });
        }
    }

    async fn handle_connection(slf: Arc<Mutex<Self>>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();
        let request = Request::new(&buffer);
        {
            let slf = slf.lock().unwrap();
            if slf.logging {
                println!("{:?}", request);
            }
        }

        let response = {
            let mut response = Response::default();
            let routes = &slf.lock().unwrap().endpoints;
            routes.iter().filter(|r| r.matches(&request)).for_each(|r| {
                response.status(200);
                let mut req = request.clone(); // TODO: without cloning
                req.populate_params(&r.route);
                (r.cb)(&req, &mut response)
            });
            response
        };
        stream
            .write(response.format_for_response().as_bytes())
            .await
            .unwrap();
        stream.flush().await.unwrap();
    }
}
