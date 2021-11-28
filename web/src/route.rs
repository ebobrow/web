use std::collections::HashMap;

use regex::Regex;

use crate::Request;

#[derive(Debug, Eq, Clone)]
pub struct Route {
    segments: Vec<String>,
}

impl Route {
    pub fn from(route: impl ToString) -> Self {
        Route {
            segments: route
                .to_string()
                .split('/')
                .filter(|a| a != &"")
                .map(String::from)
                .collect(),
        }
    }

    pub fn params(&self, req: &mut Request) {
        let mut params = HashMap::new();
        self.segments
            .iter()
            .zip(&req.route.segments)
            .filter(|(s, _)| s.starts_with(':'))
            .for_each(|(s, r)| {
                params.insert(s[1..].to_owned(), r.clone());
            });
        req.params = params;
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.segments.len() == other.segments.len()
            && self.segments.iter().zip(&other.segments).all(|(a, b)| {
                if a.chars().all(char::is_alphanumeric) {
                    a == b
                } else if a.starts_with(':') {
                    true // match on all parameters
                } else {
                    Regex::new(&a).unwrap().is_match(b)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoint::Method;

    use super::*;

    #[test]
    fn matches_normal() {
        let a = Route::from("/");
        let c = Route::from("/hi/");
        let d = Route::from("/hi");

        assert_eq!(a, a);
        assert_ne!(a, c);
        assert_eq!(c, d);
    }

    #[test]
    fn matches_regex() {
        let wild = Route::from("/h+");
        let hi = Route::from("/hi/");
        let i = Route::from("/i");

        assert_eq!(wild, hi);
        assert_ne!(wild, i);
    }

    #[test]
    fn params() {
        let endpoint = Route::from("/:hi");
        let mut request = Request {
            method: Method::GET,
            route: Route::from("/1"),
            params: HashMap::new(),
        };
        endpoint.params(&mut request);

        let mut expected = HashMap::new();
        expected.insert(String::from("hi"), String::from("1"));

        assert_eq!(expected, request.params);
    }
}
