use std::io;

use web::{App, Request, Response, StatusCode};

fn main() -> io::Result<()> {
    App::new("127.0.0.1:3000", |mut app| async move {
        app.log_with(|_| println!("special logger for home route"));
        app.get("/", home).await; // TODO: Don't want `.await`

        app.log(); // Turn on default logger
        app.get("/a", a).await;
        app.get("/a", a2).await;
        app.get("/user/:name", user).await;

        app.listen().await; // TODO: Don't want this
    })
}

async fn home(_: Request, mut res: Response) -> Response {
    res.serve_file("testing/hello.html");
    res
}

async fn a(_: Request, mut res: Response) -> Response {
    res.status(StatusCode::OK)
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
