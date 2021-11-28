use crate::{route::Route, Request, Response};

type Cb = Box<dyn Fn(&Request, &mut Response) -> () + Send>;

fn make_cb(f: fn(&Request, &mut Response) -> ()) -> Cb {
    Box::new(move |req, res| f(req, res))
}

#[derive(PartialEq, Clone)]
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET
    /// should only retrieve data.
    GET,

    /// The POST method submits an entity to the specified resource, often causing a change in
    /// state or side effects on the server.
    POST,

    /// The PUT method replaces all current representations of the target resource with the request
    /// payload.
    PUT,

    /// The DELETE method deletes the specified resource.
    DELETE,

    /// The TRACE method performs a message loop-back test along the path to the target resource.
    TRACE,

    /// The PATCH method applies partial modifications to a resource.
    PATCH,
}

pub struct Endpoint {
    pub route: Route,
    method: Method,
    pub cb: Cb,
}

impl Endpoint {
    pub fn new(route: Route, method: Method, cb: fn(&Request, &mut Response) -> ()) -> Self {
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
