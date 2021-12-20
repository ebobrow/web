pub mod app;
pub mod request;
pub mod response;
mod route;
pub mod status; // TODO: Put status and response in subfolder together?

pub use app::App;
pub use app::Method;
pub use request::Request;
pub use response::Response;
pub use status::StatusCode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
