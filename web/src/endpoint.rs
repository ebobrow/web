use std::{future::Future, pin::Pin};

use crate::{request::Request, response::Response};

type Cb = Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response>>> + Send>;

fn make_cb<T>(f: fn(Request) -> T) -> Cb
where
    T: Future<Output = Response> + 'static,
{
    Box::new(move |req| Box::pin(f(req)))
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
    // TODO: Can this be encapsulated in a type/trait?
    // cb: Box<dyn Fn(&Request, &mut Response) -> () + Send + 'static>,
    pub cb: Cb,
}

impl Endpoint {
    pub fn new<T>(route: String, method: Method, cb: fn(Request) -> T) -> Self
    where
        T: Future<Output = Response> + 'static,
    {
        Endpoint {
            route,
            method,
            // cb: Box::new(cb),
            cb: make_cb(cb),
        }
    }
    pub fn matches(&self, request: &Request) -> bool {
        self.method == request.method && self.route == request.route
    }

    // pub async fn invoke(&self, req: Request) -> String {
    //     // let mut res = Response::new();
    //     // (self.cb)(req, res);
    //     // res.format_for_response()
    //     (self.cb)(req).await.format_for_response()
    // }
}
