use super::super::{uri::schema::FmcApi, FMCUri};
use super::*;
use schema::{AuthCreds, FmcRequest, RequestType};

impl FmcRequest {
    async fn get(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::GET,
            username: None,
            secret: None,
            uri,
        }
    }
    async fn post(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::POST,
            username: None,
            secret: None,
            uri,
        }
    }
    async fn put(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::PUT,
            username: None,
            secret: None,
            uri,
        }
    }
    async fn delete(uri: FMCUri) -> Self {
        FmcRequest {
            method: RequestType::DELETE,
            username: None,
            secret: None,
            uri,
        }
    }

    async fn http_basic(self, username: &str, password: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: Some(username.to_string()),
            secret: Some(AuthCreds::HTTPBasic(password.to_string())),
            uri: self.uri,
        }
    }

    async fn xauth_access_token(self, access_token: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: self.username,
            secret: Some(AuthCreds::XAuthAccessToken(access_token.to_string())),
            uri: self.uri,
        }
    }

    async fn xauth_refresh_token(self, refresh_token: &str) -> Self {
        FmcRequest {
            method: self.method,
            username: self.username,
            secret: Some(AuthCreds::XAuthRefreshToken(refresh_token.to_string())),
            uri: self.uri,
        }
    }

    async fn build(self) -> Request<()> {
        let mut req = Request::builder()
            .uri(self.uri)
            .version(Version::HTTP_11)
            .header("Content-Type", "application/json");

        req = match self.method {
            GET => req.method("GET"),
            POST => req.method("POST"),
            PUT => req.method("PUT"),
            DELETE => req.method("DELETE"),
        };

        req = match self.secret {
            Some(AuthCreds::HTTPBasic(secret)) => req.header("HTTP-Basic-Auth", secret),
            Some(AuthCreds::XAuthAccessToken(secret)) => req.header("X-Auth-Access-Token", secret),
            Some(AuthCreds::XAuthRefreshToken(secret)) => req.header("X-Auth-Access-Token", secret),
            None => panic!("Missing auth credentials"), // FIX/HANDLE
        };

        req.body(()).unwrap() // FIX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fn_new_request() {
        let dom_uuid = uuid::Uuid::parse_str("f3b4958c-52a1-11e7-802a-010203040506").unwrap();
        let api_path = FmcApi::Deployment.path_string("10.0.0.1", dom_uuid).await;
        let req = FmcRequest::get(api_path)
            .await
            .http_basic("shane", "hulk")
            .await
            .build();
        println!("{:#?}", req.await);

        let dom_uuid = uuid::Uuid::parse_str("f3b4958c-52a1-11e7-802a-010203040506").unwrap();
        let api_path = FmcApi::Object.path_string("10.0.0.1", dom_uuid).await;
        let req = FmcRequest::get(api_path)
            .await
            .xauth_access_token("shanehulk")
            .await
            .build();
        println!("{:#?}", req.await);
    }
}
