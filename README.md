# stellar-qk

[![CI](https://github.com/YOUR_ORG/stellar-quickstart-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_ORG/stellar-quickstart-kit/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/stellar-qk.svg)](https://crates.io/crates/stellar-qk)
[![docs.rs](https://docs.rs/stellar-qk/badge.svg)](https://docs.rs/stellar-qk)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

A Rust CLI and library for Stellar developers.  
Offline key utilities plus live Testnet / Futurenet helpers — no OpenSSL, no Stellar SDK required.

---

## Table of contents

- [Install](#install)
- [Quick start](#quick-start)
- [Command reference](#command-reference)
  - [keygen](#keygen)
  - [validate](#validate)
  - [fund](#fund)
  - [balance](#balance)
  - [init](#init)
- [Library usage](#library-usage)
- [Architecture](#architecture)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Install

### From crates.io (once published)

```bash
cargo install stellar-qk
```

### From source

```bash
git clone https://github.com/YOUR_ORG/stellar-quickstart-kit.git
cd stellar-quickstart-kit
cargo install --path .
```

### Prerequisites

- Rust 1.74 or newer — install via [rustup](https://rustup.rs)

---

## Quick start

```bash
# Generate a fresh keypair
$ stellar-qk keygen
Public key :  GCKZQMQM...ABCDEF
Secret seed:  SCZXYZ...123456

⚠  Store your secret seed somewhere safe — it cannot be recovered.

# Validate a key offline (no network needed)
$ stellar-qk validate GCKZQMQM...ABCDEF
✓ Valid Stellar public key (G...)

# Fund a new account on Testnet
$ stellar-qk fund GCKZQMQM...ABCDEF
Funding GCKZQMQM...ABCDEF on Testnet…
✓ Account funded!
  Transaction: a1b2c3d4...

# Check the balance
$ stellar-qk balance GCKZQMQM...ABCDEF
Fetching balances for GCKZQMQM...ABCDEF on Testnet…
╒════════╤══════════════════╤═══════╕
│ Asset  │ Balance          │ Limit │
╞════════╪══════════════════╪═══════╡
│ XLM    │ 10000.0000000    │ —     │
╘════════╧══════════════════╧═══════╛

# Scaffold a new project
$ stellar-qk init my-dapp --template funded
✓ Created project `my-dapp`

Next steps:
  cd my-dapp
  cargo build
  cargo run
```

> **Note** — The keys shown above are illustrative placeholders.  
> Real keys are generated fresh each run and are never logged or transmitted.

---

## Command reference

### keygen

Generate a new ed25519 Stellar keypair.

```
stellar-qk keygen [--json]
```

| Flag | Description |
|------|-------------|
| `--json` | Emit `{ "public_key": "G...", "secret_seed": "S..." }` instead of plain text |

The private scalar is derived from 32 bytes of OS CSPRNG entropy (`BCryptGenRandom` on Windows, `/dev/urandom` on Unix).

---

### validate

Check a key's format and CRC-16 checksum entirely offline.

```
stellar-qk validate <KEY>
```

Accepts both `G...` public keys and `S...` secret seeds.  
Exits with code `1` if the key is invalid.

---

### fund

Fund a Testnet or Futurenet account through Friendbot.

```
stellar-qk fund <ADDRESS> [--network testnet|futurenet]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--network` | `testnet` | Network to use (`testnet` or `futurenet`) |

Not available on Mainnet.

---

### balance

Fetch XLM and asset balances from Horizon.

```
stellar-qk balance <ADDRESS> [--network testnet|futurenet|mainnet] [--json]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--network` | `testnet` | Network to query |
| `--json` | — | Emit a JSON array of balance objects |

---

### init

Scaffold a new Stellar Rust starter project.

```
stellar-qk init <NAME> [--template basic|funded]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--template` | `basic` | `basic` — keygen + offline validate; `funded` — keygen + Friendbot + balance |

Creates `./<NAME>/` with a ready-to-run `cargo run` project.

---

## Library usage

Add to your `Cargo.toml`:

```toml
[dependencies]
stellar-qk = "0.1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

### Generate a keypair

```rust
use stellar_qk::keys::generate_keypair;

let kp = generate_keypair();
println!("pub  {}", kp.public_key);
println!("seed {}", kp.secret_seed);
```

### Validate offline

```rust
use stellar_qk::keys::{validate_key, KeyType};

match validate_key("GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN") {
    Ok(KeyType::PublicKey)  => println!("valid public key"),
    Ok(KeyType::SecretSeed) => println!("valid secret seed"),
    Err(e)                  => eprintln!("invalid: {e}"),
}
```

### Fund and fetch balance

```rust
use stellar_qk::{
    keys::generate_keypair,
    network::{Network, friendbot_fund},
    horizon::fetch_balances,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kp = generate_keypair();

    friendbot_fund(&kp.public_key, Network::Testnet).await?;

    let balances = fetch_balances(&kp.public_key, Network::Testnet).await?;
    for b in &balances {
        println!("{}: {}", b.asset, b.balance);
    }
    Ok(())
}
```

---

## Architecture

```
stellar-quickstart-kit/
├── src/
│   ├── main.rs       — CLI entry point (clap derive)
│   ├── lib.rs        — Library re-exports
│   ├── keys.rs       — ed25519 keygen, base32, CRC-16 (no external crypto)
│   ├── network.rs    — Network enum, Friendbot client
│   ├── horizon.rs    — Horizon balance fetching
│   └── scaffold.rs   — `init` command, template writer
├── templates/
│   ├── basic/        — Minimal keygen starter
│   └── funded/       — Keygen + Friendbot + balance starter
├── docs/
│   ├── WAVE.md       — Maintainer guide
│   └── WAVE_BACKLOG.md — Issue backlog (Wave program)
└── .github/
    ├── workflows/ci.yml
    └── ISSUE_TEMPLATE/
```

### Key design decisions

- **No OpenSSL / no Stellar SDK** — the entire crypto path (SHA-512, GF(2²⁵⁵−19), extended Edwards coordinates, CRC-16) is implemented from scratch in `keys.rs` following RFC 8032 and the XModem CRC spec.  This keeps the dependency tree small and cross-compilation simple.
- **`rustls` for TLS** — `reqwest` is configured with `rustls-tls` so there is no system OpenSSL dependency on any platform.
- **Embedded templates** — starter project files are embedded at compile time via `include_str!`, so `stellar-qk` ships as a single binary with no runtime data files.

---

## Roadmap

| # | Item | Effort |
|---|------|--------|
| 1 | Publish to crates.io | Trivial |
| 2 | `--output` flag for `keygen` to write keys to file | Trivial |
| 3 | Shell completions (`stellar-qk completions bash/zsh/fish`) | Trivial |
| 4 | `--count N` flag for `keygen` to batch-generate keypairs | Trivial |
| 5 | Vanity key generator (`stellar-qk keygen --prefix GABC`) | Medium |
| 6 | Transaction envelope decoder (`stellar-qk decode <XDR>`) | Medium |
| 7 | `stellar-qk send` — build and submit a payment on Testnet | Medium |
| 8 | Soroban contract deploy template | High |
| 9 | Hardware wallet (Ledger) key support | High |

See [`docs/WAVE_BACKLOG.md`](docs/WAVE_BACKLOG.md) for the full backlog with Wave point values.

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

---

## License

Licensed under either of

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
