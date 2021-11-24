use crate::{request::Request, response::Response};

type Cb = Box<dyn Fn(Request, &mut Response) -> () + Send>;

fn make_cb(f: fn(Request, &mut Response) -> ()) -> Cb {
    Box::new(move |req, res| f(req, res))
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
    pub fn new(route: String, method: Method, cb: fn(Request, &mut Response) -> ()) -> Self {
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

fn default(_: Request, res: &mut Response) {
    res.serve_file("web/static/404.html").status(404);
}
// fn default(_: Request) -> Response {
//     Response::new()
//         .serve_file("web/static/404.html")
//         .status(404)
// }
