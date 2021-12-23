use std::{collections::HashMap, fmt::Debug};

use crate::{app::Method, route::Route};

#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    pub params: HashMap<String, String>,
    // TODO: These are not all
}

impl Request {
    pub fn populate_params(&mut self, route: &Route) {
        self.params = route.params(self);
    }
}

impl From<&[u8; 1024]> for Request {
    fn from(buffer: &[u8; 1024]) -> Self {
        Self::from(String::from_utf8(buffer.to_vec()).unwrap())
    }
}

impl From<String> for Request {
    fn from(req: String) -> Self {
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

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} to {:?}", self.method, self.route))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_buf() {
        let request = Request::from(String::from("GET /"));
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.route, Route { segments: vec![] });

        let request = Request::from(String::from("DELETE /a/b/c/"));
        assert_eq!(request.method, Method::DELETE);
        assert_eq!(
            request.route,
            Route {
                segments: vec![String::from("a"), String::from("b"), String::from("c")]
            }
        );
    }

    #[test]
    fn debug() {
        let debug = format!("{:?}", Request::from("POST /db".to_string()));
        assert_eq!(debug, "POST to /db");

        let debug = format!("{:?}", Request::from("PATCH /".to_string()));
        assert_eq!(debug, "PATCH to /");
    }
}
