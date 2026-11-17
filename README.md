<div align="center">

```
██████╗ ██╗  ██╗ █████╗ ███╗   ██╗████████╗ ██████╗ ███╗   ███╗
██╔══██╗██║  ██║██╔══██╗████╗  ██║╚══██╔══╝██╔═══██╗████╗ ████║
██████╔╝███████║███████║██╔██╗ ██║   ██║   ██║   ██║██╔████╔██║
██╔═══╝ ██╔══██║██╔══██║██║╚██╗██║   ██║   ██║   ██║██║╚██╔╝██║
██║     ██║  ██║██║  ██║██║ ╚████║   ██║   ╚██████╔╝██║ ╚═╝ ██║
╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚═╝     ╚═╝
```

<h3>👻 Hunt usernames across the internet — at the speed of Rust</h3>

<p>
  <strong>Find anyone's digital footprint across 481 sites in seconds.</strong><br>
  The fastest username OSINT tool ever built.
</p>

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg?logo=rust" alt="Rust"></a>
  <a href="https://crates.io/crates/phantom-osint"><img src="https://img.shields.io/crates/v/phantom-osint.svg" alt="Crates.io"></a>
  <img src="https://img.shields.io/badge/Sites-481-brightgreen.svg" alt="Sites">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg" alt="Platform">
</p>

</div>

---


  <img src="demo.gif" alt="Phantom Demo" width="800">

---

<div align="center">
  <img src="demo.gif" alt="Phantom Demo" width="800">
</div>

---

## ⚡ Why Phantom?

> **Phantom is the fastest username reconnaissance tool ever built.** It scans 481 sites in seconds — not minutes. Zero dependencies, single binary, real-time output. Nothing else comes close.


| | Sherlock (Python) | GoSearch (Go) | **Phantom (Rust)** |
|---|---|---|---|
| **Speed** | ~45s for 400 sites | ~8s | **~2-3s** |
| **Install** | pip + python + deps | go install | **Single binary** |
| **Concurrency** | Threading (GIL) | Goroutines | **Tokio async (zero-cost)** |
| **Output** | Plain text | Text/JSON | **Color table + JSON + CSV** |
| **Live results** | After completion | After completion | **Real-time as found** |
| **Binary size** | ~50MB (with Python) | ~8MB | **~4MB** |

## 🚀 Install

```bash
cargo install phantom-osint
```

Or build from source:

```bash
git clone https://github.com/DBISHT4U/phantom.git
cd phantom && cargo build --release
./target/release/phantom --help
```

## 📖 Usage

```bash
# Hunt a username across all sites
phantom john_doe

# Hunt multiple usernames
phantom john_doe jane_doe admin

# Filter by category
phantom john_doe --category social,dev

# Export results
phantom john_doe --output results.json
phantom john_doe --csv results.csv

# Use with Tor/proxy
phantom john_doe --proxy socks5://127.0.0.1:9050

# Custom timeout and workers
phantom john_doe --timeout 10 --workers 100

# Show only found results
phantom john_doe --found-only

# List all supported sites
phantom --list-sites

# List categories
phantom --list-categories

# Use custom sites database
phantom john_doe --sites my-sites.json
```

## 🎯 Categories

| Category | Icon | Sites | Examples |
|----------|------|-------|---------|
| **Social** | 👥 | 10+ | Twitter/X, Instagram, TikTok, LinkedIn, Snapchat |
| **Dev** | 💻 | 15+ | GitHub, GitLab, StackOverflow, npm, crates.io |
| **Gaming** | 🎮 | 6+ | Steam, Chess.com, Roblox, Minecraft, Itch.io |
| **Media** | 🎵 | 10+ | YouTube, Twitch, Spotify, SoundCloud |
| **Forum** | 💬 | 6+ | Reddit, Medium, Hacker News, Quora |
| **Creative** | 🎨 | 6+ | Dribbble, Behance, DeviantArt, ArtStation |
| **Security** | 🔒 | 4+ | HackerOne, Bugcrowd, TryHackMe, HackTheBox |

## 🔧 Custom Sites

Create your own `sites.json`:

```json
[
  {
    "name": "MySite",
    "url": "https://example.com/users/{}",
    "check": "status",
    "category": "custom",
    "valid": 200,
    "invalid": 404
  }
]
```

Check types: `status` (HTTP code), `body_contains` (page has text), `body_absent` (page doesn't have "not found" text).

```bash
phantom john_doe --sites my-sites.json
```

## ⚠️ Legal & Ethics

- **Only use for legitimate OSINT research, security assessments, or finding your own accounts**
- **Do not use for harassment, stalking, or any illegal purpose**
- Respect rate limits and terms of service of target sites
- Some sites may block rapid scanning — use `--timeout` and `--workers` to be respectful

## 📜 License

MIT

---

<div align="center">
<b>Built with 🦀 Rust — because your OSINT shouldn't wait for Python</b>

<sub>⭐ Star this if Phantom found something interesting!</sub>
</div>
