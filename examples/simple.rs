use std::io;

use web::App;

fn main() -> io::Result<()> {
    let app = App::new("127.0.0.1:3000")?;
    app.listen();
    Ok(())
}
