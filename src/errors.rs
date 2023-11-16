use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BotecoError {
    MissingEnvVar(String),
    MissingUrl,
    HttpError(reqwest::Error),
    DeserializerError(reqwest::Error),
    SerializerError(serde_json::Error),
    NotAZoomUrk(String),
    RegexError(regex::Error),
    CloudFlareApiError(String, String),
    ImprovMxApiError(String, String),
}

impl Display for BotecoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotecoError::MissingEnvVar(var) => {
                write!(f, "Missing environment variable: {var}")
            }
            BotecoError::MissingUrl => write!(f, "Missing URL from the command line."),
            BotecoError::HttpError(e) => write!(f, "Error while making request: {e}"),
            BotecoError::DeserializerError(e) => write!(f, "Error deserializing response: {e}"),
            BotecoError::SerializerError(e) => write!(f, "Error serializing payload: {e}"),
            BotecoError::NotAZoomUrk(url) => write!(f, "Not a Zoom URL: {url}"),
            BotecoError::RegexError(e) => write!(f, "Error while parsing regex: {e}"),
            BotecoError::CloudFlareApiError(status, body) => {
                write!(f, "Error {status} CloudFlare Api: {body}")
            }
            BotecoError::ImprovMxApiError(status, body) => {
                write!(f, "Error {status} ImproveMX Api: {body}")
            }
        }
    }
}
