

pub enum Method{
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options
}

impl Method {
    pub fn convert_method_into_reqwest_method(method: Method) -> reqwest::Method {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Delete => reqwest::Method::DELETE,
            Method::Options => reqwest::Method::OPTIONS,
        }
    }
}