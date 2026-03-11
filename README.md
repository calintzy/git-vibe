# 🎭 git-vibe

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/git-vibe.svg)](https://crates.io/crates/git-vibe)
[![npm](https://img.shields.io/npm/v/git-vibe.svg)](https://www.npmjs.com/package/git-vibe)

> Vibe check your codebase

**[English](README.md) | [한국어](README.ko.md) | [中文](README.zh-CN.md)**

---

## Why git-vibe?

Joining a new project or picking an open-source library? The first question is always **"Is this codebase healthy?"** — but there's no quick, intuitive answer.

- **No intuitive health check.** You can't quickly answer "Is this project in good shape?" without digging through months of Git logs.
- **Existing tools are boring.** `gitinspector`, `git-stats`, `git-fame` dump raw numbers in plain text — nobody wants to share that.
- **Hidden risks stay hidden.** Hotspot files that change too often, ghost files nobody maintains, bus-factor risks where one dev holds all the knowledge — these don't surface until it's too late.
- **Stats don't tell a story.** A wall of numbers doesn't convey the *feel* of a project.

**git-vibe turns Git history into a fun, shareable vibe report with emoji grades you actually *want* to post.**

One command. One score. Instant insight.

### Who Should Use It?

| Who | Use Case |
|-----|----------|
| **Developers** | Vibe-check your own project — "My repo is 😎 Chill!" |
| **Team Leads** | Spot hotspots and bus-factor risks before they become incidents |
| **Open-source evaluators** | Quickly assess library health before adding a dependency |
| **Tech content creators** | Compare famous repos — "React vs Vue vibe battle" |
| **Hiring managers** | Get a quick signal on a candidate's project quality |

---

## Demo

```
🎭 Vibe Check: my-project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Overall Vibe: 💪 Active (74/100)
📊 Period: 12m | Commits: 142 | Files: 38

🔥 Hotspots (most changed files)
╔══════════════════════════╦═════════╦══════╗
║ File                     ║ Changes ║ Heat ║
╠══════════════════════════╬═════════╬══════╣
║ src/api/auth.rs          ║ 34      ║ 🔥   ║
║ src/db/migrations.rs     ║ 21      ║ ♨️   ║
╚══════════════════════════╩═════════╩══════╝

👻 Ghost Files (untouched > 1 year)
  docs/legacy-api.md (412 days ago)

🎭 Commit Mood
  Happy 😊          45.1% ████████░░░░░░░░░░░░
  Stressed 😰       22.5% ████░░░░░░░░░░░░░░░░
  Cleanup 🧹        18.3% ███░░░░░░░░░░░░░░░░░
  Scary 💀           5.6% █░░░░░░░░░░░░░░░░░░░

🏋️ Bus Factor
  3 👍 Healthy

📈 Activity Trend
  📈 Rising

🏆 Hall of Fame
╔══════════╦═════════╦═════════╦═══════════╗
║ Author   ║ Commits ║ Added   ║ Removed   ║
╠══════════╬═════════╬═════════╬═══════════╣
║ alice    ║ 89      ║ 4201    ║ 1832      ║
║ bob      ║ 53      ║ 2100    ║ 980       ║
╚══════════╩═════════╩═════════╩═══════════╝
```

---

## Installation

### npm (recommended)

```bash
npm install -g git-vibe
```

### Cargo

```bash
cargo install git-vibe
```

With PNG support:

```bash
cargo install git-vibe --features png
```

### Build from source

```bash
git clone https://github.com/calintzy/git-vibe
cd git-vibe
cargo build --release
```

---

## Usage

### Basic

```bash
# Analyze current directory
git-vibe

# Analyze a specific repository
git-vibe --path /path/to/repo

# Analyze the last 3 months
git-vibe --period 3m

# Output as JSON
git-vibe --json

# Show full contributor leaderboard
git-vibe --leaderboard
```

### Share & Badge (v0.2)

```bash
# Generate a shareable SVG report image (800x480, dark theme)
git-vibe --share

# Save to a custom path
git-vibe --share -o my-report.svg

# Generate a PNG report (requires --features png)
git-vibe --share --format png

# Generate a shields.io-style badge for your README
git-vibe --badge
git-vibe --badge -o badge.svg
```

**Badge example:**

![vibe](https://img.shields.io/badge/vibe-😎_Chill_(82)-97CA00)

### GitHub Action

Add a vibe check to your CI pipeline:

```yaml
# .github/workflows/vibe.yml
name: Vibe Check
on: [pull_request]

jobs:
  vibe:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: ./.github/actions/git-vibe
        with:
          period: '6m'
          post-comment: 'true'
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

This posts a vibe report as a PR comment automatically.

### Options

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--path` | `-p` | `.` | Path to the Git repository |
| `--period` | `-P` | `12m` | Analysis period (`3m`, `6m`, `12m`, `1y`, `30d`, etc.) |
| `--json` | | off | Output results as JSON |
| `--leaderboard` | | off | Show all contributors (not just top 5) |
| `--share` | | off | Generate shareable SVG/PNG report image |
| `--badge` | | off | Generate shields.io-style badge SVG |
| `--format` | | `svg` | Output format for `--share` (`svg` or `png`) |
| `--output` | `-o` | auto | Custom output file path |

### Period Format

| Format | Meaning |
|--------|---------|
| `7d` | Last 7 days |
| `4w` | Last 4 weeks |
| `3m` | Last 3 months |
| `6m` | Last 6 months |
| `12m` / `1y` | Last 12 months |

---

## Scoring

The vibe score (0–100) is calculated from 6 metrics:

| Metric | Weight | Description |
|--------|--------|-------------|
| Commit Frequency | 20% | How often commits land per week |
| Hotspot Score | 20% | Ratio of frequently-changed files |
| Ghost Score | 15% | Ratio of files untouched for 1+ year |
| Mood Score | 20% | Weighted sentiment from commit messages |
| Bus Factor | 15% | Minimum authors covering 80% of changes |
| Trend Score | 10% | Comparing recent vs. older commit activity |

## Grades

| Grade | Score | Emoji | Meaning |
|-------|-------|-------|---------|
| Zen | 90–100 | 🧘 | Peak health — smooth sailing |
| Chill | 80–89 | 😎 | Healthy and sustainable |
| Active | 70–79 | 💪 | Good momentum |
| Tense | 60–69 | 😬 | Some pressure building up |
| Stressed | 50–59 | 😰 | Needs attention |
| Chaotic | 40–49 | 🔥 | Things are getting messy |
| Abandoned | 0–39 | 💀 | Significant concerns |

---

## License

MIT
