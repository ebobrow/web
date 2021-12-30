use std::io;

use web::{App, Request, Response, StatusCode};

// TODO: This kind of works but `app` isn't strongly typed
// #[web::main]
// fn main(app: Runtime) {
//     app.log_with(|_| println!("special logger for home route"));
//     app.get("/", home);

//     app.log(); // Turn on default logger
//     app.get("/a", a);
//     app.get("/a", a2);
//     app.get("/user/:name", user);
//     app.post("/", post);
// }

fn main() -> io::Result<()> {
    App::new("127.0.0.1:3000", |mut app| async move {
        app.log_with(|_| println!("special logger for home route"));
        app.get("/", home);

        app.log(); // Turn on default logger
        app.get("/a", a);
        app.get("/a", a2);
        app.get("/user/:name", user);
        app.post("/", post);

        app.listen().await; // TODO: Don't want this
    })
}

async fn home(_: Request, mut res: Response) -> Response {
    res.serve_file("examples/static/hello.html");
    res // TODO: can this be mutated in place?
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
    res.content(format!("Hello, {}", name))
        .status(StatusCode::OK);
    res
}

async fn post(req: Request, mut res: Response) -> Response {
    res.status(StatusCode::OK)
        .content(format!("Your username is {}", req.body["username"]));
    res
}
