## rust-fire
### Cisco FMC API Library

This project provides an asynchronous Rust library to simplify interacting the with Cisco FMC API. WIP. 

Uses:
* base64:               for HTTP Basic Auth encoding
* hyper and hyper-tls:  for low-level HTTP and TLS communication
* serde and serde-json: for deserializing JSON replies 
* tokio:                for async, threading, and such
* uuid:                 for uuids, duh


Example:
```
// Define the API URI 
// (HttpBasicAuth. e.g., we're getting a token)
let api_path = FmcApi::HttpBasicAuth
            .path_string("10.17.23.23", None).await;    // Host addr & domain UUID (not needed here, so none)

let res = FmcRequest::post(api_path).await              // Create a new POST req w/ the specified API path
            .http_basic("apiusr", "zxuk-gqpl").await    // Use basic auth to grab the token
            .build().await                              // Build the request
            .send().await;                              // Execute the TLS connection

println!("{:#?}", resp);                                // Print the response
```

Output:
```
Ok(
    Response {
        status: 204,
        version: HTTP/1.1,
        headers: {
            "date": "Tue, 02 Jun 2020 04:55:12 GMT",
            "server": "Apache",
            "strict-transport-security": "max-age=31536000; includeSubDomains",
            "cache-control": "no-store",
            "accept-ranges": "bytes",
            "vary": "Accept-Charset,Accept-Encoding,Accept-Language,Accept",
            "x-auth-access-token": "2760fd56-e313-13ed-a4c0-97d30773d6ef",
            "x-auth-refresh-token": "88483278-01ad-493d-a1e1-73c1a2c1b8d9",
            "user_uuid": "9b2c9d21-a129-14ea-a86b-fd47e15bced",
            "domain_id": "111",
            "domain_uuid": "e456abcd-e1e2-22e3-8891-6d9ed19b525f",
            "global": "e456abcd-e1e2-22e3-8891-6d9ed19b525f",
            "domains": "[{\"name\":\"Global\",\"uuid\":\"e456abcd-e1e2-22e3-8891-6d9ed19b525f\"}]",
            "x-frame-options": "SAMEORIGIN",
            "x-ua-compatible": "IE=edge",
            "x-permitted-cross-domain-policies": "none",
            "x-xss-protection": "1; mode=block",
            "referrer-policy": "same-origin",
            "content-security-policy": "base-uri 'self'",
            "x-content-type-options": "nosniff",
        },
        body: Body(
            Empty,
        ),
    },
)
```