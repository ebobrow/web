use std::io;

use web::{App, Request, Response};

fn main() -> io::Result<()> {
    let app = App::new("127.0.0.1:3000", |mut rt| async move {
        rt.get("/", home).await;
        rt.log();

        // TODO: It seems only the first one is sent as a response
        rt.get("/a", a2).await;
        rt.get("/a", a).await;
        rt.get("/user/:name", user).await;
    })?;
    app.listen()
}

async fn home(_: Request) -> Response {
    let mut res = Response::new();
    res.serve_file("testing/hello.html");
    res
}

async fn a(_: Request) -> Response {
    let mut res = Response::new();
    println!("hi from a1");
    res.status(200)
        .content("you will never see this".to_string());
    res
}

async fn a2(_: Request) -> Response {
    let mut res = Response::new();
    println!("hi from a2");
    res.content("hi".to_string());
    res
}

async fn user(req: Request) -> Response {
    let mut res = Response::new();
    let name = req.params.get("name").unwrap();
    res.content(format!("Hello, {}", name));
    res
}
