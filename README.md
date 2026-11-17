<div align="center">

```
 ██▓███   ██░ ██  ▄▄▄       ███▄    █ ▄▄▄█████▓ ▒█████   ███▄ ▄███▓
▓██░  ██▒▓██░ ██▒▒████▄     ██ ▀█   █ ▓  ██▒ ▓▒▒██▒  ██▒▓██▒▀█▀ ██▒
▓██░ ██▓▒▒██▀▀██░▒██  ▀█▄  ▓██  ▀█ ██▒▒ ▓██░ ▒░▒██░  ██▒▓██    ▓██░
▒██▄█▓▒ ▒░▓█ ░██ ░██▄▄▄▄██ ▓██▒  ▐▌██▒░ ▓██▓ ░ ▒██   ██░▒██    ▒██
▒██▒ ░  ░░▓█▒░██▓ ▓█   ▓██▒▒██░   ▓██░  ▒██▒ ░ ░ ████▓▒░▒██▒   ░██▒
▒▓▒░ ░  ░ ▒ ░░▒░▒ ▒▒   ▓▒█░░ ▒░   ▒ ▒   ▒ ░░   ░ ▒░▒░▒░ ░ ▒░   ░  ░
░▒ ░      ▒ ░▒░ ░  ▒   ▒▒ ░░ ░░   ░ ▒░    ░      ░ ▒ ▒░ ░ ░      ░
░        ░  ░░ ░  ░   ▒      ░   ░ ░   ░      ░ ░ ░ ▒  ░      ░
         ░  ░  ░      ░  ░         ░              ░ ░         ░
```

<h3>👻 The fastest username reconnaissance tool on the planet</h3>

<p>
  <strong>481 sites. Single binary. Seconds, not minutes.</strong><br>
  Rust-powered OSINT for digital footprint discovery.
</p>

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg?logo=rust" alt="Rust"></a>
  <a href="https://crates.io/crates/phantom-osint"><img src="https://img.shields.io/crates/v/phantom-osint.svg?label=crates.io" alt="Crates.io"></a>
  <a href="https://crates.io/crates/phantom-osint"><img src="https://img.shields.io/crates/d/phantom-osint.svg?label=downloads" alt="Downloads"></a>
  <img src="https://img.shields.io/badge/Sites-481-brightgreen.svg" alt="Sites">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg" alt="Platform">
  <a href="https://github.com/DBISHT4U/phantom"><img src="https://img.shields.io/github/repo-size/DBISHT4U/phantom?color=purple" alt="Repo Size"></a>
</p>

<p>
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-why-phantom">Why Phantom</a> •
  <a href="#%EF%B8%8F-usage">Usage</a> •
  <a href="#-how-it-works">How It Works</a> •
  <a href="#-categories">Categories</a> •
  <a href="#-custom-sites">Custom Sites</a> •
  <a href="#-contributing">Contributing</a>
</p>

<br>

<img src="https://img.shields.io/badge/-%F0%9F%91%A5%20Social%20Media-E74C3C?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%92%BB%20Dev%20Platforms-3498DB?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%8E%AE%20Gaming-2ECC71?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%8E%B5%20Media-9B59B6?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%92%AC%20Forums-F39C12?style=for-the-badge" alt="">
<br>
<img src="https://img.shields.io/badge/-%F0%9F%8E%A8%20Creative-E91E63?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%94%92%20Security-1ABC9C?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%92%B0%20Money-E67E22?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%94%97%20376%2B%20More-95A5A6?style=for-the-badge" alt="">

</div>

---

## 🚀 Quick Start

```bash
# Install
cargo install phantom-osint

# Hunt a username
phantom john_doe

# Filter by category
phantom john_doe --category social,dev

# Export results
phantom john_doe --output results.json
```

---

<div align="center">
  <img src="demo.gif" alt="Phantom Demo" width="800">
</div>

---

## ⚡ Why Phantom?

> **Phantom is the fastest username reconnaissance tool ever built.** It scans 481 sites in seconds — not minutes. Zero dependencies, single binary, real-time output. Nothing else comes close.

<table>
<tr>
<th></th>
<th>Sherlock <sub>(Python)</sub></th>
<th>GoSearch <sub>(Go)</sub></th>
<th>Phantom <sub>(Rust)</sub></th>
</tr>
<tr><td><b>Sites</b></td><td>481</td><td>~300</td><td><b>481</b></td></tr>
<tr><td><b>Scan time</b></td><td>~2–3 minutes</td><td>~15 seconds</td><td><b>~8–16 seconds</b></td></tr>
<tr><td><b>Install</b></td><td><code>pip</code> + Python + deps</td><td><code>go install</code></td><td><b>Single binary — zero deps</b></td></tr>
<tr><td><b>Concurrency</b></td><td>Threading (GIL-limited)</td><td>Goroutines</td><td><b>Tokio async (zero-cost futures)</b></td></tr>
<tr><td><b>Memory</b></td><td>~80 MB</td><td>~30 MB</td><td><b>~12 MB</b></td></tr>
<tr><td><b>Binary size</b></td><td>~50 MB (with Python)</td><td>~8 MB</td><td><b>~4 MB</b></td></tr>
<tr><td><b>Live output</b></td><td>After completion</td><td>After completion</td><td><b>Real-time as found</b></td></tr>
<tr><td><b>Export</b></td><td>txt / csv</td><td>txt / json</td><td><b>Color table + JSON + CSV</b></td></tr>
<tr><td><b>Proxy / Tor</b></td><td>✅</td><td>❌</td><td><b>✅ HTTP + SOCKS5</b></td></tr>
<tr><td><b>Category filter</b></td><td>❌</td><td>❌</td><td><b>✅ 9 categories</b></td></tr>
</table>

> Phantom uses the same upstream site database as Sherlock (automatically converted), so coverage is identical — the difference is speed, output quality, and zero install friction.

---

## 📦 Installation

### One Command (Recommended)

```bash
cargo install phantom-osint
```

### Download Binary

Download from [**Releases**](https://github.com/DBISHT4U/phantom/releases).

### Build from Source

```bash
git clone https://github.com/DBISHT4U/phantom.git
cd phantom && cargo build --release
./target/release/phantom --help
```

---

## ⌨️ Usage

<details>
<summary><b>🔍 Basic Scanning</b> — Hunt usernames across all 481 sites</summary>

```bash
# Single username
phantom john_doe

# Multiple usernames
phantom john_doe admin jane

# Only show found results
phantom john_doe --found-only

# Verbose mode (show misses and errors)
phantom john_doe --verbose
```
</details>

<details>
<summary><b>🎯 Category Filtering</b> — Scan specific platform types</summary>

```bash
# Only social media
phantom john_doe --category social

# Dev platforms + security sites
phantom john_doe --category dev,security

# Gaming only
phantom john_doe --category gaming

# List all categories
phantom --list-categories

# List all 481 supported sites
phantom --list-sites
```
</details>

<details>
<summary><b>📁 Export Results</b> — JSON and CSV output</summary>

```bash
# Export to JSON
phantom john_doe --output results.json

# Export to CSV
phantom john_doe --csv results.csv
```
</details>

<details>
<summary><b>🔒 Proxy & Tor</b> — Anonymous scanning</summary>

```bash
# Route through Tor
phantom john_doe --proxy socks5://127.0.0.1:9050

# HTTP proxy
phantom john_doe --proxy http://proxy.example.com:8080
```
</details>

<details>
<summary><b>⚙️ Advanced Options</b> — Tune performance</summary>

```bash
# Custom timeout (seconds)
phantom john_doe --timeout 10

# Max concurrent requests
phantom john_doe --workers 100

# Custom sites database
phantom john_doe --sites my-sites.json

# Plain text output (no colors)
phantom john_doe --plain
```
</details>

---

## 🔬 How It Works

```
                    ┌─────────────────┐
                    │  Username input  │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  Load 481 sites │
                    │  from database  │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
        ┌─────▼─────┐ ┌─────▼─────┐ ┌─────▼─────┐
        │  Worker 1  │ │  Worker 2  │ │ Worker N  │   50 concurrent
        │  (async)   │ │  (async)   │ │  (async)  │   by default
        └─────┬─────┘ └─────┬─────┘ └─────┬─────┘
              │              │              │
        ┌─────▼─────┐ ┌─────▼─────┐ ┌─────▼─────┐
        │  Verify:   │ │  Verify:   │ │  Verify:  │
        │ status_code│ │ body_match │ │body_absent│
        └─────┬─────┘ └─────┬─────┘ └─────┬─────┘
              │              │              │
              └──────────────┼──────────────┘
                             │
                    ┌────────▼────────┐
                    │  Real-time CLI  │───▶ JSON / CSV
                    │  output + table │
                    └─────────────────┘
```

---

## 🎯 Categories

| Category | Icon | Sites | Examples |
|:--|:--:|:--:|:--|
| **Social** | 👥 | 23 | Twitter/X, Instagram, LinkedIn, TikTok, Snapchat, Telegram |
| **Dev** | 💻 | 23 | GitHub, GitLab, StackOverflow, npm, PyPI, Docker Hub |
| **Gaming** | 🎮 | 16 | Steam, Chess.com, Roblox, Minecraft, Lichess |
| **Media** | 🎵 | 18 | YouTube, Twitch, Spotify, SoundCloud, Vimeo |
| **Forum** | 💬 | 7 | Reddit, Medium, Quora, Hacker News, Substack |
| **Creative** | 🎨 | 8 | Dribbble, Behance, DeviantArt, ArtStation |
| **Security** | 🔒 | 5 | HackerOne, Bugcrowd, TryHackMe, HackTheBox |
| **Money** | 💰 | 5 | Patreon, BuyMeACoffee, Ko-fi, Liberapay |
| **Other** | 🔗 | 376 | WordPress, Duolingo, Trello, Notion, and hundreds more |

---

## 🔧 Custom Sites

Create your own `sites.json`:

```json
[
  {
    "name": "Internal Wiki",
    "url": "https://wiki.company.com/users/{}",
    "check": "status",
    "category": "internal",
    "valid": 200,
    "invalid": 404
  }
]
```

```bash
phantom john_doe --sites my-sites.json
```

**Check types:** `status` (HTTP code) · `body_contains` (page has text) · `body_absent` (page doesn't have "not found" text)

---

## 🛡️ Use Cases

| Use Case | Description |
|:--|:--|
| 🔍 **OSINT investigations** | Map a target's digital footprint across the open web |
| 🧪 **Penetration testing** | Enumerate accounts during the reconnaissance phase |
| 🏢 **Brand monitoring** | Check if someone is impersonating your brand name |
| 👤 **Personal audit** | Discover your own forgotten accounts for cleanup |
| 🏁 **CTF competitions** | Solve OSINT challenges faster than anyone else |

---

## ⚠️ Responsible Use

Phantom is a **reconnaissance tool built for legitimate security research**.

- ✅ Use for authorized pentests, your own accounts, CTFs, brand monitoring
- ❌ Do not use for harassment, stalking, doxxing, or any illegal purpose
- ⚡ Respect rate limits — use `--timeout` and `--workers` responsibly
- 🔒 Route through Tor with `--proxy` for anonymity

---

## 🤝 Contributing

Contributions welcome! The easiest ways to help:

1. **Add sites** — submit a PR adding entries to `data/sites.json`
2. **Fix false positives** — improve detection for tricky sites
3. **Categorize** — help re-categorize the 376 "other" sites

---

## 📜 License

MIT — see [LICENSE](LICENSE)

---

<div align="center">

**Built with 🦀 Rust — because your recon shouldn't wait for Python**

<sub>⭐ Star if Phantom found something you didn't expect</sub>

<br>

<a href="https://github.com/DBISHT4U/phantom/stargazers"><img src="https://img.shields.io/github/stars/DBISHT4U/phantom?style=social" alt="Stars"></a>
<a href="https://github.com/DBISHT4U/phantom/network/members"><img src="https://img.shields.io/github/forks/DBISHT4U/phantom?style=social" alt="Forks"></a>

</div>
