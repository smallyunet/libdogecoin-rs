//! Simple JSON-RPC client for Dogecoin Core compatible nodes.
//!
//! This module is enabled by default via the `rpc` feature.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// A minimal JSON-RPC client (Dogecoin Core / Bitcoin Core compatible).
#[derive(Debug, Clone)]
pub struct DogeRpcClient {
    url: String,
    auth: Option<(String, String)>,
    user_agent: String,
}

impl DogeRpcClient {
    /// Create a new client for the given RPC endpoint URL (e.g. `http://127.0.0.1:22555`).
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            auth: None,
            user_agent: "libdogecoin-rs".to_string(),
        }
    }

    /// Set HTTP Basic auth (typical for Dogecoin Core).
    pub fn with_basic_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.auth = Some((username.into(), password.into()));
        self
    }

    /// Override User-Agent header.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Generic JSON-RPC call.
    pub fn call<T: DeserializeOwned>(&self, method: &str, params: serde_json::Value) -> Result<T, RpcError> {
        let req = JsonRpcRequest {
            jsonrpc: "1.0",
            id: "libdogecoin-rs",
            method,
            params,
        };

        let mut http_req = ureq::post(&self.url)
            .set("Content-Type", "application/json")
            .set("Accept", "application/json")
            .set("User-Agent", &self.user_agent);

        if let Some((ref user, ref pass)) = self.auth {
            http_req = http_req.set("Authorization", &basic_auth_header(user, pass));
        }

        let resp = http_req.send_json(serde_json::to_value(req).map_err(RpcError::Serialize)?);

        match resp {
            Ok(r) => {
                let value: JsonRpcResponse<T> = r.into_json().map_err(RpcError::Deserialize)?;
                if let Some(err) = value.error {
                    return Err(RpcError::Remote(err));
                }
                value.result.ok_or(RpcError::MissingResult)
            }
            Err(ureq::Error::Status(code, r)) => {
                // Try to extract a JSON error body for diagnostics.
                let body: Result<serde_json::Value, _> = r.into_json();
                Err(RpcError::HttpStatus { code, body: body.ok() })
            }
            Err(e) => Err(RpcError::Transport(e)),
        }
    }

    /// Broadcast a raw transaction hex.
    pub fn send_raw_transaction(&self, raw_tx_hex: &str) -> Result<String, RpcError> {
        self.call("sendrawtransaction", serde_json::json!([raw_tx_hex]))
    }

    /// List unspent outputs (UTXOs) for the given addresses.
    pub fn list_unspent(
        &self,
        min_conf: u32,
        max_conf: u32,
        addresses: &[String],
    ) -> Result<Vec<ListUnspentEntry>, RpcError> {
        self.call(
            "listunspent",
            serde_json::json!([min_conf, max_conf, addresses]),
        )
    }

    /// Convenience: fetch UTXOs for one address.
    pub fn utxos_for_address(
        &self,
        address: &str,
        min_conf: u32,
        max_conf: u32,
    ) -> Result<Vec<ListUnspentEntry>, RpcError> {
        self.list_unspent(min_conf, max_conf, &[address.to_string()])
    }

    /// Convenience: compute balance from `listunspent` for one address.
    pub fn utxo_balance(&self, address: &str, min_conf: u32, max_conf: u32) -> Result<f64, RpcError> {
        let utxos = self.utxos_for_address(address, min_conf, max_conf)?;
        Ok(utxos.into_iter().map(|u| u.amount).sum())
    }
}

fn basic_auth_header(user: &str, pass: &str) -> String {
    // Basic base64(user:pass)
    use base64::Engine as _;
    let raw = format!("{user}:{pass}");
    let encoded = base64::engine::general_purpose::STANDARD.encode(raw.as_bytes());
    format!("Basic {encoded}")
}

/// `listunspent` response entry.
#[derive(Debug, Clone, Deserialize)]
pub struct ListUnspentEntry {
    pub txid: String,
    pub vout: u32,

    #[serde(default)]
    pub address: Option<String>,

    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,

    pub amount: f64,

    #[serde(default)]
    pub confirmations: u64,

    #[serde(default)]
    pub spendable: Option<bool>,

    #[serde(default)]
    pub solvable: Option<bool>,
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'a str,
    id: &'a str,
    method: &'a str,
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcErrorObject>,
    #[serde(rename = "id")]
    _id: serde_json::Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonRpcErrorObject {
    pub code: i64,
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum RpcError {
    #[error("transport error: {0}")]
    Transport(ureq::Error),

    #[error("http status {code}")]
    HttpStatus { code: u16, body: Option<serde_json::Value> },

    #[error("failed to serialize request: {0}")]
    Serialize(serde_json::Error),

    #[error("failed to deserialize response: {0}")]
    Deserialize(std::io::Error),

    #[error("remote error {0:?}")]
    Remote(JsonRpcErrorObject),

    #[error("missing result field")]
    MissingResult,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_header_shape() {
        let h = basic_auth_header("user", "pass");
        assert!(h.starts_with("Basic "));
        assert!(h.len() > "Basic ".len());
    }
}
