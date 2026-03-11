# 🎭 git-vibe

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/git-vibe.svg)](https://crates.io/crates/git-vibe)

> 코드베이스의 바이브를 체크하세요

**[English](README.md) | [한국어](README.ko.md) | [中文](README.zh-CN.md)**

---

## 왜 git-vibe인가?

새 프로젝트에 합류하거나 오픈소스 라이브러리를 고를 때, 첫 번째 질문은 항상 **"이 코드베이스는 건강한가?"** 입니다. 하지만 빠르고 직관적인 답을 얻을 방법이 없습니다.

- **직관적인 건강 체크가 없습니다.** 수개월치 Git 로그를 뒤지지 않고는 "이 프로젝트 괜찮아?"라는 질문에 빠르게 답할 수 없습니다.
- **기존 도구들은 지루합니다.** `gitinspector`, `git-stats`, `git-fame`은 밋밋한 텍스트로 숫자만 나열합니다 — 누가 이걸 공유하고 싶겠습니까?
- **숨겨진 리스크는 계속 숨어 있습니다.** 너무 자주 변경되는 핫스팟 파일, 아무도 관리하지 않는 유령 파일, 한 명의 개발자가 모든 지식을 가진 버스 팩터 리스크 — 이런 것들은 너무 늦을 때까지 드러나지 않습니다.
- **숫자는 스토리를 말해주지 않습니다.** 숫자의 벽은 프로젝트의 *느낌*을 전달하지 못합니다.

**git-vibe는 Git 히스토리를 이모지 등급이 포함된 재미있고 공유하고 싶은 바이브 리포트로 변환합니다.**

하나의 명령어. 하나의 점수. 즉각적인 인사이트.

### 누가 사용해야 하나요?

| 대상 | 활용 사례 |
|------|----------|
| **개발자** | 내 프로젝트 바이브 체크 — "내 레포는 😎 Chill!" |
| **팀 리드** | 핫스팟과 버스 팩터 리스크를 사고 전에 발견 |
| **오픈소스 평가자** | 의존성 추가 전 라이브러리 건강 상태 빠른 평가 |
| **테크 콘텐츠 크리에이터** | 유명 레포 비교 — "React vs Vue 바이브 배틀" |
| **채용 담당자** | 지원자의 프로젝트 품질에 대한 빠른 시그널 확인 |

---

## 데모

```
🎭 Vibe Check: my-project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Overall Vibe: 💪 Active (74/100)
📊 Period: 12m | Commits: 142 | Files: 38

🔥 Hotspots (가장 많이 변경된 파일)
╔══════════════════════════╦═════════╦══════╗
║ File                     ║ Changes ║ Heat ║
╠══════════════════════════╬═════════╬══════╣
║ src/api/auth.rs          ║ 34      ║ 🔥   ║
║ src/db/migrations.rs     ║ 21      ║ ♨️   ║
╚══════════════════════════╩═════════╩══════╝

👻 Ghost Files (1년 이상 미변경)
  docs/legacy-api.md (412일 전)

🎭 커밋 감정
  Happy 😊          45.1% ████████░░░░░░░░░░░░
  Stressed 😰       22.5% ████░░░░░░░░░░░░░░░░
  Cleanup 🧹        18.3% ███░░░░░░░░░░░░░░░░░
  Scary 💀           5.6% █░░░░░░░░░░░░░░░░░░░

🏋️ Bus Factor
  3 👍 건강함

📈 활동 추세
  📈 상승 중
```

---

## 설치

```bash
cargo install git-vibe
```

PNG 지원 포함:

```bash
cargo install git-vibe --features png
```

소스에서 빌드:

```bash
git clone https://github.com/calintzy/git-vibe
cd git-vibe
cargo build --release
```

---

## 사용법

### 기본 사용

```bash
# 현재 디렉토리 분석
git-vibe

# 특정 레포지토리 분석
git-vibe --path /path/to/repo

# 최근 3개월 분석
git-vibe --period 3m

# JSON 출력
git-vibe --json

# 전체 기여자 순위 표시
git-vibe --leaderboard
```

### 공유 & 배지 (v0.2)

```bash
# 공유용 SVG 리포트 이미지 생성 (800x480, 다크 테마)
git-vibe --share

# 커스텀 경로에 저장
git-vibe --share -o my-report.svg

# PNG 리포트 생성 (--features png 필요)
git-vibe --share --format png

# README용 shields.io 스타일 배지 생성
git-vibe --badge
git-vibe --badge -o badge.svg
```

**배지 예시:**

![vibe](https://img.shields.io/badge/vibe-😎_Chill_(82)-97CA00)

### GitHub Action

CI 파이프라인에 바이브 체크를 추가하세요:

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

PR에 자동으로 바이브 리포트 코멘트가 게시됩니다.

### 옵션

| 플래그 | 단축 | 기본값 | 설명 |
|--------|------|--------|------|
| `--path` | `-p` | `.` | Git 레포지토리 경로 |
| `--period` | `-P` | `12m` | 분석 기간 (`3m`, `6m`, `12m`, `1y`, `30d` 등) |
| `--json` | | off | JSON 형식으로 출력 |
| `--leaderboard` | | off | 전체 기여자 표시 (상위 5명이 아닌) |
| `--share` | | off | 공유용 SVG/PNG 리포트 이미지 생성 |
| `--badge` | | off | shields.io 스타일 배지 SVG 생성 |
| `--format` | | `svg` | `--share`의 출력 형식 (`svg` 또는 `png`) |
| `--output` | `-o` | 자동 | 커스텀 출력 파일 경로 |

### 기간 형식

| 형식 | 의미 |
|------|------|
| `7d` | 최근 7일 |
| `4w` | 최근 4주 |
| `3m` | 최근 3개월 |
| `6m` | 최근 6개월 |
| `12m` / `1y` | 최근 12개월 |

---

## 점수 산정

바이브 점수(0–100)는 6개 지표로 계산됩니다:

| 지표 | 가중치 | 설명 |
|------|--------|------|
| Commit Frequency | 20% | 주당 커밋 빈도 |
| Hotspot Score | 20% | 자주 변경되는 파일 비율 |
| Ghost Score | 15% | 1년 이상 미변경 파일 비율 |
| Mood Score | 20% | 커밋 메시지 감정 분석 |
| Bus Factor | 15% | 변경의 80%를 커버하는 최소 작성자 수 |
| Trend Score | 10% | 최근 vs 과거 커밋 활동 비교 |

## 등급

| 등급 | 점수 | 이모지 | 의미 |
|------|------|--------|------|
| Zen | 90–100 | 🧘 | 최고 건강 상태 — 순항 중 |
| Chill | 80–89 | 😎 | 건강하고 지속 가능 |
| Active | 70–79 | 💪 | 좋은 모멘텀 |
| Tense | 60–69 | 😬 | 약간의 압박 축적 |
| Stressed | 50–59 | 😰 | 주의 필요 |
| Chaotic | 40–49 | 🔥 | 상황이 복잡해지는 중 |
| Abandoned | 0–39 | 💀 | 심각한 우려 |

---

## 라이선스

MIT
