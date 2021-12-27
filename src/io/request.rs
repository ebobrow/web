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

impl TryFrom<&[u8; 1024]> for Request {
    type Error = &'static str;

    fn try_from(buffer: &[u8; 1024]) -> Result<Self, Self::Error> {
        Self::try_from(String::from_utf8(buffer.to_vec()).unwrap())
    }
}

impl TryFrom<String> for Request {
    type Error = &'static str;

    fn try_from(req: String) -> Result<Self, Self::Error> {
        let mut lines = req.lines();
        let mut header = lines.next().ok_or("empty request")?.split(' ');
        let method = header.next().ok_or("invalid start-line")?;
        let route = header.next().ok_or("invalid start-line")?;

        let mut headers = HashMap::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break; // End of headers
            }

            let (key, value) = line.split_once(':').ok_or("invalid header")?;
            let mut value = value.to_lowercase();
            value.retain(|c| !c.is_whitespace());
            headers.insert(key.to_lowercase(), value);
        }

        let body = if let Some("application/json") = headers.get("content-type").map(|s| &s[..]) {
            let json: String = lines.collect();
            let end = json.find('}').ok_or("invalid body")?;
            match serde_json::from_str(&json[..end + 1]) {
                Ok(body) => body,
                Err(_) => return Err("invalid body"),
            }
        } else {
            Value::Null
        };

        Ok(Request {
            method: method.try_into()?,
            route: route.into(),
            params: HashMap::new(),
            headers,
            body,
        })
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
        let request = Request::try_from(String::from("GET /")).unwrap();
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.route, Route { segments: vec![] });

        let request = Request::try_from(String::from("DELETE /a/b/c/")).unwrap();
        assert_eq!(request.method, Method::DELETE);
        assert_eq!(
            request.route,
            Route {
                segments: vec![String::from("a"), String::from("b"), String::from("c")]
            }
        );

        let request = Request::try_from(String::from(
            r#"POST /auth/register HTTP/1.1
content-type: application/json
accept: */*
host: localhost:3000

{"username": "name", "age": 123}
"#,
        ))
        .unwrap();
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
    fn invalid_create() {
        match Request::try_from(String::new()) {
            Ok(_) => panic!("bad request didn't error"),
            Err(e) => assert_eq!(e, "empty request"),
        };

        match Request::try_from(String::from("OneLongWordWithNoSpaces")) {
            Ok(_) => panic!("bad request didn't error"),
            Err(e) => assert_eq!(e, "invalid start-line"),
        };

        match Request::try_from(String::from("GWT /")) {
            Ok(_) => panic!("bad request didn't error"),
            Err(e) => assert_eq!(e, "invalid method"),
        };

        match Request::try_from(String::from("GET / HTTP/1.1\nbad header")) {
            Ok(_) => panic!("bad request didn't error"),
            Err(e) => assert_eq!(e, "invalid header"),
        };

        match Request::try_from(String::from(
            "GET / HTTP/1.1\ncontent-type:application/json\n\n[]",
        )) {
            Ok(_) => panic!("bad request didn't error"),
            Err(e) => assert_eq!(e, "invalid body"),
        };

        match Request::try_from(String::from(
            r#"GET / HTTP/1.1
content-type:application/json

{
    "key
}"#,
        )) {
            Ok(_) => panic!("bad request didn't error"),
            Err(e) => assert_eq!(e, "invalid body"),
        };
    }

    #[test]
    fn debug() {
        let debug = format!("{:?}", Request::try_from("POST /db".to_string()).unwrap());
        assert_eq!(debug, "POST to /db");

        let debug = format!("{:?}", Request::try_from("PATCH /".to_string()).unwrap());
        assert_eq!(debug, "PATCH to /");
    }
}
