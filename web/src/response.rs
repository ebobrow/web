use phf::phf_map;

pub struct Response {
    content: String,
    status_code: usize,
}

static STATUS_CODES: phf::Map<&'static str, usize> = phf_map! {
    "Continue" => 100,
    "Switching Protocol" => 101,
    "Processing" => 102,
    "Early Hints" => 103,

    "OK" => 200,
    "Created" => 201,
    "Accepted" => 202,
    "Non-Authoritative Information" => 203,
    "No Content" => 204,
    "Reset Content" => 205,
    "Partial Content" => 206,
    "Multi-Status" => 207,
    "Already Reported" => 208,
    "IM Used" => 226,

    "Multiple Choice" => 300,
    "Moved Permanently" => 301,
    "Fonud" => 302,
    "See Other" => 303,
    "Not Modified" => 304,
    "Use Proxy" => 305,
    "unused" => 306,
    "Temporary Redirect" => 307,
    "Permanent Redirect" => 308,

    "Bad Request" => 400,
    "Unauthorized" => 401,
    "Payment Required" => 402,
    "Forbidden" => 403,
    "Not Found" => 404,
    "Method Not Allowed" => 405,
    "Not Acceptable" => 406,
    "Proxy Authentication Required" => 407,
    "Request Timeout" => 408,
    "Conflict" => 409,
    "Gone" => 410,
    "Length Required" => 411,
    "Precondition Failed" => 412,
    "Payload Too Large" => 413,
    "URI Too Long" => 414,
    "Unsupported Media Type" => 415,
    "Range Not Satisfiable" => 416,
    "Expectation Failed" => 417,
    "I'm a teapot" => 418,
    "Misdirected Request" => 421,
    "Unprocessable Entity" => 422,
    "Locked" => 423,
    "Failed Dependency" => 424,
    "Too Early" => 425,
    "Upgrade Required" => 426,
    "Precondition Required" => 428,
    "Too Many Requests" => 429,
    "Request Header Fields Too Large" => 431,
    "Unavailable For Legal Reasons" => 451,

    "Internal Server Error" => 500,
    "Not Implemented" => 501,
    "Bad Gateway" => 502,
    "Service Unavailable" => 503,
    "Gateway Timeout" => 504,
    "HTTP Version Not Supported" => 505,
    "Variant Also Negotiates" => 506,
    "Insufficient Storage" => 507,
    "Loop Detected" => 508,
    "Not Extended" => 510,
    "Network Authentication Required" => 511
};

impl Response {
    pub fn new(content: String, status_code: usize) -> Self {
        Self {
            content,
            status_code,
        }
    }

    pub fn format_for_response(&self) -> String {
        let status_line = format!(
            "HTTP/1.1 {} {}",
            self.status_code,
            STATUS_CODES
                .entries()
                .find(|(_, v)| { v == &&self.status_code })
                .unwrap()
                .0
        );
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            self.content.len(),
            self.content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_response() {
        let content = String::from("hello");
        let status_code = 200;

        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";

        assert_eq!(
            Response::new(content, status_code).format_for_response(),
            expected
        );
    }
}