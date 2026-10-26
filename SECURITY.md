# Security policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x (latest) | ✅ |
| older | ❌ |

We only patch the most recent published version on crates.io.

---

## Reporting a vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Instead, email **YOUR_CONTACT_EMAIL** with:

- A description of the vulnerability and its potential impact
- Steps to reproduce (proof-of-concept if possible)
- Any suggested mitigations

You will receive an acknowledgement within **48 hours** and a resolution timeline within **7 days**.

We follow [coordinated disclosure](https://en.wikipedia.org/wiki/Responsible_disclosure): we ask that you give us at least **90 days** to release a fix before any public disclosure.

---

## Scope

Areas of particular interest:

| Area | Notes |
|------|-------|
| `src/keys.rs` — ed25519 keygen | Incorrect randomness or key derivation could produce weak keys |
| `src/keys.rs` — CRC-16 / base32 | Validation bypass would allow accepting malformed keys |
| `src/network.rs` — HTTP client | SSRF or TLS downgrade issues |
| `src/scaffold.rs` — file writes | Path traversal when writing scaffolded projects |

Out of scope:

- Vulnerabilities in third-party crates (report those upstream)
- Issues requiring physical access to the user's machine
- Social-engineering attacks on users

---

## Disclosure credit

We will acknowledge security researchers in the release notes unless they prefer to remain anonymous.
