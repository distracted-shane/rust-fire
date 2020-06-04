use super::Uuid;
use super::{DateTime, Utc};
use super::{FMCUri, Request}; // Local

#[derive(Debug)]
pub(super) enum RequestType {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub(super) struct FmcRequest<'a> {
    pub(super) method: Option<RequestType>,
    pub(super) host: Option<&'a str>,
    pub(super) uri: Option<FMCUri>,
    pub(super) req: Option<Request<hyper::Body>>,
    pub(super) is_new_auth: bool,
    pub(super) sess_creds: SessionCreds,
}

#[derive(Debug)]
pub(super) struct SessionCreds {
    pub(super) api_username: Option<String>,
    pub(super) api_password: Option<String>,
    pub(super) api_basic_auth: Option<String>,
    pub(super) xa_token: Option<String>,
    pub(super) xar_token: Option<String>,
    pub(super) dom_uuid: Option<Uuid>,
    pub(super) token_issue_time: Option<DateTime<Utc>>,
    pub(super) token_expires: Option<DateTime<Utc>>,
}
