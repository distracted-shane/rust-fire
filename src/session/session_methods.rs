use super::HttpsConnector;
use super::Uuid;
use super::{Client, HeaderMap, Request, Version}; // From crate: Hyper // From crate: Hyper-TLS
use super::{DateTime, Duration, FixedOffset, Utc};

use super::super::json::json_schema::devices;
use super::{collect_body, hdr_string, session_schema, FMCUri, FmcApi}; // Local
use session_schema::{FmcRequest, RequestType, SessionCreds}; // Local

impl<'a> FmcRequest<'a> {
    async fn new() -> FmcRequest<'a> {
        FmcRequest {
            method: None,
            host: None,
            uri: None,
            req: None,
            is_new_auth: false,
            sess_creds: SessionCreds::new().await,
        }
    }

    async fn host(mut self, host: &'a str) -> FmcRequest<'a> {
        self.host = Some(host);
        self
    }

    async fn get(mut self, req_type: FmcApi) -> FmcRequest<'a> {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_creds.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };

        self.method = Some(RequestType::GET);
        self.uri = Some(api_path);
        self
    }
    //pub fn arg<'a>(&'a mut self, arg: String) -> &'a mut Command
    async fn post(mut self, req_type: FmcApi) -> FmcRequest<'a> {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_creds.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };

        self.method = Some(RequestType::POST);
        self.uri = Some(api_path);
        self
    }

    async fn put(mut self, req_type: FmcApi) -> FmcRequest<'a> {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_creds.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };
        self.method = Some(RequestType::PUT);
        self.uri = Some(api_path);
        self
    }

    async fn delete(mut self, req_type: FmcApi) -> FmcRequest<'a> {
        let api_path = match &self.host {
            Some(host) => {
                req_type
                    .path_string(Some(&host), self.sess_creds.dom_uuid)
                    .await
            }
            None => panic!("No host specified!"),
        };
        self.method = Some(RequestType::DELETE);
        self.uri = Some(api_path);
        self
    }

    async fn http_basic(mut self, username: &str, password: &str) -> FmcRequest<'a> {
        self.sess_creds.http_basic_auth(username, password);
        self.is_new_auth = true;
        self
    }

    async fn build(mut self) -> FmcRequest<'a> {
        let uri = self.uri.clone().unwrap();

        let mut req = Request::builder()
            .uri(uri)
            .version(Version::HTTP_11)
            .header("Content-Type", "application/json");

        req = match &self.method {
            Some(RequestType::GET) => req.method("GET"),
            Some(RequestType::POST) => req.method("POST"),
            Some(RequestType::PUT) => req.method("PUT"),
            Some(RequestType::DELETE) => req.method("DELETE"),
            None => panic!("No method defined"), //handle!
        };

        req = match &self.is_new_auth {
            true => {
                let b64_string = self.sess_creds.api_basic_auth.clone().unwrap();
                req.header("Authorization", ["Basic", &b64_string].join(" "))
            }
            false => {
                let token_string = self.sess_creds.xa_token.clone().unwrap();
                req.header("X-Auth-Access-Token", token_string)
            }
        };

        let req = req.body(hyper::Body::empty()).unwrap(); // FIX

        self.req = Some(req);
        self
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
    async fn next(mut self) -> (hyper::body::Body, FmcRequest<'a>) {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let resp = client.request(self.req.unwrap()); //handle error
        let (resp_headers, resp_body) = resp.await.unwrap().into_parts();
        let mut resp_headers = resp_headers.headers;

        if self.is_new_auth {}
        self.sess_creds.record_tokens(&mut resp_headers).await;
        (
            resp_body,
            FmcRequest {
                method: None,
                host: self.host,
                uri: None,
                req: None,
                is_new_auth: false,
                sess_creds: self.sess_creds,
            },
        )
    }

    /// Terminates the builder chain and stores the fiendish creations
    async fn store(self) -> FmcRequest<'a> {
        self
    }
}

impl<'a> SessionCreds {
    async fn new() -> Self {
        SessionCreds {
            api_username: None,
            api_password: None,
            api_basic_auth: None,
            xa_token: None,
            xar_token: None,
            dom_uuid: None,
            token_issue_time: None,
            token_expires: None,
        }
    }

    async fn http_basic_auth(&mut self, username: &str, password: &str) {
        let auth_str = vec![username, ":", password].join("");
        let auth_b64 = base64::encode(auth_str.as_bytes());

        self.api_username = Some(username.to_string());
        self.api_password = Some(password.to_string());
        self.api_basic_auth = Some(auth_b64);
    }

    async fn record_tokens(&mut self, headers: &mut HeaderMap) {
        let xa_token = hdr_string(headers.remove("x-auth-access-token")).await;
        let xar_token = hdr_string(headers.remove("x-auth-refresh-token")).await;

        let dom_uuid_str = hdr_string(headers.remove("domain_uuid")).await;
        let dom_uuid = Uuid::parse_str(&dom_uuid_str).unwrap();

        let time_str = hdr_string(headers.remove("date")).await;
        let time = DateTime::parse_from_rfc2822(&time_str)
            .unwrap()
            .with_timezone(&Utc);

        let thirty_mins = Duration::minutes(30);
        let expiry = time + thirty_mins;

        self.xa_token = Some(xa_token);
        self.xar_token = Some(xar_token);
        self.dom_uuid = Some(dom_uuid);
        self.token_issue_time = Some(time);
        self.token_expires = Some(expiry);
    }

    fn token_age(&self) -> i64 {
        let right_now: DateTime<Utc> = Utc::now();
        let remainder = self.token_expires.unwrap() - right_now;
        Duration::num_seconds(&remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Uuid;
    use super::*; // From crate: Uuid

    #[tokio::test]
    async fn fn_new_request() {
        let (resp, new_req) = FmcRequest::new()
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

        println!("{:#?}", &new_req);

        println!("{:#?}", new_req.sess_creds.token_age());
        // let resp = new_req
        //     .get(FmcApi::Devices)
        //     .await
        //     .xauth_access_token(None)
        //     .await
        //     .build()
        //     .await
        //     .send()
        //     .await;

        // let body = collect_body(resp).await;

        // let json: devices::DeviceRecords = serde_json::from_str(&body).unwrap();
        // println!("{:#?}", json);
    }
}
