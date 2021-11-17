pub enum Method {
    GET,
    POST,
    PUT,
    // ...
}

pub struct Endpoint {
    route: String,
    method: Method,
    cb: Box<dyn Fn() -> String + Send + 'static>,
}

impl Endpoint {
    pub fn new<T: Send + Fn() -> String + 'static>(route: String, method: Method, cb: T) -> Self {
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
        (self.cb)()
    }
}