use std::io;

use web::{response::Response, App};

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get("/", || {
        println!("GET to /");
        Response::new().serve_file("testing/hello.html")
    });
    app.get("/egg", || {
        println!("GET to /egg");
        Response::new().content("{ field: 123, field2: 1234 }".to_string())
    });
    app.listen();
    Ok(())
}
