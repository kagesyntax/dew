//! Minimal Turso (libSQL) client over the Hrana-over-HTTP `v2/pipeline` endpoint.
//! Works in wasm and native via `reqwest`, with no native SQLite dependency.
//!
//! Schema: see `db/schema.sql`.

use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, thiserror::Error)]
pub enum TursoError {
    #[error("Turso is not configured for this build")]
    NotConfigured,
    #[error("network error: {0}")]
    Network(String),
    #[error("database error: {0}")]
    Database(String),
}

#[derive(Clone)]
pub struct Turso {
    base_url: String,
    token: String,
}

#[derive(Deserialize)]
struct PipelineResponse {
    results: Vec<PipelineResult>,
}

#[derive(Deserialize)]
struct PipelineResult {
    #[serde(rename = "type")]
    kind: String,
    response: Option<Value>,
    error: Option<Value>,
}

impl Turso {
    /// Build from compile-time config (`TURSO_DATABASE_URL`, `TURSO_AUTH_TOKEN`).
    pub fn from_config() -> Result<Self, TursoError> {
        let url = crate::config::TURSO_DATABASE_URL.filter(|u| !u.is_empty()).ok_or(TursoError::NotConfigured)?;
        let token = crate::config::TURSO_AUTH_TOKEN.unwrap_or_default();
        Ok(Self::new(url, token))
    }

    pub fn new(url: &str, token: &str) -> Self {
        let base_url = url.replacen("libsql://", "https://", 1).trim_end_matches('/').to_string();
        Self { base_url, token: token.to_string() }
    }

    /// Run one statement and return rows as JSON values (column order preserved).
    pub async fn query(&self, sql: &str, args: &[&str]) -> Result<Vec<Vec<Value>>, TursoError> {
        let args: Vec<Value> = args.iter().map(|a| json!({ "type": "text", "value": a })).collect();
        let body = json!({
            "requests": [
                { "type": "execute", "stmt": { "sql": sql, "args": args } },
                { "type": "close" }
            ]
        });
        let resp = reqwest::Client::new()
            .post(format!("{}/v2/pipeline", self.base_url))
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await
            .map_err(|e| TursoError::Network(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(TursoError::Network(format!("HTTP {}", resp.status())));
        }
        let parsed: PipelineResponse = resp.json().await.map_err(|e| TursoError::Network(e.to_string()))?;
        let first = parsed.results.into_iter().next().ok_or_else(|| TursoError::Database("empty response".into()))?;
        if first.kind != "ok" {
            return Err(TursoError::Database(first.error.map(|e| e.to_string()).unwrap_or_default()));
        }
        let rows = first
            .response
            .and_then(|r| r.pointer("/result/rows").cloned())
            .and_then(|r| r.as_array().cloned())
            .unwrap_or_default();
        Ok(rows
            .into_iter()
            .map(|row| {
                row.as_array()
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|cell| cell.get("value").cloned().unwrap_or(Value::Null))
                    .collect()
            })
            .collect())
    }
}
