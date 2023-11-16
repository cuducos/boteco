use futures::future::try_join_all;
use serde::{Deserialize, Serialize};

use crate::{errors::BotecoError, settings::Settings, zoom::ZoomUrl};

const API_URL: &str = "https://api.cloudflare.com/client/v4/";

#[derive(Debug, Deserialize, Serialize)]
pub struct Constraint {
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    pub target: String,
    pub constraint: Constraint,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Value {
    pub url: String,
    pub status_code: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub id: String,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub id: String,
    pub targets: Vec<Target>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    pub result: Vec<Rule>,
}

#[derive(Debug, Serialize)]
pub struct Payload {
    pub targets: Vec<Target>,
    pub actions: Vec<Action>,
}

pub struct CloudFlare {
    settings: Settings,
    client: reqwest::Client,
    url: ZoomUrl,
}

impl CloudFlare {
    pub fn new(url: String) -> Result<Self, BotecoError> {
        Ok(CloudFlare {
            settings: Settings::new()?,
            client: reqwest::Client::new(),
            url: ZoomUrl::new(url)?,
        })
    }

    async fn rules(&self) -> Result<Vec<Rule>, BotecoError> {
        let endpoint = format!(
            "{}zones/{}/pagerules",
            API_URL, self.settings.cloud_flare.zone_id
        );
        let auth = format!("Bearer {}", self.settings.cloud_flare.api_token);
        let resp = self
            .client
            .get(endpoint)
            .header("authorization", auth)
            .send()
            .await
            .map_err(BotecoError::HttpError)?;

        let rules: Rules = resp.json().await.map_err(BotecoError::DeserializerError)?;
        Ok(rules.result)
    }

    fn is_target_rule(&self, rule: &Rule) -> bool {
        if rule.targets.len() != 1 || rule.actions.len() != 1 {
            debug!(
                "Skipping rule: {} targets and {} actions",
                rule.targets.len(),
                rule.actions.len()
            );
            return false;
        }
        for target in &rule.targets {
            if target.constraint.operator != "matches" {
                debug!("Skipping rule: operator is not \"matches\"");
                return false;
            }
            if target.constraint.value != self.settings.public_url
                && target.constraint.value != self.settings.private_url
            {
                debug!(
                    "Skipping rule: constraint target {} is not {} or {}",
                    target.constraint.value, self.settings.public_url, self.settings.private_url
                );
                return false;
            }
        }
        for action in &rule.actions {
            if action.id != "forwarding_url" || !action.value.url.contains("zoom.us") {
                debug!(
                    "Skipping rule: action is {} and it's URL is {}",
                    action.id, action.value.url
                );
                return false;
            }
        }
        true
    }

    fn redirect_url_for(&self, rule: &Rule) -> String {
        for target in &rule.targets {
            if target.constraint.value == self.settings.private_url {
                return self.url.full.clone();
            }
        }
        self.url.short.clone()
    }

    async fn redirect(&self, mut rule: Rule) -> Result<(), BotecoError> {
        if !self.is_target_rule(&rule) {
            return Ok(());
        }
        let endpoint = format!(
            "{}zones/{}/pagerules/{}",
            API_URL, self.settings.cloud_flare.zone_id, rule.id
        );
        let redirect_to = self.redirect_url_for(&rule);
        for action in &mut rule.actions {
            action.value.url = redirect_to.clone();
        }
        let payload = Payload {
            targets: rule.targets,
            actions: rule.actions,
        };
        let body = serde_json::to_string(&payload).map_err(BotecoError::SerializerError)?;
        let auth = format!("Bearer {}", self.settings.cloud_flare.api_token);
        let resp = self
            .client
            .put(endpoint)
            .header("authorization", auth)
            .body(body)
            .send()
            .await
            .map_err(BotecoError::HttpError)?;
        if !resp.status().is_success() {
            let status = resp.status().as_str().to_string();
            let msg = resp
                .text()
                .await
                .unwrap_or_else(|_| "Could not ready the response body".to_string());
            return Err(BotecoError::CloudFlareApiError(status, msg));
        }
        println!(
            "https://{} => {}",
            payload.targets[0].constraint.value, redirect_to
        );
        Ok(())
    }

    pub async fn run(&self) -> Result<(), BotecoError> {
        let tasks = self
            .rules()
            .await?
            .into_iter()
            .map(|rule| self.redirect(rule));
        try_join_all(tasks).await?;
        Ok(())
    }
}
