use std::io;

use web::{request::Request, response::Response, App};

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get("/", home);
    app.get("/a", a);
    app.get("/a", a2);
    app.listen();
    Ok(())
}

fn home(_: &Request, res: &mut Response) {
    println!("GET to /");
    res.serve_file("testing/hello.html");
}

fn a(_: &Request, res: &mut Response) {
    println!("first GET to /a");
    res.status(200)
        .content("you will never see this".to_string());
}

fn a2(_: &Request, res: &mut Response) {
    println!("second GET to /a");
    res.content("hi".to_string());
}
