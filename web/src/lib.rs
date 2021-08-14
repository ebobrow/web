use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, ToSocketAddrs},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::response::Response;

pub mod response;

// TODO: Extract to app file
pub struct App {
    addr: SocketAddr,
    // gets: HashMap<String, Box<dyn Fn() -> String>>,
    gets: HashMap<String, String>,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> io::Result<Self> {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            gets: HashMap::new(),
        })
    }

    // TODO: Use macros instead, like:
    // #[web::get("/")]
    // fn home() { /* ... */ }
    pub fn get(&mut self, route: impl ToString, handler: Box<dyn Fn() -> String>) {
        // TODO: Run functions after request so that Request object can be passed
        self.gets.insert(
            format!("GET {} HTTP/1.1\r\n", route.to_string()),
            Response::new(handler(), 200).format_for_response(),
        );
    }

    #[tokio::main]
    pub async fn listen(self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let gets = self.gets.clone(); // TODO: Better way?
            tokio::spawn(async move {
                handle_connection(socket, gets).await;
            });
        }
    }
}

// TODO: Make method on `App` instead of passing gets
async fn handle_connection(mut stream: TcpStream, gets: HashMap<String, String>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let response = match gets
        .into_iter()
        .find(|(k, _)| buffer.starts_with(k.as_bytes()))
    {
        Some((_, res)) => res,
        None => String::new(),
    };

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

// TODO: Write tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
