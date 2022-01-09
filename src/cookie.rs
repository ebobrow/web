use std::time::SystemTime;

use httpdate::fmt_http_date;
use macros::Builder;

#[derive(Debug, Clone)]
pub enum SameSite {
    /// means that the browser sends the cookie only for same-site requests, that is, requests
    /// originating from the same site that set the cookie. If a request originates from a URL
    /// different from the current one, no cookies with the SameSite=Strict attribute are sent.
    Strict,

    /// means that the cookie is not sent on cross-site requests, such as on requests to load
    /// images or frames, but is sent when a user is navigating to the origin site from an external
    /// site (for example, when following a link). This is the default behavior if the SameSite
    /// attribute is not specified.
    Lax,

    /// means that the browser sends the cookie with both cross-site and same-site requests. The
    /// Secure attribute must also be set when setting this value, like so SameSite=None; Secure
    None,
}

impl ToString for SameSite {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Clone, Builder)]
pub struct Cookie {
    name: String,
    value: String,
    expires: Option<SystemTime>, // can use SystemTime::from(chrono::DateTime)
    max_age: Option<i32>,
    domain: Option<String>,
    path: Option<String>,
    secure: bool,
    http_only: bool,
    same_site: SameSite,
}

impl Cookie {
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: SameSite::Lax,
        }
    }

    pub(crate) fn as_header(&self) -> String {
        let mut header = format!("{}={}", self.name, self.value);
        if let Some(expires) = &self.expires {
            header.push_str("; Expires=");
            header.push_str(&fmt_http_date(*expires));
        }
        if let Some(max_age) = &self.max_age {
            header.push_str("; Max-Age=");
            header.push_str(&max_age.to_string());
        }
        if let Some(domain) = &self.domain {
            header.push_str("; Domain=");
            header.push_str(domain);
        }
        if let Some(path) = &self.path {
            header.push_str("; Path=");
            header.push_str(path);
        }
        if self.secure {
            header.push_str("; Secure");
        }
        if self.http_only {
            header.push_str("; HTTPOnly");
        }
        header.push_str("; SameSite=");
        header.push_str(&self.same_site.to_string());
        header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        assert_eq!(
            Cookie::new("name", "value").as_header(),
            "name=value; SameSite=Lax"
        );

        let now = SystemTime::now();
        assert_eq!(
            Cookie::new("a", "b")
                .expires(now)
                .path("/")
                .secure(true)
                .same_site(SameSite::None)
                .as_header(),
            format!(
                "a=b; Expires={}; Path=/; Secure; SameSite=None",
                fmt_http_date(now)
            )
        );
    }
}
