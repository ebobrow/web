use std::{
    io,
    net::{SocketAddr, ToSocketAddrs},
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::endpoint::{Endpoint, Method};

pub struct App {
    addr: SocketAddr,
    // gets: Arc<Mutex<HashMap<String, String>>>,
    // gets: HashMap<String, tokio::task::JoinHandle<String>>,
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

    // pub fn get(&mut self, route: impl ToString, handler: Box<dyn Fn() -> String>) {
    //     // TODO: Run functions after request so that Request object can be passed
    //     let mut gets = self.gets.lock().unwrap();
    //     gets.insert(
    //         format!("GET {} HTTP/1.1\r\n", route.to_string()),
    //         Response::new(handler(), 200).format_for_response(),
    //     );
    // }
    pub fn get<T: Fn() -> String + Send + 'static>(&mut self, route: impl ToString, handler: T) {
        self.endpoints
            .push(Endpoint::new(route.to_string(), Method::GET, handler));
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

        let response = {
            let routes = endpoints.lock().unwrap();
            let (status_line, contents) = routes.iter().find(|r| r.matches(&buffer)).map_or(
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", String::new()),
                |endpoint| ("HTTP/1.1 200 OK\r\n\r\n", endpoint.invoke()),
            );

            format!("{}{}", status_line, contents)
        };
        stream.write(response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}
