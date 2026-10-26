# Wave backlog

13 issues sorted by category.  File each one on GitHub using the
**Feature request** template and link the issue number back here once created.

Point values: **Trivial = 100 pts · Medium = 150 pts · High = 200 pts**

---

## Trivial (100 pts each)

### T-1 — Publish to crates.io

**GitHub issue:** <!-- #N once filed -->

Publish `stellar-qk` to crates.io so developers can install it with
`cargo install stellar-qk`.

Acceptance criteria:
- `Cargo.toml` has all required metadata (description, license, repository, readme, keywords, categories).
- `cargo publish --dry-run` passes with no errors.
- The crate is published and `cargo install stellar-qk` works.

---

### T-2 — `--output` flag for `keygen`

**GitHub issue:** <!-- #N once filed -->

Add `--output <FILE>` to `stellar-qk keygen` to write the keypair to a
file instead of stdout.

Acceptance criteria:
- `stellar-qk keygen --output keys.json --json` writes a valid JSON file.
- `stellar-qk keygen --output keys.txt` writes the plain-text format.
- If the file already exists, the command refuses unless `--force` is also
  passed.
- Unit test covers the file-already-exists guard.

---

### T-3 — Shell completions sub-command

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk completions <SHELL>` to emit shell completion scripts.
Supported shells: `bash`, `zsh`, `fish`, `powershell`.

Acceptance criteria:
- Uses `clap_complete` to generate completions at runtime.
- `stellar-qk completions bash` prints a valid bash completion script to
  stdout.
- README install steps show how to activate completions for each shell.

---

### T-4 — `--count N` for batch keygen

**GitHub issue:** <!-- #N once filed -->

Add `--count <N>` to `stellar-qk keygen` to generate N keypairs in one
invocation.

Acceptance criteria:
- `stellar-qk keygen --count 5` prints five keypairs, each on its own line
  block.
- `stellar-qk keygen --count 5 --json` emits a JSON array of N objects.
- N must be in the range 1–1000; values outside that range produce a clear
  error.
- Unit test checks the array length matches N.

---

### T-5 — `--network` short alias `-n`

**GitHub issue:** <!-- #N once filed -->

Add `-n` as a short alias for `--network` on `fund`, `balance`, and future
network-aware commands.

Acceptance criteria:
- `stellar-qk balance ADDR -n mainnet` is equivalent to
  `stellar-qk balance ADDR --network mainnet`.
- All three commands (`fund`, `balance`, and `init` if applicable) support
  the alias.
- `--help` output lists `-n` next to `--network`.

---

## Medium (150 pts each)

### M-1 — Vanity key generator

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk keygen --prefix <PREFIX>` to generate keypairs whose
public key starts with `G<PREFIX>`.

Acceptance criteria:
- Brute-forces random seeds until the encoded public key matches the
  desired prefix (case-insensitive).
- Prints progress every 100 000 attempts.
- Rejects prefixes longer than 6 characters (safety limit on search time).
- `--prefix` and `--count` are mutually exclusive.
- Unit test: prefix `"G"` always matches immediately.

---

### M-2 — Transaction envelope decoder

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk decode <XDR_BASE64>` to decode and pretty-print a
base64-encoded Stellar XDR transaction envelope.

Acceptance criteria:
- Accepts the envelope on stdin if `<XDR_BASE64>` is `-`.
- Outputs a human-readable summary: source account, sequence number,
  operation type(s), fee, memo.
- `--json` emits the decoded envelope as JSON.
- Tested with at least one real Testnet transaction envelope.

---

### M-3 — `stellar-qk send` — payment on Testnet

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk send --from <SEED> --to <ADDR> --amount <XLM>` to build
and submit a native XLM payment on Testnet.

Acceptance criteria:
- Builds a minimal payment transaction (fee, sequence number fetched from
  Horizon).
- Signs with the ed25519 implementation in `keys.rs` (adds signing path).
- Submits via the Horizon `POST /transactions` endpoint.
- Prints the resulting transaction hash.
- `--network` flag works (testnet/futurenet only; mainnet blocked with a
  confirmation prompt).
- Integration test using the Testnet (skipped in offline CI, run manually).

---

### M-4 — Config file (`~/.config/stellar-qk/config.toml`)

**GitHub issue:** <!-- #N once filed -->

Let users persist default settings (network, Horizon URL overrides) in a
TOML config file so they don't have to pass `--network` every time.

Acceptance criteria:
- Config file is loaded from `$XDG_CONFIG_HOME/stellar-qk/config.toml` on
  Linux/macOS and `%APPDATA%\stellar-qk\config.toml` on Windows.
- Supports `default_network`, `horizon_testnet_url`,
  `horizon_futurenet_url`, `horizon_mainnet_url` keys.
- CLI flags override config-file values.
- `stellar-qk config show` prints the active configuration.
- Unit tests cover the override precedence logic.

---

### M-5 — Structured logging with `--verbose`

**GitHub issue:** <!-- #N once filed -->

Add `--verbose` / `-v` global flag that emits structured (JSON) log lines
to stderr for every HTTP request and response.

Acceptance criteria:
- At `-v` level, each outbound request logs: timestamp, method, URL, status
  code, latency (ms).
- At `-vv` level, also logs request/response bodies (truncated to 1 KB).
- Secrets (secret seeds, auth headers) are never logged.
- Uses the `tracing` + `tracing-subscriber` crates.

---

## High (200 pts each)

### H-1 — Soroban contract deploy template

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk init <NAME> --template soroban` to scaffold a minimal
Soroban smart contract project.

Acceptance criteria:
- Generated project compiles with `cargo build --target wasm32-unknown-unknown`.
- Includes a `deploy.sh` script that uploads and instantiates the contract
  on Testnet using the Soroban CLI.
- `README.md` in the generated project explains the Soroban build + deploy
  flow.
- The template is embedded in the binary via `include_str!` like the
  existing `basic` and `funded` templates.

---

### H-2 — Ledger hardware wallet key support

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk keygen --ledger` and `stellar-qk validate --ledger` to
read the public key from a connected Ledger hardware wallet.

Acceptance criteria:
- Uses the Ledger HID transport to send the `GET_PUBLIC_KEY` APDU to the
  Stellar app.
- Prints the public key and BIP-32 derivation path used.
- Gracefully errors if no Ledger is connected or the Stellar app is not open.
- Works on Linux (hidraw), macOS (IOHIDDevice), and Windows (HID API).
- Does not expose the private key at any point.

---

### H-3 — Multi-signature transaction builder

**GitHub issue:** <!-- #N once filed -->

Add `stellar-qk multisig` sub-commands to manage multi-signature accounts
and co-sign XDR envelopes.

Acceptance criteria:
- `stellar-qk multisig add-signer --account <ADDR> --signer <ADDR> --weight <N>
  --from <SEED>` submits a `SET_OPTIONS` operation to add a signer on
  Testnet.
- `stellar-qk multisig sign --envelope <XDR_FILE> --seed <SEED>` adds a
  signature to an existing XDR envelope file and writes the updated
  envelope back to the file.
- `stellar-qk multisig inspect --envelope <XDR_FILE>` lists all existing
  signatures and the signers' weights.
- Unit tests cover the signature append and inspect paths using a
  fixed test envelope.
