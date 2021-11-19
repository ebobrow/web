use crate::{request::Request, response::Response};

#[derive(PartialEq)]
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
    cb: Box<dyn Fn(&Request, &mut Response) -> () + Send + 'static>,
}

impl Endpoint {
    pub fn new<T: Send + Fn(&Request, &mut Response) -> () + 'static>(
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
    pub fn matches(&self, request: &Request) -> bool {
        self.method == request.method && self.route == request.route
    }

    pub fn invoke(&self, req: &Request) -> String {
        let mut res = Response::new();
        (self.cb)(req, &mut res);
        res.format_for_response()
    }
}
