use crate::endpoint::Method;

pub struct Request {
    pub method: Method,
    pub route: String,
    // TODO: These are not all
}

impl Request {
    pub fn new(buffer: &[u8; 1024]) -> Self {
        // TODO: is this memory efficient?
        let req = String::from_utf8(buffer.to_vec()).unwrap();
        let mut parts = req.split(' ');
        let method = match parts.next() {
            Some("GET") => Method::GET,
            Some("PUT") => Method::PUT,
            Some("POST") => Method::POST,
            _ => panic!(),
        };

        Request {
            method,
            route: parts.next().unwrap().to_string(),
        }
    }
}
