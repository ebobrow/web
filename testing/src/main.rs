use std::io;

use web::{App, Request, Response};

fn main() -> io::Result<()> {
    let app = App::new("127.0.0.1:3000", |mut rt| async move {
        rt.get("/", home).await;
        rt.log();
        rt.get("/a", a).await;
        rt.get("/a", a2).await;
        rt.get("/user/:name", user).await;
    })?;
    app.listen()
}

fn home(_: &Request, res: &mut Response) {
    res.serve_file("testing/hello.html");
}

fn a(_: &Request, res: &mut Response) {
    res.status(200)
        .content("you will never see this".to_string());
}

fn a2(_: &Request, res: &mut Response) {
    res.content("hi".to_string());
}

fn user(req: &Request, res: &mut Response) {
    let name = req.params.get("name").unwrap();
    res.content(format!("Hello, {}", name));
}
