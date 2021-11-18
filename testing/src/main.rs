use std::io;

use web::App;

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get("/", |res| {
        println!("GET to /");
        res.serve_file("testing/hello.html");
    });
    app.get("/egg", |res| {
        println!("GET to /egg");
        res.content("{ field: 123, field2: 1234 }".to_string());
    });
    app.listen();
    Ok(())
}
