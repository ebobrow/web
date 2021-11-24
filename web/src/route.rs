use regex::Regex;

#[derive(Debug, Eq)]
pub struct Route {
    segments: Vec<String>,
    // TODO: parameters sent to Request struct
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
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.segments.len() == other.segments.len()
            && !self
                .segments
                .iter()
                .zip(&other.segments)
                .find(|(a, b)| {
                    if a.chars().all(char::is_alphanumeric) {
                        a != b
                    } else {
                        !Regex::new(&a).unwrap().is_match(b)
                    }
                })
                .is_some()
    }
}

#[cfg(test)]
mod tests {
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
}
