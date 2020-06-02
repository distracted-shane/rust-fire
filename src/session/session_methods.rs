use super::HttpsConnector;
use super::{Client, Request, Version}; // From crate: Hyper // From crate: Hyper-TLS

use super::{session_schema, FMCUri, FmcApi}; // Local
use session_schema::{AuthCreds, FmcRequest, RequestType}; // Local

impl FmcRequest {
    async fn get(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::GET,
            username: None,
            secret: None,
            uri,
            req: None,
        }
    }
    async fn post(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::POST,
            username: None,
            secret: None,
            uri,
            req: None,
        }
    }
    async fn put(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::PUT,
            username: None,
            secret: None,
            uri,
            req: None,
        }
    }
    async fn delete(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::DELETE,
            username: None,
            secret: None,
            uri,
            req: None,
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
            uri: self.uri,
            req: None,
        }
    }

    async fn xauth_access_token(self, access_token: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: self.username,
            secret: Some(AuthCreds::XAuthAccessToken(access_token.to_string())),
            uri: self.uri,
            req: None,
        }
    }

    async fn xauth_refresh_token(self, refresh_token: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: self.username,
            secret: Some(AuthCreds::XAuthRefreshToken(refresh_token.to_string())),
            uri: self.uri,
            req: None,
        }
    }

    async fn build(self) -> Self {
        let mut req = Request::builder()
            .uri(&self.uri)
            .version(Version::HTTP_11)
            .header("Content-Type", "application/json");

        req = match &self.method {
            RequestType::GET => req.method("GET"),
            RequestType::POST => req.method("POST"),
            RequestType::PUT => req.method("PUT"),
            RequestType::DELETE => req.method("DELETE"),
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
            uri: self.uri,
            req: Some(req),
        }
    }

    async fn send(self) -> hyper::client::ResponseFuture {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        client.request(self.req.unwrap()) //handle
    }
}

#[cfg(test)]
mod tests {
    use super::super::Uuid;
    use super::*; // From crate: Uuid

    #[tokio::test]
    async fn fn_new_request() {
        let api_path = FmcApi::HttpBasicAuth
            .path_string("10.17.11.151", None)
            .await;
        let req = FmcRequest::post(api_path)
            .await
            .http_basic("apiuser", "vZZ90-8D1z")
            .await
            .build()
            .await
            .send()
            .await;

        println!("{:#?}", req.await);
    }
}
