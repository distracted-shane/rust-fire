## rust-fire
### Cisco FMC API Library

This project provides an asynchronous Rust library to simplify interacting the with Cisco FMC API. WIP. 

Uses:
* base64:               for HTTP Basic Auth encoding
* hyper and hyper-tls:  for low-level HTTP and TLS communication
* serde and serde-json: for deserializing JSON replies 
* tokio:                for async, threading, and such
* uuid:                 for uuids, duh


Example: Authenticating to FMC and polling devices. 
```
// Create a new request (and keep it around using next())
let (resp, next_req) = FmcRequest::new().await     // (resp to this request, struct for the next) 
  .host("10.0.1.121").await                        // Specify URL
  .post(FmcApi::HttpBasicAuth).await               // (POST req to the API URI for HTTP auth)
  .http_basic("apiusr", "xefZ80-8Dfe1z").await     // (HTTP Basic auth credentials)
  .build().await                                   // Build the request
  .next().await;                                   // Send request; Save response & rew struct

let resp = next_req.get(FmcApi::Devices).await               // reuse struct for GET request
  .xauth_access_token(None).await                            // Auth'd (token now in struct) so None
  .build().await                                             // Build
  .send().await;                                             // Send(). Unlike next(), we can't reuse

let body = collect_body(resp).await;               // Helper fn to collect the raw data into String

let json: devices::DeviceRecords = serde_json::from_str(&body).unwrap();  // Parse JSON to matching struct
println!("{:#?}", json);                                                  // Print the response
```

Output:
```
DeviceRecords {
    links: Links {
        _self: "https://10.1.1.1/api/fmc_config/v1/domain/e456abcd-e1e2-22e3-8891-6d9ed19b525f... (snip),
        parent: None,
    },
    items: [
        DeviceItem {
            id: "88483278-01ad-493d-a1e1-73c1a2c1b8d9",
            _type: "Device",
            links: Links {
                _self: "https://10.1.1.1//api/fmc_config/v1/domain/{snip UUID}/devices/devicerecords/{snip UUID}",
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
                _self: "https://10.1.1.1//api/fmc_config/v1/domain/{snip UUID}/devices/devicerecords/{snip UUID}",
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