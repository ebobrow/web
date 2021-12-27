use std::{collections::HashMap, fmt::Debug};

use regex::Regex;

use crate::Request;

#[derive(Eq, Clone)]
pub struct Route {
    pub(crate) segments: Vec<String>,
}

impl Route {
    pub fn params(&self, req: &Request) -> HashMap<String, String> {
        self.segments
            .iter()
            .zip(&req.route.segments)
            .filter(|(s, _)| s.starts_with(':'))
            .map(|(s, o)| (s[1..].to_owned(), o.clone()))
            .collect()
    }
}

impl<T: ToString> From<T> for Route {
    fn from(route: T) -> Self {
        Route {
            segments: route
                .to_string()
                .split('/')
                .filter(|a| a != &"")
                .map(String::from)
                .collect(),
        }
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
                    Regex::new(a).unwrap().is_match(b)
                }
            })
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("/{}", &self.segments.join("/")))
    }
}

#[cfg(test)]
mod tests {
    use crate::app::Method;

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
            params: Default::default(),
            headers: Default::default(),
            body: Default::default(),
        };
        request.populate_params(&endpoint);

        let mut expected = HashMap::new();
        expected.insert(String::from("hi"), String::from("1"));

        assert_eq!(expected, request.params);
    }
}
