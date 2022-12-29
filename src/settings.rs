use crate::errors::BotecoError;

pub struct Settings {
    pub api_token: String,
    pub zone_id: String,
    pub private_url: String,
    pub public_url: String,
}

const ZONE_ID: &str = "BOTECO_CLOUD_FLARE_ZONE_ID";
const API_TOKEN: &str = "BOTECO_CLOUD_FLARE_API_TOKEN";
const PRIVATE_URL: &str = "BOTECO_PRIVATE_URL";
const PUBLIC_URL: &str = "BOTECO_PUBLIC_URL";

impl Settings {
    pub fn new() -> Result<Self, BotecoError> {
        [API_TOKEN, ZONE_ID, PRIVATE_URL, PUBLIC_URL]
            .iter()
            .map(|k| std::env::var(k).map_err(|_| BotecoError::MissingEnvVar(k.to_string())))
            .collect::<Result<Vec<String>, BotecoError>>()
            .map(|args| Settings {
                api_token: args[0].clone(),
                zone_id: args[1].clone(),
                private_url: args[2].clone(),
                public_url: args[3].clone(),
            })
    }
}
