use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

mod keys;
mod network;
mod horizon;
mod scaffold;

use network::Network;

// ---------------------------------------------------------------------------
// CLI definition
// ---------------------------------------------------------------------------

/// stellar-qk — Stellar developer toolkit
///
/// Offline key utilities plus live Testnet/Futurenet helpers.
#[derive(Parser)]
#[command(
    name = "stellar-qk",
    version,
    author,
    about = "Stellar developer toolkit: keygen · validate · fund · balance · init",
    long_about = None,
    propagate_version = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new Stellar keypair
    Keygen {
        /// Output as JSON instead of plain text
        #[arg(long)]
        json: bool,
    },

    /// Validate the format and checksum of a Stellar key (offline)
    Validate {
        /// The G... public key or S... secret seed to validate
        key: String,
    },

    /// Fund an account via Friendbot (Testnet or Futurenet only)
    Fund {
        /// The G... address to fund
        address: String,

        /// Network to use [testnet | futurenet]
        #[arg(long, default_value = "testnet")]
        network: String,
    },

    /// Show the XLM and asset balances for an address
    Balance {
        /// The G... address to query
        address: String,

        /// Network to use [testnet | futurenet | mainnet]
        #[arg(long, default_value = "testnet")]
        network: String,

        /// Output as JSON instead of a table
        #[arg(long)]
        json: bool,
    },

    /// Scaffold a new Stellar Rust starter project
    Init {
        /// Project name (used as the crate name and directory name)
        name: String,

        /// Template to use [basic | funded]
        #[arg(long, default_value = "basic")]
        template: String,
    },
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { json } => cmd_keygen(json),
        Commands::Validate { key } => cmd_validate(&key),
        Commands::Fund { address, network } => cmd_fund(&address, &network).await,
        Commands::Balance { address, network, json } => cmd_balance(&address, &network, json).await,
        Commands::Init { name, template } => cmd_init(&name, &template),
    }
}

// ---------------------------------------------------------------------------
// Command implementations
// ---------------------------------------------------------------------------

fn cmd_keygen(json: bool) -> Result<()> {
    let kp = keys::generate_keypair();

    if json {
        let out = serde_json::json!({
            "public_key": kp.public_key,
            "secret_seed": kp.secret_seed,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("{} {}", "Public key :".bold(), kp.public_key.green());
        println!("{} {}", "Secret seed:".bold(), kp.secret_seed.yellow());
        println!();
        println!("{}", "⚠  Store your secret seed somewhere safe — it cannot be recovered.".red());
    }
    Ok(())
}

fn cmd_validate(key: &str) -> Result<()> {
    match keys::validate_key(key) {
        Ok(keys::KeyType::PublicKey) => {
            println!("{} Valid Stellar public key (G...)", "✓".green().bold());
        }
        Ok(keys::KeyType::SecretSeed) => {
            println!("{} Valid Stellar secret seed (S...)", "✓".green().bold());
        }
        Err(e) => {
            println!("{} {}", "✗ Invalid key:".red().bold(), e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn cmd_fund(address: &str, network_str: &str) -> Result<()> {
    let net = parse_network(network_str)?;

    if net == Network::Mainnet {
        anyhow::bail!("Friendbot is not available on Mainnet.");
    }

    println!("Funding {} on {}…", address.cyan(), net.name().bold());

    match network::friendbot_fund(address, net).await {
        Ok(tx_hash) => {
            println!("{} Account funded!", "✓".green().bold());
            println!("  Transaction: {}", tx_hash.dimmed());
        }
        Err(e) => {
            println!("{} {}", "✗ Fund failed:".red().bold(), e);
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn cmd_balance(address: &str, network_str: &str, json: bool) -> Result<()> {
    let net = parse_network(network_str)?;

    println!("Fetching balances for {} on {}…", address.cyan(), net.name().bold());

    let balances = horizon::fetch_balances(address, net).await?;

    if balances.is_empty() {
        println!("No balances found (account may not exist yet).");
        return Ok(());
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&balances)?);
    } else {
        use comfy_table::{Table, Cell, presets::UTF8_FULL};
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec![
            Cell::new("Asset").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Balance").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Limit").add_attribute(comfy_table::Attribute::Bold),
        ]);
        for b in &balances {
            table.add_row(vec![&b.asset, &b.balance, b.limit.as_deref().unwrap_or("—")]);
        }
        println!("{table}");
    }
    Ok(())
}

fn cmd_init(name: &str, template: &str) -> Result<()> {
    scaffold::init_project(name, template)?;
    println!("{} Created project `{}`", "✓".green().bold(), name.cyan());
    println!();
    println!("Next steps:");
    println!("  cd {name}");
    println!("  cargo build");
    println!("  cargo run");
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_network(s: &str) -> Result<Network> {
    match s.to_lowercase().as_str() {
        "testnet"   => Ok(Network::Testnet),
        "futurenet" => Ok(Network::Futurenet),
        "mainnet"   => Ok(Network::Mainnet),
        other => anyhow::bail!("Unknown network '{}'. Use testnet, futurenet, or mainnet.", other),
    }
}
