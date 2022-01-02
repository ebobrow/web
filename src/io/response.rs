use std::{collections::HashMap, fs, path::Path};

use crate::{cookie::Cookie, io::status::Status, StatusCode};

#[derive(Clone)]
pub struct Response {
    content: String,
    status: Status,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            status: Status::from(StatusCode::OK),
            headers: HashMap::new(),
        }
    }

    pub fn serve_file<P>(mut self, path: P) -> Self
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

    pub fn status(mut self, status: impl Into<Status>) -> Self {
        self.status = status.into();
        self
    }

    pub fn status_num(mut self, status: usize) -> Result<Self, &'static str> {
        self.status = status.try_into()?;
        Ok(self)
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn set_cookie(mut self, cookie: Cookie) -> Self {
        self.headers
            .insert(String::from("Set-Cookie"), cookie.as_header());
        self
    }

    pub fn delete_cookie(mut self, name: impl ToString) -> Self {
        self.headers.insert(
            String::from("Set-Cookie"),
            format!(
                "{}=; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
                name.to_string()
            ),
        );
        self
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let status_line = format!("HTTP/1.1 {}", self.status);
        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("\n{}: {}", k, v))
            .collect();
        format!(
            "{}\r\nContent-Length: {}{}\r\n\r\n{}",
            status_line,
            self.content.len(),
            headers,
            self.content
        )
    }
}

impl Default for Response {
    fn default() -> Self {
        Response::new()
            .serve_file("static/404.html")
            .status(StatusCode::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_response() {
        let content = String::from("hello");
        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
        assert_eq!(Response::new().content(content).to_string(), expected);

        let expected =
            "HTTP/1.1 200 OK\r\nContent-Length: 0\nSet-Cookie: key=value; SameSite=Lax\r\n\r\n";
        assert_eq!(
            expected,
            Response::new()
                .set_cookie(Cookie::new("key", "value"))
                .to_string()
        );
    }
}
