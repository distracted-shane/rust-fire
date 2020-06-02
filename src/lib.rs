pub mod json;
pub mod session;
pub mod uri;

use base64::encode;
use uuid::Uuid;
use hyper::{Client, Request, Version, http::HeaderValue};
use hyper_tls::HttpsConnector;

type FMCUri = String;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
