# Fwetch
### [This package](https://crates.io/crates/fwetch)
*Uses [Reqwest](https://crates.io/crates/reqwest)*
**by [Franklin Blanco](https://github.com/franklinblanco)**

Brought to you from the javascript world, this is a fetch() implementation. But as the name suggests, it utilizes reqwest.

## How to use this crate?

Useful features:

**Fwetch**
```rust
// Say you have a struct
#[derive(Serialize, Deserialize)] // <-- This is from serde (serde.rs)
struct Person{
	pub name: String
}
// If this panics you will get more context regarding what happened than you usually would with reqwest.
let person: Person = fwetch("testurl.com", Method::Get, None, None).await.unwrap();
// You can now use this struct freely.
```

Interopability with Actix-web (Optional feature called "actix").
Enable by adding to your *Cargo.toml*
```toml
[dependencies]
fwetch = { version = "0", features = ["all"] }
```

This allows you to create a proxy with two lines of code! Convert an actix request into a reqwest with  `convert_actix_request_to_reqwest()` and send it with `forward_request()`
```rust
use actix_web::{HttpRequest, HttpResponse, web};

use serde_json::Value;

use fwetch::helpers::actix::{convert_actix_request_to_reqwest, forward_reqwest};

[post("/some/route")]
async fn route(request: HttpRequest, body: web::Json<Value>) -> HttpResponse {
	let converted_request = convert_actix_request_to_reqwest(&request, &body.0).unwrap();
	forward_reqwest(converted_request).await.unwrap()
}
```



### Known issues with Reqwest
If you get a compilation error that's really long and confusing, it's probably because you don't have openssl (or at least the version reqwest needs).
Solution: (Cargo.toml)
```toml
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```
