use serde::{Deserialize, Serialize};

use crate::{errors::BotecoError, settings::Settings};

const API_URL: &str = "https://api.improvmx.com/v3/domains/";

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    pub forward: String,
    pub alias: String,
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub alias: Alias,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct RequestBody {
    pub forward: String,
}

impl RequestBody {
    pub fn new(permanent: String, host: Option<String>) -> Self {
        RequestBody {
            forward: match host {
                Some(host) => format!("{},{}", permanent, host),
                None => permanent,
            },
        }
    }
}

pub struct ImprovMx {
    settings: Settings,
    client: reqwest::Client,
    host: Option<String>,
}

impl ImprovMx {
    pub fn new(host: Option<String>) -> Result<Self, BotecoError> {
        Ok(ImprovMx {
            settings: Settings::new()?,
            client: reqwest::Client::new(),
            host,
        })
    }

    pub async fn run(&self) -> Result<(), BotecoError> {
        let domain = self.settings.public_url.trim_end_matches('/');
        let url = format!("{API_URL}{domain}/aliases/host");
        let auth = format!("Basic api:{}", self.settings.improv_mx.api_token);
        let payload = RequestBody::new(self.settings.email.clone(), self.host.clone());
        let body = serde_json::to_string(&payload).map_err(BotecoError::SerializerError)?;
        let resp = self
            .client
            .put(url)
            .header("authorization", auth)
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(BotecoError::HttpError)?;

        let status = resp.status().as_str().to_string();
        if !resp.status().is_success() {
            let msg = resp
                .text()
                .await
                .unwrap_or_else(|_| "Could not ready the response body".to_string());
            return Err(BotecoError::CloudFlareApiError(status, msg));
        }

        let body: Response = resp.json().await.map_err(BotecoError::DeserializerError)?;
        if !body.success {
            return Err(BotecoError::ImprovMxApiError(
                status,
                serde_json::to_string(&body).map_err(BotecoError::SerializerError)?,
            ));
        }

        println!("host@{domain} => {}", body.alias.forward);

        Ok(())
    }
}
