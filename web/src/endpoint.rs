use crate::{request::Request, response::Response};

type Cb = Box<dyn Fn(Request) -> Response + Send>;

fn make_cb(f: fn(Request) -> Response) -> Cb {
    Box::new(move |req| f(req))
}

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
    pub cb: Cb,
}

impl Endpoint {
    pub fn new(route: String, method: Method, cb: fn(Request) -> Response) -> Self {
        Endpoint {
            route,
            method,
            cb: make_cb(cb),
        }
    }
    pub fn matches(&self, request: &Request) -> bool {
        self.method == request.method && self.route == request.route
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint {
            route: String::from("*"),
            method: Method::GET,
            cb: make_cb(default),
        }
    }
}

fn default(_: Request) -> Response {
    let mut res = Response::new();
    res.status(404).serve_file("web/static/404.html");
    // TODO: allow chaining like return Response::new().status(404).serve_file(...);
    res
}
