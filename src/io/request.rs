use std::{collections::HashMap, fmt::Debug};

use serde_json::Value;

use crate::{app::Method, route::Route};

#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Value,
}

impl Request {
    pub fn populate_params(&mut self, route: &Route) {
        self.params = route.params(self);
    }
}

impl From<&[u8; 1024]> for Request {
    fn from(buffer: &[u8; 1024]) -> Self {
        Self::from(String::from_utf8(buffer.to_vec()).unwrap())
    }
}

// TODO: TryFrom?
impl From<String> for Request {
    fn from(req: String) -> Self {
        let mut lines = req.lines();
        let mut header = lines.next().unwrap().split(' ');
        let method = header.next().unwrap();
        let route = header.next().unwrap();

        let mut headers = HashMap::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break; // End of headers
            }

            let (key, value) = line.split_once(':').unwrap();
            let mut value = value.to_lowercase();
            value.retain(|c| !c.is_whitespace());
            headers.insert(key.to_lowercase(), value);
        }

        let body = if let Some("application/json") = headers.get("content-type").map(|s| &s[..]) {
            let json: String = lines.collect();
            let end = json.find('}').unwrap();
            serde_json::from_str(&json[..end + 1]).unwrap()
        } else {
            Value::Null
        };

        Request {
            method: method.into(),
            route: route.into(),
            params: HashMap::new(),
            headers,
            body,
        }
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} to {:?}", self.method, self.route))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let request = Request::from(String::from("GET /"));
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.route, Route { segments: vec![] });

        let request = Request::from(String::from("DELETE /a/b/c/"));
        assert_eq!(request.method, Method::DELETE);
        assert_eq!(
            request.route,
            Route {
                segments: vec![String::from("a"), String::from("b"), String::from("c")]
            }
        );

        let request = Request::from(String::from(
            r#"POST /auth/register HTTP/1.1
content-type: application/json
accept: */*
host: localhost:3000

{"username": "name", "age": 123}
"#,
        ));
        assert_eq!(request.method, Method::POST);
        assert_eq!(
            request.route,
            Route {
                segments: vec!["auth".to_string(), "register".to_string()]
            }
        );
        assert_eq!(
            request.headers.get("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(request.body["username"], Value::String("name".into()));
        assert_eq!(request.body["age"], Value::Number(123.into()));
    }

    #[test]
    fn debug() {
        let debug = format!("{:?}", Request::from("POST /db".to_string()));
        assert_eq!(debug, "POST to /db");

        let debug = format!("{:?}", Request::from("PATCH /".to_string()));
        assert_eq!(debug, "PATCH to /");
    }
}
