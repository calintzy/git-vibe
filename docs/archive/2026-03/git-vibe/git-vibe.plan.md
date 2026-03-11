# Plan: git-vibe MVP 완성 및 v0.2 공유 기능

## Executive Summary

| 항목 | 내용 |
|------|------|
| Feature | git-vibe: Git 히스토리 기반 이모지 바이브 체크 CLI |
| 작성일 | 2026-03-11 |
| 작성자 | AI Assistant |
| 상태 | Draft |

### Value Delivered

| 관점 | 설명 |
|------|------|
| **Problem** | 새 프로젝트 합류 시 코드베이스 상태를 직관적으로 파악할 수 없고, 기존 도구는 딱딱한 통계만 제공하여 재미없고 공유 욕구가 없음 |
| **Solution** | Git 히스토리를 분석하여 이모지 기반 바이브 등급(Zen~Abandoned)과 핫스팟/유령파일/커밋감정/Bus Factor를 한눈에 보여주는 CLI 도구 |
| **Function UX Effect** | 한 줄 명령으로 프로젝트 건강 상태를 재미있는 이모지 리포트로 즉시 확인, SNS 공유 가능한 이미지 생성 |
| **Core Value** | "이 프로젝트 괜찮아?"에 대한 재미있고 직관적인 답변 → 바이럴 확산을 통한 개발자 도구 생태계 진입 |

---

## 1. 프로젝트 개요

### 1.1 배경
- **프로젝트명**: git-vibe (가칭, 이름 변경 필요 - GitHub에서 이미 사용 중)
- **한줄 소개**: Git 히스토리를 분석해 이모지 기반 "바이브 체크" 리포트를 생성하는 재미있는 CLI
- **카테고리**: Developer Tooling / Fun & Viral
- **라이선스**: MIT

### 1.2 문제 정의

| 핵심 고통점 | 설명 |
|-------------|------|
| 직관적 파악 불가 | "이 프로젝트 괜찮아?"에 대한 빠른 답이 없음 |
| 통계의 지루함 | 숫자 나열은 감정적으로 와닿지 않음 |
| 공유 불가 | 기존 도구 결과를 SNS에 올리고 싶지 않음 |
| 숨겨진 리스크 | 핫스팟, 유령 파일, 기술 부채가 보이지 않음 |

### 1.3 타겟 사용자

| 사용자 유형 | 사용 시나리오 |
|------------|-------------|
| 개발자 | 자기 프로젝트 바이브 체크 → SNS 공유 |
| 테크 인플루언서 | 유명 레포 바이브 비교 콘텐츠 |
| 채용 담당자 | 지원자 GitHub 레포 평가 |
| 오픈소스 선택자 | 라이브러리 선택 시 건강 확인 |

---

## 2. 현재 구현 상태 분석

### 2.1 MVP (v0.1) 기능 현황

| 기능 | 상태 | 구현 파일 | 비고 |
|------|------|----------|------|
| Git 히스토리 파서 (gix) | ✅ 완료 | `src/git/commits.rs` | gix 0.72, 커밋 수집 + 파일 변경 추적 |
| 핫스팟 감지 | ✅ 완료 | `src/git/hotspots.rs` | 커밋 수 기반, 상위 N개 파일 |
| 유령 파일 감지 | ✅ 완료 | `src/git/ghosts.rs` | 365일 미접촉 파일 |
| 커밋 감정 분석 | ✅ 완료 | `src/git/mood.rs` | 키워드 기반 4분류 (happy/stressed/cleanup/scary) |
| Bus Factor 계산 | ✅ 완료 | `src/git/authors.rs` | 80% 커버리지 기준 최소 저자 수 |
| 바이브 점수 계산 | ✅ 완료 | `src/scoring/mod.rs` | 6개 하위 점수 가중 합산 |
| 이모지 등급 시스템 | ✅ 완료 | `src/scoring/grades.rs` | 7단계 (Zen~Abandoned) |
| 컬러풀 터미널 출력 | ✅ 완료 | `src/output/terminal.rs` | colored + comfy-table |
| JSON 출력 | ✅ 완료 | `src/output/json.rs` | `--json` 플래그 |
| CLI 인터페이스 | ✅ 완료 | `src/cli.rs` | clap derive, path/period/json/leaderboard |
| 통합 테스트 | ✅ 완료 | `tests/cli_tests.rs` | 6개 테스트 (help, version, invalid path, run, json, period) |
| 단위 테스트 | ✅ 완료 | `mood.rs`, `authors.rs`, `grades.rs` | mood 6개, authors 5개, grades 12개 |

### 2.2 기술 스택 (현재)

| 구성 요소 | 기술 | 버전 |
|----------|------|------|
| 언어 | Rust | 2021 edition |
| Git 파싱 | gix (gitoxide) | 0.72 |
| CLI | clap | 4 (derive) |
| 터미널 출력 | colored + comfy-table | 3 / 7 |
| 시간 처리 | chrono | 0.4 |
| 직렬화 | serde + serde_json | 1 |
| 에러 처리 | thiserror | 2 |

### 2.3 아키텍처 구조

```
src/
├── main.rs          # 진입점: CLI 파싱 → 분석 → 점수 → 출력
├── lib.rs           # 모듈 공개
├── cli.rs           # clap CLI 정의
├── error.rs         # VibeError 에러 타입
├── git/
│   ├── mod.rs       # AnalysisResult + analyze() 오케스트레이션
│   ├── commits.rs   # 커밋 수집, 기간 파싱, 트리 순회
│   ├── hotspots.rs  # 핫스팟 탐지
│   ├── ghosts.rs    # 유령 파일 탐지
│   ├── mood.rs      # 커밋 감정 분석
│   └── authors.rs   # 저자 통계 + Bus Factor
├── scoring/
│   ├── mod.rs       # VibeScore 계산 (6개 하위 점수)
│   └── grades.rs    # VibeGrade 등급 (7단계)
└── output/
    ├── mod.rs       # render() 분기 (terminal/json)
    ├── terminal.rs  # 컬러풀 터미널 렌더링
    └── json.rs      # JSON 직렬화
```

---

## 3. 개선 필요 사항 (MVP 품질 향상)

### 3.1 기능적 개선

| ID | 항목 | 우선순위 | 설명 |
|----|------|---------|------|
| F-01 | 활동 추세 시각화 | HIGH | 기획서에 있는 월별 바 그래프가 터미널 출력에 누락. 현재 Rising/Stable/Declining 한 줄만 표시 |
| F-02 | Hall of Fame 이모지 칭호 | MEDIUM | 기획서에 있는 "👑 코드 여왕", "🦾 머신", "🌱 새싹" 같은 재미 요소 누락 |
| F-03 | 파일 변경 감지 정확도 | HIGH | 현재 트리 비교 방식은 수정된(동일 경로) 파일을 감지하지 못함. blob ID 비교 필요 |
| F-04 | 미분류 커밋 처리 | MEDIUM | classify_message에서 None 반환 시 비율이 100%에 미달. "기타" 카테고리 추가 필요 |
| F-05 | `--compare` 브랜치 비교 | LOW | v0.2 범위이나, CLI 옵션만 먼저 준비 |

### 3.2 품질/성능 개선

| ID | 항목 | 우선순위 | 설명 |
|----|------|---------|------|
| Q-01 | 대형 레포 성능 | HIGH | 수만 커밋 레포에서 트리 순회가 O(commits * files). 캐싱 또는 diff 기반으로 전환 필요 |
| Q-02 | 에러 메시지 개선 | MEDIUM | Git 에러가 원시 문자열로 전달. 사용자 친화적 메시지 필요 |
| Q-03 | 고스트 파일 정렬 | LOW | 현재 정렬 없이 반환. days_ago 기준 내림차순 정렬 필요 |

---

## 4. v0.2 기능 범위 (공유 기능)

기획서 v0.2 범위에서 우선순위를 선정합니다.

### 4.1 v0.2 핵심 기능

| ID | 기능 | 우선순위 | 설명 |
|----|------|---------|------|
| V2-01 | `--share` SVG/PNG 이미지 생성 | HIGH | 바이럴의 핵심. resvg로 결과를 시각적 이미지로 렌더링 |
| V2-02 | 활동 추세 월별 그래프 | HIGH | 터미널과 이미지 모두에 막대 그래프 표시 |
| V2-03 | GitHub Action | MEDIUM | PR에 바이브 코멘트 자동 추가 |
| V2-04 | README 배지 생성 | MEDIUM | `![Vibe: 😎 Chill](badge-url)` 형태 |

### 4.2 v0.2 범위 제외 (v0.3+)

- 웹 뷰어 (URL로 결과 공유) → 서버 인프라 필요, v0.3으로
- 브랜치/기간 비교 → v0.3
- 코드 복잡도 통합 → v0.3
- VS Code 확장 → v1.0

---

## 5. 마일스톤

### Phase A: MVP 품질 향상 (현재 → 안정화)

| 순서 | 작업 | 예상 규모 | 의존성 |
|------|------|----------|--------|
| A-1 | 파일 변경 감지 개선 (blob ID 비교) | 중 | 없음 |
| A-2 | 활동 추세 월별 시각화 추가 | 소 | 없음 |
| A-3 | Hall of Fame 이모지 칭호 추가 | 소 | 없음 |
| A-4 | 미분류 커밋 "기타" 카테고리 추가 | 소 | 없음 |
| A-5 | 고스트 파일 정렬 | 소 | 없음 |
| A-6 | 대형 레포 성능 최적화 | 대 | A-1 |

### Phase B: v0.2 공유 기능

| 순서 | 작업 | 예상 규모 | 의존성 |
|------|------|----------|--------|
| B-1 | SVG 템플릿 설계 | 중 | Phase A 완료 |
| B-2 | `--share` 옵션 + resvg 이미지 생성 | 대 | B-1 |
| B-3 | 활동 추세 그래프 (SVG 포함) | 중 | A-2, B-1 |
| B-4 | GitHub Action 제작 | 중 | B-2 |
| B-5 | README 배지 생성 | 소 | B-2 |

### Phase C: 출시 준비

| 순서 | 작업 | 예상 규모 | 의존성 |
|------|------|----------|--------|
| C-1 | 이름 확정 (git-vibe 충돌 해결) | 소 | 없음 |
| C-2 | README 작성 + 스크린샷 | 중 | Phase B 완료 |
| C-3 | `cargo publish` 준비 | 소 | C-1 |
| C-4 | Homebrew formula 작성 | 소 | C-3 |

---

## 6. 기술적 결정 사항

### 6.1 확정 사항

| 결정 | 선택 | 이유 |
|------|------|------|
| 언어 | Rust | 이미 구현됨. 빠른 Git 분석 + 단일 바이너리 |
| Git 라이브러리 | gix 0.72 | Rust 네이티브, libgit2보다 빠름 |
| CLI | clap 4 (derive) | Rust 표준, 이미 구현됨 |
| 에러 처리 | thiserror | 이미 적용됨 |

### 6.2 결정 필요 사항

| 항목 | 옵션 | 추천 | 이유 |
|------|------|------|------|
| 이미지 생성 | resvg vs plotters | resvg | SVG→PNG 변환에 특화, 커스텀 레이아웃 자유도 |
| 이름 변경 | repo-vibe / vibecheck / code-aura / git-mood | vibecheck | 짧고 직관적, 사용 가능 여부 확인 필요 |
| 배포 채널 | cargo only vs cargo+brew+npm | cargo+brew | Rust 사용자 + macOS 사용자 커버 |

---

## 7. 리스크

| 리스크 | 확률 | 영향 | 대응 |
|--------|------|------|------|
| 대형 레포 성능 | 중 | 높음 | gix diff 기반 전환, `--period` 기본값 유지 |
| 이름 충돌 | 확정 | 중 | crates.io 사용 가능 이름 사전 확인 |
| resvg 의존성 크기 | 중 | 낮음 | feature flag로 `--share` 기능 선택적 컴파일 |
| "장난감" 인식 | 중 | 높음 | CI 통합(GitHub Action)으로 실용성 강조 |

---

## 8. 성공 지표

| 지표 | MVP 목표 | v0.2 목표 |
|------|---------|----------|
| `cargo test` 통과율 | 100% | 100% |
| 대형 레포(10k+ commits) 실행 시간 | < 30초 | < 30초 |
| 기획서 MVP 기능 구현율 | 100% | - |
| `--share` 이미지 생성 | - | 동작 확인 |
| GitHub Action 동작 | - | PR 코멘트 성공 |

---

## 9. 즉시 실행 가능한 다음 단계

현재 MVP가 기능적으로 완성되어 있으므로, 추천하는 우선 작업:

1. **F-03 (파일 변경 감지 개선)** — 분석 정확도의 핵심
2. **F-01 (활동 추세 시각화)** — 기획서 핵심 출력에 포함
3. **F-02 (Hall of Fame 칭호)** — 바이럴 재미 요소
4. **Q-01 (성능 최적화)** — 실사용 가능성 확보

> **추천**: `/pdca design git-vibe`로 Design 문서를 작성하여 구체적인 구현 설계를 진행하세요.
