use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BotecoError {
    MissingEnvVar(String),
    MissingURL,
    HttpError(reqwest::Error),
    DeserializerError(reqwest::Error),
    SerializerError(serde_json::Error),
    NotAZoomURL(String),
    RegexError(regex::Error),
    CloudFlareAPIError(String, String),
}

impl Display for BotecoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotecoError::MissingEnvVar(var) => {
                write!(f, "Missing environment variable: {}", var)
            }
            BotecoError::MissingURL => write!(f, "Missing URL from the command line."),
            BotecoError::HttpError(e) => write!(f, "Error while making request: {}", e),
            BotecoError::DeserializerError(e) => write!(f, "Error deserializing response: {}", e),
            BotecoError::SerializerError(e) => write!(f, "Error serializing payload: {}", e),
            BotecoError::NotAZoomURL(url) => write!(f, "Not a Zoom URL: {}", url),
            BotecoError::RegexError(e) => write!(f, "Error while parsing regex: {}", e),
            BotecoError::CloudFlareAPIError(status, body) => {
                write!(f, "Error {} CloudFlare API: {}", status, body)
            }
        }
    }
}
