use std::{fs, io};

use web::App;

fn main() -> io::Result<()> {
    let mut app = App::new("127.0.0.1:3000")?;
    app.get(
        "/",
        Box::new(|| {
            let status_line = "HTTP/1.1 200 OK";
            let file = fs::read_to_string("examples/hello.html").unwrap();

            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                file.len(),
                file
            )
        }),
    );
    app.listen();
    Ok(())
}
