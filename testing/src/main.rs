use std::io;

use web::{App, Request, Response};

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    // TODO: process these in order (log only applies to things below it)
    // possibly put these in closure that runs on every request and change functions like .get() to
    // mean run function if applies
    app.log();
    app.get("/", home);
    app.get("/a", a);
    app.get("/a", a2);
    app.get("/user/:name", user);
    app.listen()
}

fn home(_: &Request, res: &mut Response) {
    // println!("GET to /");
    res.serve_file("testing/hello.html");
}

fn a(_: &Request, res: &mut Response) {
    // println!("first GET to /a");
    res.status(200)
        .content("you will never see this".to_string());
}

fn a2(_: &Request, res: &mut Response) {
    // println!("second GET to /a");
    res.content("hi".to_string());
}

fn user(req: &Request, res: &mut Response) {
    // println!("GET to /user/:name");
    let name = req.params.get("name").unwrap();
    res.content(format!("Hello, {}", name));
}
