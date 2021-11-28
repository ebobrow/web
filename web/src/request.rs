use std::collections::HashMap;

use crate::{endpoint::Method, route::Route};

#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    pub params: HashMap<String, String>,
    // TODO: These are not all
}

impl Request {
    pub fn new(buffer: &[u8; 1024]) -> Self {
        // TODO: is this memory efficient?
        let req = String::from_utf8(buffer.to_vec()).unwrap();
        let mut parts = req.split(' ');
        let method = match parts.next() {
            Some("GET") => Method::GET,
            Some("POST") => Method::POST,
            Some("PUT") => Method::PUT,
            Some("DELETE") => Method::DELETE,
            Some("TRACE") => Method::TRACE,
            Some("PATCH") => Method::PATCH,
            _ => panic!(),
        };

        Request {
            method,
            route: Route::from(parts.next().unwrap()),
            params: HashMap::new(),
        }
    }
}
