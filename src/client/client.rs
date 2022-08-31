use serde::{Serialize, de::DeserializeOwned};

use crate::dto::{error::Error, method::Method};

/// Takes a complete url, a method enum, a body (can be None), and headers (can be None)
pub async fn fwetch<B: Serialize, R: DeserializeOwned>(
    url: String,
    method: Method,
    body: Option<B>,
    headers: Option<Vec<(String, String)>>,
) -> Result<R, Error> {
    let client = reqwest::Client::new();
    let mut req_incomplete =
        client.request(Method::convert_method_into_reqwest_method(method), url);

    match headers {
        Some(all_headers) => {
            for header in all_headers {
            req_incomplete = req_incomplete.header(&header.0, &header.1);
        }},
        None => {}
    };
    let req_complete = match body {
        Some(b) => req_incomplete.json(&b),
        None => req_incomplete.header("content-length", 0),
    };
    match req_complete.send().await {
        Ok(res) => {
            // Request sent correctly
            let response_text = match res.text().await {
                Ok(text) => text,
                Err(e) => return Err(Error::SerdeError(e.to_string()))
            };
            match serde_json::from_str(&response_text) {
                Ok(resp_dto) => Ok(resp_dto), //  Return correctly deserialized obj
                Err(e) => Err(Error::SerdeError(format!("String that was attempted to deserialize: {} \n\n\nError: {}", response_text, e.to_string()))),
            }
        }
        Err(e) => {
            //  Request couldn't be sent
            Err(Error::ClientError(e))
        }
    }
}