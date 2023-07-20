pub use method::Method; // for exposing http::Method at main.rs
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError;
pub use request::Request; // for use http::Request at main.rs
pub mod method;
pub mod query_string;
pub mod request;
