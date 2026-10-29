use crate::game::types::{AccountConfig, PlayerSession, PlayerState};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;
use std::time::Duration;
use tracing::{error, info};

pub struct LolidleClient {
    client: reqwest::Client,
    base_url: String,
    token: String,
    instance_id: String,
}

impl LolidleClient {
    pub fn new(config: &AccountConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            ),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Idioma", HeaderValue::from_static("pt"));

        let inst = uuid::Uuid::new_v4().simple().to_string();
        if let Ok(v) = HeaderValue::from_str(&inst) {
            headers.insert("X-Instancia", v);
        }

        if !config.credentials.token.is_empty() {
            let auth_str = format!("Bearer {}", config.credentials.token);
            if let Ok(val) = HeaderValue::from_str(&auth_str) {
                headers.insert(AUTHORIZATION, val);
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(15))
            .build()
            .expect("HTTP Client");

        let base_url = if config.credentials.origin.is_empty() {
            "https://lolidle.com".to_string()
        } else {
            config.credentials.origin.trim_end_matches('/').to_string()
        };

        Self {
            client,
            base_url,
            token: config.credentials.token.clone(),
            instance_id: inst,
        }
    }

    pub async fn get_eu(&self) -> Result<PlayerSession, String> {
        let url = format!("{}/api/eu", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Falha de conexão /api/eu: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("Erro HTTP /api/eu: {}", resp.status()));
        }

        resp.json::<PlayerSession>()
            .await
            .map_err(|e| format!("Falha de decode /api/eu: {e}"))
    }

    pub async fn save_state(&self, state: &PlayerState) -> Result<bool, String> {
        let url = format!("{}/api/save", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "estado": state }))
            .send()
            .await
            .map_err(|e| format!("Falha de conexão /api/save: {e}"))?;

        Ok(resp.status().is_success())
    }

    pub async fn claim_daily(&self) -> Result<bool, String> {
        let url = format!("{}/api/recompensas/diaria", self.base_url);
        let resp = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(|e| format!("Falha diária: {e}"))?;
        Ok(resp.status().is_success())
    }

    pub async fn ornn_forge(&self, state: &PlayerState, slot_idx: usize) -> Result<Value, String> {
        let url = format!("{}/api/ornn/forjar", self.base_url);
        let payload = serde_json::json!({
            "estado": state,
            "onde": "itens",
            "k": slot_idx,
            "vezes": 1
        });
        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Falha Ornn: {e}"))?;

        resp.json::<Value>().await.map_err(|e| e.to_string())
    }
}
