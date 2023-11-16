use crate::errors::BotecoError;

pub struct CloudFlareSettings {
    pub api_token: String,
    pub zone_id: String,
}

pub struct ImprovMxSettings {
    pub api_token: String,
}

pub struct Settings {
    pub private_url: String,
    pub public_url: String,
    pub email: String,
    pub cloud_flare: CloudFlareSettings,
    pub improv_mx: ImprovMxSettings,
}

const PRIVATE_URL: &str = "BOTECO_PRIVATE_URL";
const PUBLIC_URL: &str = "BOTECO_PUBLIC_URL";
const EMAIL: &str = "BOTECO_EMAIL";

const CLOUD_FLARE_ZONE_ID: &str = "BOTECO_CLOUD_FLARE_ZONE_ID";
const CLOUD_FLARE_API_TOKEN: &str = "BOTECO_CLOUD_FLARE_API_TOKEN";

const IMPROVMX_API_TOKEN: &str = "BOTECO_IMPROVMX_API_KEY";

impl Settings {
    pub fn new() -> Result<Self, BotecoError> {
        [
            PRIVATE_URL,
            PUBLIC_URL,
            EMAIL,
            CLOUD_FLARE_API_TOKEN,
            CLOUD_FLARE_ZONE_ID,
            IMPROVMX_API_TOKEN,
        ]
        .iter()
        .map(|k| std::env::var(k).map_err(|_| BotecoError::MissingEnvVar(k.to_string())))
        .collect::<Result<Vec<String>, BotecoError>>()
        .map(|args| Settings {
            private_url: args[0].clone(),
            public_url: args[1].clone(),
            email: args[2].clone(),
            cloud_flare: CloudFlareSettings {
                api_token: args[3].clone(),
                zone_id: args[4].clone(),
            },
            improv_mx: ImprovMxSettings {
                api_token: args[5].clone(),
            },
        })
    }
}
