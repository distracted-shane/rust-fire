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
// Create a new request (and keep it around using next())
async fn fn_new_request() {
    let (resp, next_req) = FmcRequest::new()          .await  // (response to this request, struct for next request) 
        .post("10.14.31.111", FmcApi::HttpBasicAuth)  .await  // (POST request to this IP, correct API URI for HTTP auth)
        .http_basic("apiusr", "xefZ80-8Dfe1z")        .await  // (HTTP Basic auth credentials)
        .build()                                      .await  // Build the request
        .next()                                       .await; // Send request; Save response & struct as (resp, next_req)

    let resp = next_req.get("10.14.31.111", FmcApi::Devices)    .await  // reuse request struct
            .xauth_access_token(None)                           .await  // We auth'd & token is in struct, so None
            .build()                                            .await  // Build
            .send()                                             .await; // Send. Unlike next, we can't reuse w/ send()

        let body = collect_body(resp).await;                            // Helper fn to collect the raw data into String

        let json: devices::DeviceRecords = serde_json::from_str(&body).unwrap();     // Parse JSON to matching struct
        println!("{:#?}", json);                                                     // Print the response
```

Output:
```
DeviceRecords {
    links: Links {
        _self: "https://10.14.31.111/api/fmc_config/v1/domain/e456abcd-e1e2-22e3-8891-6d9ed19b525f/devices/devicerecords?offset=0&limit=2",
        parent: None,
    },
    items: [
        DeviceItem {
            id: "88483278-01ad-493d-a1e1-73c1a2c1b8d9",
            _type: "Device",
            links: Links {
                _self: "https://10.17.11.151/api/fmc_config/v1/domain/e456abcd-e1e2-22e3-8891-6d9ed19b525f/devices/devicerecords/88483278-01ad-493d-a1e1-73c1a2c1b8d9",
                parent: None,
            },
            name: "CatNumberOne",
            host_name: None,
            ftd_mode: None,
        },
        DeviceItem {
            id: "2760fd56-e313-13ed-a4c0-97d30773d6ef",
            _type: "Device",
            links: Links {
                _self: "https://10.17.11.151/api/fmc_config/v1/domain/e276abec-e0f2-11e3-8169-6d9ed49b625f/devices/devicerecords/2760fd56-e313-13ed-a4c0-97d30773d6ef",
                parent: None,
            },
            name: "CatNumberTwo",
            host_name: None,
            ftd_mode: None,
        },
    ],
    paging: Paging {
        offset: 0,
        limit: 2,
        count: 2,
        pages: 1,
    },
}
)
```