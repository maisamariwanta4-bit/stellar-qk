# Maintainer guide — Drips Wave program

This document explains how `stellar-qk` participates in the
[Drips Wave](https://drips.network) open-source funding program and what
maintainers need to do to keep the project healthy and the funding flowing.

---

## What is Drips Wave?

Drips Wave is a program that distributes funds to maintainers of open-source
projects that Stellar ecosystem teams depend on.  Projects earn points by
closing issues from their backlog; points convert to a share of the monthly
Wave allocation.

---

## Point values

| Category | Points per issue |
|----------|-----------------|
| Trivial  | 100 |
| Medium   | 150 |
| High     | 200 |

Points are assigned when an issue is **closed with a merged PR** that
references it (`Closes #N`).  Closing without a PR (e.g. "won't fix")
does not earn points.

---

## Issue backlog

The full backlog is in [`WAVE_BACKLOG.md`](WAVE_BACKLOG.md).

Issues are filed on GitHub from that list.  Each GitHub issue should:

1. Use the **Feature request** template.
2. Copy the description from the backlog entry.
3. Add the Wave label matching its category: `wave:trivial`, `wave:medium`,
   or `wave:high`.
4. Be linked back from the backlog entry once it has a GitHub issue number.

---

## Applying to the Wave program

1. Create the repository inside a **GitHub organization** (not a personal
   account).
2. Get CI green on `main` (all four jobs: fmt, clippy, test, build-release).
3. File at least three issues from the backlog (one from each category).
4. Go to the [Drips Wave app](https://www.drips.network/) →
   **Maintainers → Orgs and Repos** and submit the org/repo.
5. Approval is at the discretion of the Wave program organizers and typically
   takes 1–2 weeks.

---

## Monthly workflow

| Week | Action |
|------|--------|
| 1 | Triage new issues; label and assign |
| 2–3 | Work through backlog; open PRs |
| 4 | Merge PRs, close issues, update backlog |
| End of month | Drips snapshot — ensure all merged PRs have `Closes #N` |

---

## Replacing `YOUR_ORG` and `YOUR_CONTACT_EMAIL`

Before publishing the repository, do a global search-and-replace:

| Placeholder | Replace with |
|-------------|-------------|
| `YOUR_ORG` | Your GitHub organization name |
| `YOUR_CONTACT_EMAIL` | A monitored security/contact address |

Files that contain these placeholders:

- `Cargo.toml`
- `README.md`
- `SECURITY.md`
- `CODE_OF_CONDUCT.md`
- `docs/WAVE.md` (this file)
- `docs/WAVE_BACKLOG.md`

---

## Labels to create on GitHub

```
bug
enhancement
needs-triage
good first issue
help wanted
wave:trivial
wave:medium
wave:high
breaking-change
```

---

## Release process

1. Bump the version in `Cargo.toml`.
2. Add a `## [x.y.z] — YYYY-MM-DD` section to `CHANGELOG.md`.
3. Open a PR titled `chore: release x.y.z`.
4. After merge, tag the commit: `git tag vx.y.z && git push origin vx.y.z`.
5. The tag triggers crates.io publishing (add a publish workflow when ready).
