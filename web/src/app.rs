use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, ToSocketAddrs},
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::response::Response;

pub struct App {
    addr: SocketAddr,
    gets: Arc<Mutex<HashMap<String, String>>>,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> io::Result<Self> {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            // gets: Arc::new(Mutex::new(HashMap::new())),
            gets: Default::default(),
        })
    }

    // TODO: Use macros instead, like:
    // #[web::get("/")]
    // fn home() { /* ... */ }
    pub fn get(&mut self, route: impl ToString, handler: Box<dyn Fn() -> String>) {
        // TODO: Run functions after request so that Request object can be passed
        let mut gets = self.gets.lock().unwrap();
        gets.insert(
            format!("GET {} HTTP/1.1\r\n", route.to_string()),
            Response::new(handler(), 200).format_for_response(),
        );
    }

    #[tokio::main]
    pub async fn listen(self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();

        let slf = Arc::new(Mutex::new(self));
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let slf = slf.clone();
            tokio::spawn(async move {
                App::handle_connection(slf, socket).await;
            });
        }
    }

    async fn handle_connection(slf: Arc<Mutex<Self>>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();

        let response = {
            let slf = slf.lock().unwrap();
            let gets = slf.gets.lock().unwrap();
            match gets
                .clone()
                .into_iter()
                .find(|(k, _)| buffer.starts_with(k.as_bytes()))
            {
                Some((_, res)) => res,
                None => String::new(),
            }
        };

        stream.write(response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}
