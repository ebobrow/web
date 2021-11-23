use std::io;

use web::{request::Request, response::Response, App};

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get("/", home);
    // app.get("/egg", egg);
    app.listen();
    Ok(())
}

async fn home(req: Request) -> Response {
    println!("GET to /");
    res.serve_file("testing/hello.html");
    res
}

async fn egg(req: Request, res: &mut Response) {
    println!("GET to /egg");
    res.content("{ field: 123, field2: 1234 }".to_string());
}
