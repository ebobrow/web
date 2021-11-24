use std::io;

use web::{request::Request, response::Response, App};

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get("/", home);
    app.get("/e*", egg);
    app.listen();
    Ok(())
}

fn home(_: Request, res: &mut Response) {
    println!("GET to /");
    res.serve_file("testing/hello.html");
}

fn egg(_: Request, res: &mut Response) {
    println!("GET to /e*");
    res.content("{ field: 123, field2: 1234 }".to_string());
}
