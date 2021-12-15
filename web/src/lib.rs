pub mod app;
pub mod request;
pub mod response;
mod route;

pub use app::App;
pub use app::Method;
pub use request::Request;
pub use response::Response;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
