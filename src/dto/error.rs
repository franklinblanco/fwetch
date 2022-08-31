#[derive(Debug)]
pub enum Error{
    SerdeError(String),
    ClientError(reqwest::Error)
}