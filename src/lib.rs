//! # stellar-qk
//!
//! A Rust library and CLI for Stellar developers.
//!
//! ## Library quick-start
//!
//! ```rust,no_run
//! use stellar_qk::keys::{generate_keypair, validate_key, KeyType};
//! use stellar_qk::network::Network;
//!
//! // Generate a fresh keypair
//! let kp = generate_keypair();
//! println!("Public key : {}", kp.public_key);
//! println!("Secret seed: {}", kp.secret_seed);
//!
//! // Validate a key offline
//! match validate_key(&kp.public_key) {
//!     Ok(KeyType::PublicKey) => println!("Valid G... public key"),
//!     Ok(KeyType::SecretSeed) => println!("Valid S... secret seed"),
//!     Err(e) => println!("Invalid: {e}"),
//! }
//! ```
//!
//! ## Async operations
//!
//! ```rust,no_run
//! use stellar_qk::{network::Network, horizon};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let balances = horizon::fetch_balances("GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN",
//!                                             Network::Testnet).await?;
//!     for b in &balances {
//!         println!("{} {}", b.balance, b.asset);
//!     }
//!     Ok(())
//! }
//! ```

pub mod keys;
pub mod network;
pub mod horizon;
pub mod scaffold;

// Re-export the most-used types at the crate root for convenience.
pub use keys::{generate_keypair, validate_key, KeyPair, KeyType, KeyError};
pub use network::Network;
