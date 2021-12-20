use phf::phf_map;

#[derive(Clone)]
pub struct Status {
    num: usize,
    msg: String,
}

impl From<usize> for Status {
    fn from(num: usize) -> Self {
        Status {
            num,
            msg: STATUS_CODES
                .entries()
                .find(|(_, v)| v == &&num)
                .unwrap()
                .0
                .to_string(),
        }
    }
}

impl From<StatusCode> for Status {
    fn from(msg: StatusCode) -> Self {
        Status {
            num: *STATUS_CODES
                .entries()
                .find(|(k, _)| k == &&&msg.to_string())
                .unwrap()
                .1,
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.num, self.msg.to_string())
    }
}

// Once again bad naming
pub enum StatusCode {
    Continue,
    SwitchingProtocol,
    Processing,
    EarlyHints,

    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    MultipleChoice,
    MovedPermanently,
    Fonud, // TODO: Found?
    SeeOther,
    NotModified,
    UseProxy,
    Unused,
    TemporaryRedirect,
    PermanentRedirect,

    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Continue => String::from("Continue"),
            StatusCode::SwitchingProtocol => String::from("Switching Protocol"),
            StatusCode::Processing => String::from("Processing"),
            StatusCode::EarlyHints => String::from("Early Hints"),

            StatusCode::OK => String::from("OK"),
            StatusCode::Created => String::from("Created"),
            StatusCode::Accepted => String::from("Accepted"),
            StatusCode::NonAuthoritativeInformation => {
                String::from("Non-Authoritative Information")
            }
            StatusCode::NoContent => String::from("No Content"),
            StatusCode::ResetContent => String::from("Reset Content"),
            StatusCode::PartialContent => String::from("Partial Content"),
            StatusCode::MultiStatus => String::from("Multi-Status"),
            StatusCode::AlreadyReported => String::from("Already Resported"),
            StatusCode::IMUsed => String::from("IM Used"),

            StatusCode::MultipleChoice => String::from("Multiple Choice"),
            StatusCode::MovedPermanently => String::from("Moved Permanently"),
            StatusCode::Fonud => String::from("Fonud"),
            StatusCode::SeeOther => String::from("See Other"),
            StatusCode::NotModified => String::from("Not Modified"),
            StatusCode::UseProxy => String::from("Use Proxy"),
            StatusCode::Unused => String::from("unused"),
            StatusCode::TemporaryRedirect => String::from("Temporary Redirect"),
            StatusCode::PermanentRedirect => String::from("Permanent Redirect"),

            StatusCode::BadRequest => String::from("Bad Request"),
            StatusCode::Unauthorized => String::from("Unauthorized"),
            StatusCode::PaymentRequired => String::from("Payment Required"),
            StatusCode::Forbidden => String::from("Forbidden"),
            StatusCode::NotFound => String::from("Not Found"),
            StatusCode::MethodNotAllowed => String::from("Method Not Allowed"),
            StatusCode::NotAcceptable => String::from("Not Acceptable"),
            StatusCode::ProxyAuthenticationRequired => {
                String::from("Proxy Authentication Required")
            }
            StatusCode::RequestTimeout => String::from("Request Timeout"),
            StatusCode::Conflict => String::from("Conflict"),
            StatusCode::Gone => String::from("Gone"),
            StatusCode::LengthRequired => String::from("Length Required"),
            StatusCode::PreconditionFailed => String::from("Precondition Failed"),
            StatusCode::PayloadTooLarge => String::from("Payload Too Large"),
            StatusCode::URITooLong => String::from("URI Too Long"),
            StatusCode::UnsupportedMediaType => String::from("Unsupported Media Type"),
            StatusCode::RangeNotSatisfiable => String::from("Range Not Satisfiable"),
            StatusCode::ExpectationFailed => String::from("Expectation Failed"),
            StatusCode::ImATeapot => String::from("I'm a teapot"),
            StatusCode::MisdirectedRequest => String::from("Misdirection Request"),
            StatusCode::UnprocessableEntity => String::from("Unprocessalbe Entity"),
            StatusCode::Locked => String::from("Locked"),
            StatusCode::FailedDependency => String::from("Failed Dependency"),
            StatusCode::TooEarly => String::from("Too Early"),
            StatusCode::UpgradeRequired => String::from("Upgrade Required"),
            StatusCode::PreconditionRequired => String::from("Precondition Required"),
            StatusCode::TooManyRequests => String::from("Too Many Requests"),
            StatusCode::RequestHeaderFieldsTooLarge => {
                String::from("Request Header Fields Too Large")
            }
            StatusCode::UnavailableForLegalReasons => String::from("Unavailable For Legal Reasons"),

            StatusCode::InternalServerError => String::from("Internal Server Error"),
            StatusCode::NotImplemented => String::from("Not Implemented"),
            StatusCode::BadGateway => String::from("Bad Gateway"),
            StatusCode::ServiceUnavailable => String::from("Service Unavailable"),
            StatusCode::GatewayTimeout => String::from("Gateway Timeout"),
            StatusCode::HTTPVersionNotSupported => String::from("HTTP Version Not Supported"),
            StatusCode::VariantAlsoNegotiates => String::from("Variant Also Negotiates"),
            StatusCode::InsufficientStorage => String::from("Insufficient Storage"),
            StatusCode::LoopDetected => String::from("Loop Detected"),
            StatusCode::NotExtended => String::from("Not Extended"),
            StatusCode::NetworkAuthenticationRequired => {
                String::from("Network Authentication Required")
            }
        }
    }
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
