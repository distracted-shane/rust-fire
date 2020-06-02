use super::HeaderValue; // From hyper
use super::Uuid;
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
    pub(super) method: Option<RequestType>,
    pub(super) username: Option<String>,
    pub(super) secret: Option<AuthCreds>,
    pub(super) uri: Option<FMCUri>,
    pub(super) req: Option<Request<hyper::Body>>,
    pub(super) is_auth: bool,
    pub(super) sess_ids: SessionIDs,
}

#[derive(Debug)]
pub(super) struct SessionIDs {
    pub(super) xa_token: Option<String>,
    pub(super) xar_token: Option<String>,
    pub(super) dom_uuid: Option<Uuid>,
    pub(super) time: Option<String>,
}
