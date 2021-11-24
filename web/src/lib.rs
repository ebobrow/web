pub mod app;
mod endpoint;
pub mod request;
pub mod response;
mod route;

pub use crate::app::App;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
