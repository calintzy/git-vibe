# 🎭 git-vibe

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/git-vibe.svg)](https://crates.io/crates/git-vibe)
[![npm](https://img.shields.io/npm/v/@git-vibe/cli.svg)](https://www.npmjs.com/package/@git-vibe/cli)

> 检查你的代码库氛围

**[English](README.md) | [한국어](README.ko.md) | [中文](README.zh-CN.md)**

---

## 为什么选择 git-vibe？

加入新项目或选择开源库时，第一个问题总是 **"这个代码库健康吗？"** — 但没有快速、直观的答案。

- **没有直观的健康检查。** 不翻阅几个月的 Git 日志，你无法快速回答"这个项目还好吗？"。
- **现有工具太无聊。** `gitinspector`、`git-stats`、`git-fame` 只会用纯文本输出一堆数字 — 没人想分享这些。
- **隐藏的风险一直隐藏。** 变更过于频繁的热点文件、无人维护的幽灵文件、一个开发者掌握所有知识的巴士因子风险 — 这些直到为时已晚才会暴露。
- **数据不会讲故事。** 一堆数字无法传达项目的*氛围*。

**git-vibe 将 Git 历史转化为有趣的、可分享的氛围报告，配有你真正*想要*发布的 emoji 等级。**

一个命令。一个分数。即时洞察。

### 谁应该使用？

| 对象 | 使用场景 |
|------|----------|
| **开发者** | 检查自己项目的氛围 — "我的仓库是 😎 Chill！" |
| **团队负责人** | 在事故发生前发现热点和巴士因子风险 |
| **开源评估者** | 添加依赖前快速评估库的健康状况 |
| **技术内容创作者** | 比较知名仓库 — "React vs Vue 氛围对决" |
| **招聘经理** | 快速了解候选人的项目质量 |

---

## 演示

```
🎭 Vibe Check: my-project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Overall Vibe: 💪 Active (74/100)
📊 Period: 12m | Commits: 142 | Files: 38

🔥 热点文件（变更最频繁的文件）
╔══════════════════════════╦═════════╦══════╗
║ File                     ║ Changes ║ Heat ║
╠══════════════════════════╬═════════╬══════╣
║ src/api/auth.rs          ║ 34      ║ 🔥   ║
║ src/db/migrations.rs     ║ 21      ║ ♨️   ║
╚══════════════════════════╩═════════╩══════╝

👻 幽灵文件（超过1年未变更）
  docs/legacy-api.md（412天前）

🎭 提交情绪
  Happy 😊          45.1% ████████░░░░░░░░░░░░
  Stressed 😰       22.5% ████░░░░░░░░░░░░░░░░
  Cleanup 🧹        18.3% ███░░░░░░░░░░░░░░░░░
  Scary 💀           5.6% █░░░░░░░░░░░░░░░░░░░

🏋️ 巴士因子
  3 👍 健康

📈 活动趋势
  📈 上升中
```

---

## 安装

### npm（推荐）

```bash
npm install -g @git-vibe/cli
```

### Cargo

```bash
cargo install git-vibe
```

包含 PNG 支持：

```bash
cargo install git-vibe --features png
```

### 从源码构建

```bash
git clone https://github.com/calintzy/git-vibe
cd git-vibe
cargo build --release
```

---

## 使用方法

### 基本使用

```bash
# 分析当前目录
git-vibe

# 分析指定仓库
git-vibe --path /path/to/repo

# 分析最近3个月
git-vibe --period 3m

# JSON 输出
git-vibe --json

# 显示完整贡献者排行榜
git-vibe --leaderboard
```

### 分享和徽章 (v0.2)

```bash
# 生成可分享的 SVG 报告图片（800x480，暗色主题）
git-vibe --share

# 保存到自定义路径
git-vibe --share -o my-report.svg

# 生成 PNG 报告（需要 --features png）
git-vibe --share --format png

# 生成 shields.io 风格的 README 徽章
git-vibe --badge
git-vibe --badge -o badge.svg
```

**徽章示例：**

![vibe](https://img.shields.io/badge/vibe-😎_Chill_(82)-97CA00)

### GitHub Action

在 CI 流水线中添加氛围检查：

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

自动在 PR 中发布氛围报告评论。

### 选项

| 标志 | 简写 | 默认值 | 说明 |
|------|------|--------|------|
| `--path` | `-p` | `.` | Git 仓库路径 |
| `--period` | `-P` | `12m` | 分析期间（`3m`、`6m`、`12m`、`1y`、`30d` 等） |
| `--json` | | off | 以 JSON 格式输出 |
| `--leaderboard` | | off | 显示所有贡献者（不仅前5名） |
| `--share` | | off | 生成可分享的 SVG/PNG 报告图片 |
| `--badge` | | off | 生成 shields.io 风格徽章 SVG |
| `--format` | | `svg` | `--share` 的输出格式（`svg` 或 `png`） |
| `--output` | `-o` | 自动 | 自定义输出文件路径 |

### 期间格式

| 格式 | 含义 |
|------|------|
| `7d` | 最近7天 |
| `4w` | 最近4周 |
| `3m` | 最近3个月 |
| `6m` | 最近6个月 |
| `12m` / `1y` | 最近12个月 |

---

## 评分

氛围分数（0–100）由6个指标计算：

| 指标 | 权重 | 说明 |
|------|------|------|
| Commit Frequency | 20% | 每周提交频率 |
| Hotspot Score | 20% | 频繁变更文件比例 |
| Ghost Score | 15% | 超过1年未变更文件比例 |
| Mood Score | 20% | 提交信息情绪分析 |
| Bus Factor | 15% | 覆盖80%变更的最少作者数 |
| Trend Score | 10% | 近期与过去提交活动对比 |

## 等级

| 等级 | 分数 | Emoji | 含义 |
|------|------|-------|------|
| Zen | 90–100 | 🧘 | 巅峰健康 — 一帆风顺 |
| Chill | 80–89 | 😎 | 健康且可持续 |
| Active | 70–79 | 💪 | 良好的势头 |
| Tense | 60–69 | 😬 | 有些压力在积累 |
| Stressed | 50–59 | 😰 | 需要关注 |
| Chaotic | 40–49 | 🔥 | 情况变得混乱 |
| Abandoned | 0–39 | 💀 | 严重问题 |

---

## 许可证

MIT
