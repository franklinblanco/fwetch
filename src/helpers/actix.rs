use actix_web::{HttpRequest, web::Json, HttpResponse};
use reqwest::header::HeaderMap;
use serde_json::Value;
use crate::dto::error::Error;

/// Convert an actix request to a reqwest. (Useful for reverse proxy forwarding)
/// Usage:
/// ```
/// use actix_web::{HttpRequest, HttpResponse, web};
/// use serde_json::Value;
/// use fwetch::helpers::actix::{convert_actix_request_to_reqwest, forward_reqwest};
/// 
/// // [post("/some/route")] <-- This wouldn't be a comment in your code
/// async fn route(request: HttpRequest, body: web::Json<Value>) -> HttpResponse {
///     let converted_request = convert_actix_request_to_reqwest(&request, &body.0).unwrap(); // <-- Relevant to this code
///     forward_reqwest(converted_request).await.unwrap()
/// }
/// ```
pub fn convert_actix_request_to_reqwest(inbound_request: &HttpRequest, body: &Value) -> Result<reqwest::RequestBuilder, Error> {
    //Translate actix web headers to reqwest HeaderMap
    let mut headers: HeaderMap = HeaderMap::new();
    for header in inbound_request.headers()
    {headers.append(header.0, header.1.to_owned());}

    let client = reqwest::Client::new();

    //Create the reqwest request
    let url = inbound_request.uri().to_string();
    let request_to_be_sent = client
    .request(inbound_request.method().to_owned(), url)
    .headers(headers)
    .json(&Json(body));

    Ok(request_to_be_sent)
}

/// Send a reqwest and give back an HttpResponse (Useful for reverse proxy forwarding)
/// Feed the result of this to an actix route and done.
/// Usage:
/// ```
/// use actix_web::{HttpRequest, HttpResponse, web};
/// use serde_json::Value;
/// use fwetch::helpers::actix::{convert_actix_request_to_reqwest, forward_reqwest};
/// 
/// // [post("/some/route")] <-- This wouldn't be a comment in your code
/// async fn route(request: HttpRequest, body: web::Json<Value>) -> HttpResponse {
///     let converted_request = convert_actix_request_to_reqwest(&request, &body.0).unwrap(); // <-- Relevant to this code
///     forward_reqwest(converted_request).await.unwrap()
/// }
/// ```
pub async fn forward_reqwest(request: reqwest::RequestBuilder) -> Result<HttpResponse, Error> {
    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => return Err(Error::ClientError(e))
    };
    let mut response_build = &mut HttpResponse::build(actix_web::http::StatusCode::from_u16(response.status().as_u16()).unwrap());
    
    for header in response.headers() {
        response_build = response_build.append_header(header);
    }

    let response_to_return = match response.json::<Value>().await {
        Ok(json) => {
            response_build.json(json)},
        Err(_e) => {response_build.finish()}
    };
    Ok(response_to_return)
}