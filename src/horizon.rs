//! Horizon API client — balance fetching.
//!
//! Only the account/balances endpoint is implemented; enough for the
//! `balance` command.  If you need full Horizon coverage, consider the
//! `stellar-sdk` crate.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::network::Network;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// A single balance line returned by Horizon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Human-readable asset label: "XLM" for native, or "CODE:ISSUER".
    pub asset: String,
    /// Balance as a string (e.g. "100.0000000").
    pub balance: String,
    /// Trust-line limit, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

// Horizon's raw balance object
#[derive(Deserialize)]
struct HorizonBalance {
    balance: String,
    asset_type: String,
    #[serde(default)]
    asset_code: String,
    #[serde(default)]
    asset_issuer: String,
    #[serde(default)]
    limit: String,
}

// Minimal account response — we only need the balances array.
#[derive(Deserialize)]
struct HorizonAccount {
    balances: Vec<HorizonBalance>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Fetch the balances for `address` from Horizon.
///
/// Returns an empty Vec if the account does not exist (404).
pub async fn fetch_balances(address: &str, network: Network) -> Result<Vec<Balance>> {
    let url = format!("{}/accounts/{}", network.horizon_url(), address);

    let client = reqwest::Client::builder()
        .user_agent(concat!("stellar-qk/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("failed to build HTTP client")?;

    let resp = client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("Horizon request to {url} failed"))?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(vec![]);
    }

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Horizon returned {}: {}", status, body);
    }

    let account: HorizonAccount = resp
        .json()
        .await
        .context("failed to parse Horizon account JSON")?;

    let balances = account
        .balances
        .into_iter()
        .map(|b| {
            let asset = if b.asset_type == "native" {
                "XLM".to_string()
            } else {
                format!("{}:{}", b.asset_code, &b.asset_issuer[..6])
            };
            let limit = if b.limit.is_empty() || b.limit == "0" {
                None
            } else {
                Some(b.limit)
            };
            Balance {
                asset,
                balance: b.balance,
                limit,
            }
        })
        .collect();

    Ok(balances)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke-test the asset label logic for the native asset.
    #[test]
    fn native_asset_label() {
        let raw = HorizonBalance {
            balance: "100.0000000".to_string(),
            asset_type: "native".to_string(),
            asset_code: String::new(),
            asset_issuer: String::new(),
            limit: String::new(),
        };
        let b = Balance {
            asset: if raw.asset_type == "native" {
                "XLM".to_string()
            } else {
                format!("{}:{}", raw.asset_code, &raw.asset_issuer[..6])
            },
            balance: raw.balance.clone(),
            limit: None,
        };
        assert_eq!(b.asset, "XLM");
        assert_eq!(b.balance, "100.0000000");
    }

    /// Non-native asset label uses CODE:ISSUER[:6].
    #[test]
    fn non_native_asset_label() {
        let issuer = "GABCDE1234567890ABCDE1234567890ABCDE1234567890ABCDE12345";
        let raw = HorizonBalance {
            balance: "50.0000000".to_string(),
            asset_type: "credit_alphanum4".to_string(),
            asset_code: "USDC".to_string(),
            asset_issuer: issuer.to_string(),
            limit: "1000.0000000".to_string(),
        };
        let asset = format!("{}:{}", raw.asset_code, &raw.asset_issuer[..6]);
        assert_eq!(asset, "USDC:GABCDE");
    }
}
