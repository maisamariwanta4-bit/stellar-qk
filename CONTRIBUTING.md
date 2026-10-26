# Contributing to stellar-qk

Thank you for taking the time to contribute! This document covers everything you need to open a good issue or pull request.

---

## Table of contents

- [Code of conduct](#code-of-conduct)
- [How to report a bug](#how-to-report-a-bug)
- [How to request a feature](#how-to-request-a-feature)
- [Development setup](#development-setup)
- [Making a change](#making-a-change)
- [Pull request checklist](#pull-request-checklist)
- [Commit style](#commit-style)
- [CI checks](#ci-checks)

---

## Code of conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).  
By participating you agree to abide by its terms.

---

## How to report a bug

1. Search [existing issues](https://github.com/YOUR_ORG/stellar-quickstart-kit/issues) first — the bug may already be tracked.
2. If not, open a **Bug report** issue using the template and include:
   - Your OS and Rust version (`rustc --version`)
   - The exact command you ran
   - The full output (stdout + stderr)
   - What you expected to happen

For security vulnerabilities, **do not open a public issue** — see [SECURITY.md](SECURITY.md).

---

## How to request a feature

Open a **Feature request** issue using the template and describe:

- The problem you are trying to solve
- Your proposed solution or API
- Any alternatives you considered

---

## Development setup

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_ORG/stellar-quickstart-kit.git
cd stellar-quickstart-kit

# 2. Make sure you have a recent stable Rust
rustup update stable

# 3. Build
cargo build

# 4. Run the test suite (offline tests only — no network required)
cargo test

# 5. Check formatting and lints
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Making a change

1. Create a branch from `main`:
   ```bash
   git checkout -b feat/my-feature   # or fix/my-bug
   ```
2. Make your changes, keeping commits focused and small.
3. Add or update tests that cover the new behaviour.
4. Run `cargo fmt --all` and `cargo clippy` before pushing.
5. Open a pull request against `main` and fill in the PR template.

### Adding a new command

- Add the `clap` variant in `src/main.rs` under `Commands`.
- Implement the logic in the appropriate module (`src/keys.rs`, `src/network.rs`, etc.) or create a new one.
- Register any new module in `src/lib.rs` so library consumers can use it too.
- Add unit tests in the module and an integration smoke-test if the command is async.

### Changing crypto code

`src/keys.rs` contains a from-scratch implementation of ed25519 (SHA-512, field arithmetic, extended Edwards coordinates).  Any change there **must** keep all existing test vectors passing, including:

- `crc16_check_value` — CRC-16/XModem NIST check value `0x31C3`
- `ed25519_rfc8032_vector1` — RFC 8032 §6.1 vector 1
- `sha512_abc` — NIST SHA-512("abc") vector

---

## Pull request checklist

- [ ] `cargo build` passes without warnings
- [ ] `cargo test` passes
- [ ] `cargo fmt --all -- --check` passes (run `cargo fmt --all` to fix)
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] New public API is documented with `///` doc comments
- [ ] `CHANGELOG.md` entry added under `[Unreleased]` (if this is a user-visible change)
- [ ] PR description references the issue it resolves (`Closes #N`)

---

## Commit style

We loosely follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add --count flag to keygen
fix: correct CRC endianness for keys with high bytes
docs: expand library usage examples in README
chore: bump reqwest to 0.12.5
test: add SHA-512 empty-string vector
```

Keep the subject line under 72 characters.  Use the body for anything that needs more explanation.

---

## CI checks

Every PR runs the following on Linux, macOS, and Windows:

| Step | Command |
|------|---------|
| Format | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` |
| Test | `cargo test --all-features` |
| Build release | `cargo build --release` |

All four must be green before a PR can be merged.
