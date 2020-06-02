use super::{FMCUri, Request}; // Local

#[derive(Debug)]
pub(super) enum RequestType {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub(super) enum AuthCreds {
    HTTPBasic(String),
    XAuthAccessToken(String),
    XAuthRefreshToken(String),
}

#[derive(Debug)]
pub(super) struct FmcRequest {
    pub(super) method: RequestType,
    pub(super) username: Option<String>,
    pub(super) secret: Option<AuthCreds>,
    pub(super) uri: FMCUri,
    pub(super) req: Option<Request<hyper::Body>>,
}
