//! Network definitions and Friendbot funding helper.

use anyhow::{Context, Result};
use serde::Deserialize;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Network enum
// ---------------------------------------------------------------------------

/// A Stellar network target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Testnet,
    Futurenet,
    Mainnet,
}

impl Network {
    /// Human-readable name shown in CLI output.
    pub fn name(&self) -> &'static str {
        match self {
            Network::Testnet   => "Testnet",
            Network::Futurenet => "Futurenet",
            Network::Mainnet   => "Mainnet (Public)",
        }
    }

    /// Base URL of the Horizon server for this network.
    pub fn horizon_url(&self) -> &'static str {
        match self {
            Network::Testnet   => "https://horizon-testnet.stellar.org",
            Network::Futurenet => "https://horizon-futurenet.stellar.org",
            Network::Mainnet   => "https://horizon.stellar.org",
        }
    }

    /// Friendbot URL, if available.
    pub fn friendbot_url(&self) -> Option<&'static str> {
        match self {
            Network::Testnet   => Some("https://friendbot.stellar.org"),
            Network::Futurenet => Some("https://friendbot-futurenet.stellar.org"),
            Network::Mainnet   => None,
        }
    }

    /// The network passphrase used in transaction signing.
    pub fn passphrase(&self) -> &'static str {
        match self {
            Network::Testnet =>
                "Test SDF Network ; September 2015",
            Network::Futurenet =>
                "Test SDF Future Network ; October 2022",
            Network::Mainnet =>
                "Public Global Stellar Network ; September 2015",
        }
    }
}

// ---------------------------------------------------------------------------
// Friendbot errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum FrienbotError {
    #[error("Friendbot is not available on {0}")]
    NotAvailable(String),
    #[error("Friendbot request failed: {0}")]
    Http(String),
    #[error("Unexpected Friendbot response: {0}")]
    Unexpected(String),
}

// ---------------------------------------------------------------------------
// Friendbot
// ---------------------------------------------------------------------------

/// Fund an account on Testnet or Futurenet using Friendbot.
///
/// Returns the transaction hash on success.
pub async fn friendbot_fund(address: &str, network: Network) -> Result<String> {
    let url = network
        .friendbot_url()
        .ok_or_else(|| FrienbotError::NotAvailable(network.name().to_string()))?;

    let client = reqwest::Client::builder()
        .user_agent(concat!("stellar-qk/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("failed to build HTTP client")?;

    let resp = client
        .get(url)
        .query(&[("addr", address)])
        .send()
        .await
        .with_context(|| format!("Friendbot request to {url} failed"))?;

    let status = resp.status();
    let body: serde_json::Value = resp
        .json()
        .await
        .context("failed to parse Friendbot JSON response")?;

    if status.is_success() {
        // Extract the transaction hash
        let hash = body
            .get("hash")
            .and_then(|v| v.as_str())
            .unwrap_or("<unknown>")
            .to_string();
        Ok(hash)
    } else {
        // Friendbot returns {"detail": "..."} on error
        let detail = body
            .get("detail")
            .and_then(|v| v.as_str())
            .unwrap_or("no detail provided");
        anyhow::bail!("Friendbot returned {}: {}", status, detail)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mainnet_has_no_friendbot() {
        assert!(Network::Mainnet.friendbot_url().is_none());
    }

    #[test]
    fn testnet_has_friendbot() {
        assert!(Network::Testnet.friendbot_url().is_some());
    }

    #[test]
    fn horizon_urls_differ() {
        assert_ne!(
            Network::Testnet.horizon_url(),
            Network::Mainnet.horizon_url()
        );
    }

    #[test]
    fn passphrases_differ() {
        assert_ne!(
            Network::Testnet.passphrase(),
            Network::Mainnet.passphrase()
        );
    }
}
