use std::{
    collections::HashMap,
    fs, io,
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time,
};

pub struct App {
    addr: SocketAddr,
    // gets: HashMap<String, A>,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> io::Result<Self> {
        let addr = addr.to_socket_addrs()?.find(|_| true).unwrap();

        Ok(Self {
            addr,
            // gets: HashMap::new(),
        })
    }

    // pub fn get(&mut self, route: String, handler: A) {
    //     self.gets.insert(route, handler);
    // }

    #[tokio::main]
    pub async fn listen(self) {
        let listener = TcpListener::bind(self.addr).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                handle_connection(socket).await;
            });
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "examples/hello.html")
    } else if buffer.starts_with(sleep) {
        time::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "examples/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "examples/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
