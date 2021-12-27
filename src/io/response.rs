use std::{fs, path::Path};

use crate::{io::status::Status, StatusCode};

#[derive(Clone)]
pub struct Response {
    content: String,
    status: Status,
}

impl Response {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            status: Status::from(StatusCode::OK),
        }
    }

    pub fn serve_file<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.content = match fs::read_to_string(path) {
            Ok(content) => {
                self.status = Status::from(StatusCode::OK);
                content
            }
            Err(_) => {
                self.status = Status::from(StatusCode::NotFound);
                fs::read_to_string("static/404.html").unwrap()
            }
        };
        self
    }

    pub fn status(&mut self, status: impl Into<Status>) -> &mut Self {
        self.status = status.into();
        self
    }

    pub fn status_num(&mut self, status: usize) -> Result<&mut Self, &'static str> {
        self.status = status.try_into()?;
        Ok(self)
    }

    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    pub fn format_for_response(&self) -> String {
        let status_line = format!("HTTP/1.1 {}", self.status);
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            self.content.len(),
            self.content
        )
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        self.format_for_response()
    }
}

impl Default for Response {
    fn default() -> Self {
        let mut res = Response::new();
        res.serve_file("static/404.html")
            .status(StatusCode::NotFound);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_response() {
        let content = String::from("hello");

        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";

        assert_eq!(
            Response::new().content(content).format_for_response(),
            expected
        );
    }
}
