* Namespace: was mod/{mod.rs, methods.rs, schema.rs} dumb? prepend with module name?
* API URI-- base paths for major functions are good, but what to do about full path (e.g., the last couple parts of URI)
* API URI-- Is there a point to having fn() -> Vec<String> AND fn() -> String? I should probably use a real URI type, too 
* HTTP basic auth header- how to do the thing
* Timer - for auth tokens
* Tests - current unit tests are nonsense and no integration tests
* State - this crate's job or users' job?
