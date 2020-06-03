use super::HttpsConnector;
use super::Uuid;
use super::{Client, Request, Version}; // From crate: Hyper // From crate: Hyper-TLS

use super::super::json::json_schema::devices;
use super::{collect_body, session_schema, FMCUri, FmcApi}; // Local
use session_schema::{AuthCreds, FmcRequest, RequestType, SessionIDs}; // Local

impl FmcRequest {
    async fn new() -> Self {
        FmcRequest {
            method: None,
            username: None,
            secret: None,
            host: None,
            uri: None,
            req: None,
            is_auth: false,
            sess_ids: SessionIDs::new(),
        }
    }

    async fn host(self, host: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: self.username,
            secret: self.secret,
            host: Some(String::from(host)),
            uri: self.uri,
            req: None,
            is_auth: false,
            sess_ids: self.sess_ids,
        }
    }

    async fn get(self, req_type: FmcApi) -> Self {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_ids.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };
        FmcRequest {
            method: Some(RequestType::GET),
            username: self.username,
            secret: self.secret,
            host: self.host, // Add check for empty field
            uri: Some(api_path),
            req: None,
            is_auth: false,
            sess_ids: self.sess_ids,
        }
    }
    async fn post(self, req_type: FmcApi) -> Self {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_ids.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };
        FmcRequest {
            method: Some(RequestType::POST),
            username: self.username,
            secret: self.secret,
            host: self.host, // Add check for empty field
            uri: Some(api_path),
            req: None,
            is_auth: false,
            sess_ids: self.sess_ids,
        }
    }

    async fn put(self, req_type: FmcApi) -> Self {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_ids.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };
        FmcRequest {
            method: Some(RequestType::PUT),
            username: self.username,
            secret: self.secret,
            host: self.host, // Add check for empty field
            uri: Some(api_path),
            req: None,
            is_auth: false,
            sess_ids: self.sess_ids,
        }
    }
    async fn delete(self, req_type: FmcApi) -> Self {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_ids.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };
        FmcRequest {
            method: Some(RequestType::DELETE),
            username: self.username,
            secret: self.secret,
            host: self.host, // Add check for empty field
            uri: Some(api_path),
            req: None,
            is_auth: false,
            sess_ids: self.sess_ids,
        }
    }

    async fn http_basic(self, username: &str, password: &str) -> Self {
        let auth_string =
            vec![username.to_string(), ":".to_string(), password.to_string()].join("");
        let auth_b64 = base64::encode(auth_string.as_bytes());

        FmcRequest {
            method: self.method,
            username: Some(username.to_string()),
            secret: Some(AuthCreds::HTTPBasic(auth_b64)),
            host: self.host,
            uri: self.uri,
            req: None,
            is_auth: true,
            sess_ids: self.sess_ids,
        }
    }

    async fn xauth_access_token(self, access_token: Option<String>) -> Self {
        match access_token {
            Some(token) => FmcRequest {
                method: self.method,
                username: self.username,
                secret: Some(AuthCreds::XAuthAccessToken(token)),
                host: self.host,
                uri: self.uri,
                req: None,
                is_auth: self.is_auth,
                sess_ids: self.sess_ids,
            },
            None => {
                // handle missing
                let stoken = self.sess_ids.xa_token.clone();
                FmcRequest {
                    method: self.method,
                    username: self.username,
                    secret: self.secret,
                    host: self.host,
                    uri: self.uri,
                    req: None,
                    is_auth: self.is_auth,
                    sess_ids: self.sess_ids,
                }
            }
        }
    }

    async fn xauth_refresh_token(self, refresh_token: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: self.username,
            secret: Some(AuthCreds::XAuthRefreshToken(refresh_token.to_string())),
            host: self.host,
            uri: self.uri,
            req: None,
            is_auth: self.is_auth,
            sess_ids: self.sess_ids,
        }
    }

    async fn build(self) -> Self {
        let uri = self.uri.unwrap();

        let mut req = Request::builder()
            .uri(&uri)
            .version(Version::HTTP_11)
            .header("Content-Type", "application/json");

        req = match &self.method {
            Some(RequestType::GET) => req.method("GET"),
            Some(RequestType::POST) => req.method("POST"),
            Some(RequestType::PUT) => req.method("PUT"),
            Some(RequestType::DELETE) => req.method("DELETE"),
            None => panic!("No method defined"), //handle!
        };

        req = match &self.secret {
            Some(AuthCreds::HTTPBasic(secret)) => {
                req.header("Authorization", ["Basic", secret].join(" "))
            }
            Some(AuthCreds::XAuthAccessToken(secret)) => req.header("X-Auth-Access-Token", secret),
            Some(AuthCreds::XAuthRefreshToken(secret)) => req.header("X-Auth-Access-Token", secret),
            None => panic!("Missing auth credentials"), // FIX/HANDLE
        };

        let req = req.body(hyper::Body::empty()).unwrap(); // FIX

        FmcRequest {
            method: self.method,
            username: self.username,
            secret: self.secret,
            host: self.host,
            uri: Some(uri),
            req: Some(req),
            is_auth: self.is_auth,
            sess_ids: self.sess_ids,
        }
    }

    /// Terminates the builder chain by sending the
    /// request, consuming the struct in the process.
    /// Useful for one-off requests when you have the token.
    async fn send(self) -> hyper::client::ResponseFuture {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        client.request(self.req.unwrap()) //handle
    }

    /// Terminates the builder chain by sending the request, yielding
    /// a response and new struct with authentication credentials intact.
    /// Deconstruct with let (resp, req) = FmcRequest::...
    /// Useful series of requests while retaining authentication credentials.
    async fn next(self) -> (hyper::body::Body, Self) {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let resp = client.request(self.req.unwrap()); //handle error
        let (mut resp_headers, resp_body) = resp.await.unwrap().into_parts();

        let sess_ids = match self.is_auth {
            true => SessionIDs {
                xa_token: Some(
                    resp_headers
                        .headers
                        .remove("x-auth-access-token")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                xar_token: Some(
                    resp_headers
                        .headers
                        .remove("x-auth-refresh-token")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                dom_uuid: Some(
                    Uuid::parse_str(
                        resp_headers
                            .headers
                            .remove("domain_uuid")
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    )
                    .unwrap(),
                ),
                time: Some(
                    resp_headers
                        .headers
                        .remove("date")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
            },
            false => self.sess_ids,
        };

        (
            resp_body,
            FmcRequest {
                method: None,
                username: self.username,
                secret: Some(AuthCreds::XAuthAccessToken(
                    sess_ids.xa_token.clone().unwrap(),
                )),
                host: self.host,
                uri: None,
                req: None,
                is_auth: false,
                sess_ids,
            },
        )
    }
}

impl SessionIDs {
    fn new() -> Self {
        SessionIDs {
            xa_token: None,
            xar_token: None,
            dom_uuid: None,
            time: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Uuid;
    use super::*; // From crate: Uuid

    #[tokio::test]
    async fn fn_new_request() {
        let (_resp, new_req) = FmcRequest::new()
            .await
            .host("10.17.11.151")
            .await
            .post(FmcApi::HttpBasicAuth)
            .await
            .http_basic("apiuser", "vZZ90-8D1z")
            .await
            .build()
            .await
            .next()
            .await;

        println!("{:#?}", new_req);

        let resp = new_req
            .get(FmcApi::Devices)
            .await
            .xauth_access_token(None)
            .await
            .build()
            .await
            .send()
            .await;

        let body = collect_body(resp).await;

        let json: devices::DeviceRecords = serde_json::from_str(&body).unwrap();
        println!("{:#?}", json);
    }
}
