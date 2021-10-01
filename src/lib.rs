// Types of arguments for auth handlers
pub mod response;
pub mod request;
// not all conumers need a client
#[cfg(feature = "client")]
pub mod client;
