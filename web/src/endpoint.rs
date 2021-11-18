use crate::response::Response;

pub enum Method {
    GET,
    POST,
    PUT,
    // ...
}

pub struct Endpoint {
    route: String,
    method: Method,
    // TODO: Can this be encapsulated in a type/trait?
    cb: Box<dyn Fn(&mut Response) -> () + Send + 'static>,
}

impl Endpoint {
    pub fn new<T: Send + Fn(&mut Response) -> () + 'static>(
        route: String,
        method: Method,
        cb: T,
    ) -> Self {
        Endpoint {
            route,
            method,
            cb: Box::new(cb),
        }
    }
    pub fn matches(&self, buf: &[u8; 1024]) -> bool {
        match self.method {
            Method::GET => buf.starts_with(format!("GET {} HTTP/1.1\r\n", self.route).as_bytes()),
            Method::POST => buf.starts_with(format!("POST {} HTTP/1.1\r\n", self.route).as_bytes()),
            Method::PUT => buf.starts_with(format!("PUT {} HTTP/1.1\r\n", self.route).as_bytes()),
        }
    }

    pub fn invoke(&self) -> String {
        let mut res = Response::new();
        (self.cb)(&mut res);
        res.format_for_response()
    }
}
