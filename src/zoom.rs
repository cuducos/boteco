use regex::Regex;

use crate::errors::BotecoError;

pub struct ZoomURL {
    pub full: String,
    pub short: String,
}

impl ZoomURL {
    pub fn new(url: String) -> Result<Self, BotecoError> {
        let number = Regex::new(r"/j/(?P<number>\d+)\?\w+")
            .map_err(BotecoError::RegexError)?
            .captures(url.as_str())
            .ok_or(BotecoError::NotAZoomURL(url.clone()))?
            .name("number")
            .ok_or(BotecoError::NotAZoomURL(url.clone()))?
            .as_str()
            .to_string();

        Ok(ZoomURL {
            full: url,
            short: format!("https://zoom.us/j/{number}"),
        })
    }
}
