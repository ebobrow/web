use std::io;

use web::{App, Request, Response};

fn main() -> io::Result<()> {
    let app = App::new("127.0.0.1:3000", |mut rt| async move {
        rt.log_with(|_| println!("special logger for home route"));
        rt.get("/", home).await; // TODO: Don't want `.await`

        rt.log(); // Turn on default logger
        rt.get("/a", a).await;
        rt.get("/a", a2).await;
        rt.get("/user/:name", user).await;

        rt.end().await; // TODO: Don't want this
    })?;
    app.listen()
}

async fn home(_: Request, mut res: Response) -> Response {
    res.serve_file("testing/hello.html");
    res
}

async fn a(_: Request, mut res: Response) -> Response {
    res.status(200)
        .content("you will never see this".to_string());
    res
}

async fn a2(_: Request, mut res: Response) -> Response {
    res.content("hi".to_string());
    res
}

async fn user(req: Request, mut res: Response) -> Response {
    let name = req.params.get("name").unwrap();
    res.content(format!("Hello, {}", name)).status(200);
    res
}
