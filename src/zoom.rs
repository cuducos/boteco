use regex::Regex;

use crate::errors::BotecoError;

pub struct ZoomUrl {
    pub full: String,
    pub short: String,
}

impl ZoomUrl {
    pub fn new(url: String) -> Result<Self, BotecoError> {
        let number = Regex::new(r"/j/(?P<number>\d+)\?\w+")
            .map_err(BotecoError::RegexError)?
            .captures(url.as_str())
            .ok_or(BotecoError::NotAZoomUrk(url.clone()))?
            .name("number")
            .ok_or(BotecoError::NotAZoomUrk(url.clone()))?
            .as_str()
            .to_string();

        Ok(ZoomUrl {
            full: url,
            short: format!("https://zoom.us/j/{number}"),
        })
    }
}
