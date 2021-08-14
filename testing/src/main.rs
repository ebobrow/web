use std::{fs, io};

use web::App;

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get(
        "/",
        Box::new(|| fs::read_to_string("testing/hello.html").unwrap()),
    );
    app.listen();
    Ok(())
}
