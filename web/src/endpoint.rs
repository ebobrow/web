use crate::{request::Request, response::Response};

type Cb = Box<dyn Fn(Request, &mut Response) -> () + Send>;

fn make_cb(f: fn(Request, &mut Response) -> ()) -> Cb {
    Box::new(move |req, res| f(req, res))
}

#[derive(PartialEq)]
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET
    /// should only retrieve data.
    GET,

    /// The HEAD method asks for a response identical to a GET request, but without the response
    /// body.
    HEAD,

    /// The POST method submits an entity to the specified resource, often causing a change in
    /// state or side effects on the server.
    POST,

    /// The PUT method replaces all current representations of the target resource with the request
    /// payload.
    PUT,

    /// The DELETE method deletes the specified resource.
    DELETE,

    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    CONNECT,

    /// The OPTIONS method describes the communication options for the target resource.
    OPTIONS,

    /// The TRACE method performs a message loop-back test along the path to the target resource.
    TRACE,

    /// The PATCH method applies partial modifications to a resource.
    PATCH,
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
