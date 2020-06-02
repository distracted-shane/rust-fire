use super::*;
use super::{uri::uri_schema::FmcApi, FMCUri}; // Local

mod session_methods;
pub mod session_schema;

async fn collect_body(body: hyper::client::ResponseFuture) -> String {
    let bytes = hyper::body::to_bytes(body.await.unwrap().body_mut())
        .await
        .unwrap(); //ok this works, fix later
    let text = std::str::from_utf8(&bytes).unwrap(); //handle

    String::from(text)
}
