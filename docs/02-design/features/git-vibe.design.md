# Design: git-vibe MVP 품질 향상

## Executive Summary

| 항목 | 내용 |
|------|------|
| Feature | git-vibe MVP 품질 향상 (Phase A) |
| Plan 참조 | `docs/01-plan/features/git-vibe.plan.md` |
| 작성일 | 2026-03-11 |
| 상태 | Draft |
| 범위 | Plan Phase A (F-01 ~ F-04, Q-01 ~ Q-03) |

---

## 1. 설계 범위

Plan 문서의 **Phase A: MVP 품질 향상** 항목에 대한 구현 설계입니다.

| ID | 항목 | 변경 파일 |
|----|------|----------|
| F-01 | 활동 추세 월별 시각화 | `src/git/mod.rs`, `src/output/terminal.rs`, `src/output/json.rs` |
| F-02 | Hall of Fame 이모지 칭호 | `src/output/terminal.rs` |
| F-03 | 파일 변경 감지 정확도 (blob ID 비교) | `src/git/commits.rs` |
| F-04 | 미분류 커밋 "기타" 카테고리 | `src/git/mood.rs`, `src/output/terminal.rs`, `src/output/json.rs` |
| Q-01 | 대형 레포 성능 최적화 | `src/git/commits.rs` |
| Q-02 | 에러 메시지 개선 | `src/error.rs` |
| Q-03 | 고스트 파일 정렬 | `src/git/ghosts.rs` |

---

## 2. F-03: 파일 변경 감지 정확도 개선

### 2.1 현재 문제

`src/git/commits.rs`의 `collect_file_changes_by_id()` 함수가 트리 간 **경로 집합의 차이(difference)** 만 비교합니다. 같은 경로에 있지만 **내용이 변경된 파일**을 감지하지 못합니다.

```rust
// 현재: 추가/삭제만 감지, 수정 감지 불가
for f in current_set.difference(&parent_set) { ... } // 추가된 파일
for f in parent_set.difference(&current_set) { ... } // 삭제된 파일
// 교집합(수정된 파일) 누락!
```

### 2.2 설계

`collect_file_changes_by_id()` 를 수정하여 **blob OID 비교** 방식으로 전환합니다.

**변경 대상**: `src/git/commits.rs`

**핵심 변경**:

1. `collect_tree_files()` 반환 타입을 `Vec<String>` → `Vec<(String, gix::ObjectId)>` (경로 + blob ID)로 변경
2. 부모 트리와 현재 트리에서 같은 경로의 blob ID가 다르면 "수정"으로 분류

```rust
// 변경 후 의사 코드
fn collect_tree_files_with_oids(
    repo: &gix::Repository,
    tree: &gix::Tree,
    prefix: &str,
    files: &mut Vec<(String, gix::ObjectId)>,  // (경로, blob OID)
) -> Result<(), VibeError>

fn collect_file_changes_by_id(...) -> Result<Vec<FileChange>, VibeError> {
    // 1. 현재 트리: HashMap<경로, OID>
    // 2. 부모 트리: HashMap<경로, OID>
    // 3. 비교:
    //    - 현재에만 있음 → 추가 (additions: 1)
    //    - 부모에만 있음 → 삭제 (deletions: 1)
    //    - 양쪽 모두 있지만 OID 다름 → 수정 (additions: 1, deletions: 1)
}
```

### 2.3 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/git/commits.rs` | `collect_tree_files()` 시그니처 변경, `collect_file_changes_by_id()` 로직 수정 |
| `src/git/commits.rs` | `list_all_files()` — 기존 `Vec<String>` 반환 유지 (내부에서 OID 무시) |

### 2.4 테스트

- 기존 통합 테스트 통과 확인
- 수정된 파일이 hotspot에 정상 카운트되는지 검증 (수동: 실제 레포에서 실행)

---

## 3. F-01: 활동 추세 월별 시각화

### 3.1 현재 상태

`src/scoring/mod.rs`의 `calc_trend_score()`가 3등분 비율만 계산하고, `src/output/terminal.rs`에서 `Rising/Stable/Declining` 한 줄로만 출력합니다. 기획서의 월별 바 그래프가 없습니다.

### 3.2 설계

**새 구조체**: `AnalysisResult`에 월별 활동 데이터를 추가합니다.

**변경 대상**: `src/git/mod.rs`, `src/output/terminal.rs`, `src/output/json.rs`

```rust
// src/git/mod.rs에 추가
#[derive(Debug, Clone, Serialize)]
pub struct MonthlyActivity {
    pub label: String,      // "2026-01", "2026-02" 등
    pub commit_count: u32,
}

// AnalysisResult에 필드 추가
pub struct AnalysisResult {
    // ... 기존 필드 ...
    pub monthly_activity: Vec<MonthlyActivity>,
}
```

**월별 집계 함수**: `src/git/mod.rs`의 `analyze()` 내부에서 커밋 타임스탬프로 월별 그룹핑

```rust
fn calc_monthly_activity(commits: &[CommitData]) -> Vec<MonthlyActivity> {
    // 1. 커밋을 "YYYY-MM" 키로 그룹핑 (HashMap<String, u32>)
    // 2. 키 기준 정렬 (오래된 순)
    // 3. Vec<MonthlyActivity>로 변환
}
```

**터미널 출력** (`src/output/terminal.rs`):

```
📈 Activity Trend
  2026-01: ████████████ 45
  2026-02: ████████     32
  2026-03: ███          12 📉
```

- 최대 커밋 수 기준으로 바 길이 정규화 (최대 20칸)
- 마지막 월이 이전 월 대비 30% 이상 감소 시 📉, 30% 이상 증가 시 📈 표시

**JSON 출력** (`src/output/json.rs`):

```json
{
  "monthly_activity": [
    { "month": "2026-01", "commits": 45 },
    { "month": "2026-02", "commits": 32 }
  ]
}
```

### 3.3 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/git/mod.rs` | `MonthlyActivity` 구조체 추가, `analyze()`에서 월별 집계 |
| `src/output/terminal.rs` | `render_terminal()`에서 월별 바 그래프 출력 (기존 1줄 → 그래프) |
| `src/output/json.rs` | `JsonReport`에 `monthly_activity` 필드 추가 |

---

## 4. F-02: Hall of Fame 이모지 칭호

### 4.1 설계

기획서의 재미 요소를 추가합니다. 기여 순위에 따라 이모지 칭호를 부여합니다.

**변경 대상**: `src/output/terminal.rs`

**칭호 규칙** (순위 기반):

| 순위 | 칭호 | 기준 |
|------|------|------|
| 1위 | 👑 | 최다 기여자 |
| 2위 | 🦾 | 2위 기여자 |
| 3위 | 🥉 | 3위 기여자 |
| 4위~ | 🌱 | 나머지 |

**구현**: `render_terminal()`의 Hall of Fame 테이블에 칭호 컬럼 추가

```rust
let badge = match rank {
    0 => "👑",
    1 => "🦾",
    2 => "🥉",
    _ => "🌱",
};
// 테이블 헤더: ["", "Author", "Commits", "Added", "Removed"]
// 첫 번째 컬럼에 badge 삽입
```

### 4.2 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/output/terminal.rs` | Hall of Fame 테이블에 칭호 컬럼 추가 |

---

## 5. F-04: 미분류 커밋 "기타" 카테고리

### 5.1 현재 문제

`src/git/mood.rs`의 `classify_message()`가 분류하지 못하면 `None`을 반환합니다. 그 결과 비율의 합이 100%에 미달합니다. (예: happy 25% + stressed 50% = 75%, 나머지 25% 표시 안 됨)

### 5.2 설계

**변경 대상**: `src/git/mood.rs`, `src/output/terminal.rs`, `src/output/json.rs`

`MoodStats`에 `other` 필드를 추가합니다.

```rust
// src/git/mood.rs
#[derive(Debug, Clone, Serialize)]
pub struct MoodStats {
    pub happy: f64,
    pub stressed: f64,
    pub cleanup: f64,
    pub scary: f64,
    pub other: f64,       // 추가
    pub total_commits: u32,
}

// analyze_mood() 수정
pub fn analyze_mood(commits: &[CommitData]) -> MoodStats {
    // ... 기존 카운트 ...
    let classified = happy + stressed + cleanup + scary;
    let other = total - classified;
    // ...
    MoodStats {
        // ... 기존 ...
        other: other as f64 / t,
    }
}
```

**터미널 출력**: 기존 4줄 아래에 추가

```
  Other 🤷       25.0% ██████░░░░░░░░░░░░░░
```

**JSON 출력**: `MoodJson`에 `other_pct` 필드 추가

### 5.3 테스트 수정

기존 `test_feat_is_happy` 등에서 `other` 필드 검증 추가. `test_mixed_commits_ratios`에서 `chore: update deps`가 `other`로 분류되는지 확인.

### 5.4 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/git/mood.rs` | `MoodStats`에 `other` 추가, `analyze_mood()` 수정 |
| `src/output/terminal.rs` | Commit Mood에 Other 행 추가 |
| `src/output/json.rs` | `MoodJson`에 `other_pct` 추가 |
| `src/scoring/mod.rs` | `mood_score` 계산에 `other` 가중치 추가 (50.0 — 중립) |

---

## 6. Q-01: 대형 레포 성능 최적화

### 6.1 현재 문제

`collect_file_changes_by_id()`에서 **매 커밋마다** 전체 트리를 재귀적으로 순회합니다. `O(커밋 수 × 파일 수)`의 시간 복잡도로, 대형 레포(10k+ 커밋)에서 매우 느립니다.

### 6.2 설계

**전략: diff 기반 변경 감지**

gix의 트리 비교 API를 활용하는 것이 이상적이지만, 현재 gix 0.72의 diff API가 제한적이므로 **실용적 최적화**를 적용합니다.

**접근법**: 트리 OID 캐싱 + 조기 중단

```rust
fn collect_file_changes_by_id(
    repo: &gix::Repository,
    commit: &gix::Commit,
    parent_ids: &[gix::ObjectId],
) -> Result<Vec<FileChange>, VibeError> {
    let current_tree = commit.tree()?;

    if parent_ids.is_empty() {
        // 최초 커밋: 전체 파일 추가 (기존과 동일)
        return Ok(collect_all_as_additions(repo, &current_tree));
    }

    let parent_obj = repo.find_object(parent_ids[0])?.into_commit();
    let parent_tree = parent_obj.tree()?;

    // 최적화 1: 트리 루트 OID가 같으면 변경 없음
    if current_tree.id == parent_tree.id {
        return Ok(vec![]);
    }

    // 최적화 2: 서브트리 비교 시 OID 같으면 스킵
    diff_trees(repo, &parent_tree, &current_tree, "")
}

fn diff_trees(
    repo: &gix::Repository,
    old_tree: &gix::Tree,
    new_tree: &gix::Tree,
    prefix: &str,
) -> Result<Vec<FileChange>, VibeError> {
    let old_entries = tree_to_map(old_tree)?;  // HashMap<이름, (mode, OID)>
    let new_entries = tree_to_map(new_tree)?;

    let mut changes = Vec::new();

    for (name, (new_mode, new_oid)) in &new_entries {
        let full_path = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", prefix, name)
        };

        match old_entries.get(name) {
            None => {
                // 추가됨
                if is_blob(*new_mode) {
                    changes.push(FileChange { path: full_path, additions: 1, deletions: 0 });
                } else if is_tree(*new_mode) {
                    // 새 서브트리의 모든 파일 추가
                    collect_tree_additions(repo, *new_oid, &full_path, &mut changes);
                }
            }
            Some((old_mode, old_oid)) => {
                if new_oid == old_oid {
                    continue; // OID 같으면 스킵 (핵심 최적화)
                }
                if is_blob(*new_mode) && is_blob(*old_mode) {
                    // 수정됨
                    changes.push(FileChange { path: full_path, additions: 1, deletions: 1 });
                } else if is_tree(*new_mode) && is_tree(*old_mode) {
                    // 서브트리 재귀 비교
                    let old_sub = repo.find_object(*old_oid)?.into_tree();
                    let new_sub = repo.find_object(*new_oid)?.into_tree();
                    changes.extend(diff_trees(repo, &old_sub, &new_sub, &full_path)?);
                }
            }
        }
    }

    // 삭제된 항목
    for (name, (old_mode, _)) in &old_entries {
        if !new_entries.contains_key(name) {
            let full_path = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };
            if is_blob(*old_mode) {
                changes.push(FileChange { path: full_path, additions: 0, deletions: 1 });
            }
        }
    }

    Ok(changes)
}
```

**기대 성능 향상**:
- 변경이 없는 서브트리 전체를 OID 비교 한 번으로 스킵
- 일반적인 커밋은 소수 파일만 변경하므로 O(변경된 파일 수)에 가까움
- F-03(수정 파일 감지)도 이 설계에 포함됨 (OID 비교 기반)

### 6.3 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/git/commits.rs` | `collect_file_changes_by_id()` 전면 리팩토링, `diff_trees()` 신규 |
| `src/git/commits.rs` | `collect_tree_files()` — `list_all_files()`에서만 사용하도록 유지 |

> **참고**: F-03과 Q-01은 동일 함수를 수정하므로 **동시 구현**합니다. diff_trees() 방식이 blob ID 비교와 성능 최적화를 모두 해결합니다.

---

## 7. Q-02: 에러 메시지 개선

### 7.1 설계

**변경 대상**: `src/error.rs`

사용자 친화적 메시지를 추가합니다.

```rust
#[derive(Error, Debug)]
pub enum VibeError {
    #[error("Git 저장소를 열 수 없습니다: {0}\n  힌트: git-vibe를 Git 저장소 안에서 실행하거나 --path 옵션을 사용하세요")]
    GitOpen(String),

    #[error("Git 분석 중 오류: {0}")]
    Git(String),

    #[error("잘못된 기간 형식: {0}\n  사용법: 3m (3개월), 6m (6개월), 1y (1년), 30d (30일)")]
    InvalidPeriod(String),

    #[error("지정한 기간에 커밋이 없습니다\n  힌트: --period 옵션으로 더 긴 기간을 지정해 보세요 (예: --period 1y)")]
    NoCommits,

    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),
}
```

`Git(String)` 중 저장소 열기 실패는 `GitOpen`으로 분리하여 힌트 메시지를 제공합니다.

### 7.2 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/error.rs` | `GitOpen` 변형 추가, 기존 에러 메시지 한국어+힌트 |
| `src/git/commits.rs` | `gix::open()` 실패 시 `VibeError::GitOpen` 사용 |
| `src/git/mod.rs` | `gix::open()` 호출부도 `GitOpen` 적용 |

---

## 8. Q-03: 고스트 파일 정렬

### 8.1 설계

**변경 대상**: `src/git/ghosts.rs`

`detect_ghosts()` 반환 전에 `days_ago` 기준 내림차순 정렬을 추가합니다.

```rust
pub fn detect_ghosts(...) -> Vec<GhostFile> {
    let mut ghosts: Vec<GhostFile> = all_files
        .iter()
        .filter_map(|file| { /* 기존 로직 */ })
        .collect();

    // 가장 오래된 파일이 먼저 표시되도록 정렬
    ghosts.sort_by(|a, b| b.days_ago.cmp(&a.days_ago));

    ghosts
}
```

### 8.2 영향 범위

| 파일 | 변경 내용 |
|------|----------|
| `src/git/ghosts.rs` | `detect_ghosts()` 반환 전 정렬 추가 (1줄) |

---

## 9. 구현 순서

의존성과 위험도를 고려한 구현 순서입니다.

```
┌─────────────────────────────────────────────────────────┐
│ Step 1: F-03 + Q-01 (동시)                               │
│   파일 변경 감지 + 성능 최적화                              │
│   → src/git/commits.rs 전면 리팩토링                      │
│   → diff_trees() 기반으로 전환                            │
├─────────────────────────────────────────────────────────┤
│ Step 2: F-04                                            │
│   커밋 감정 "기타" 카테고리                                 │
│   → src/git/mood.rs, output 수정                        │
├─────────────────────────────────────────────────────────┤
│ Step 3: F-01                                            │
│   활동 추세 월별 시각화                                    │
│   → src/git/mod.rs, output 수정                         │
├─────────────────────────────────────────────────────────┤
│ Step 4: F-02                                            │
│   Hall of Fame 이모지 칭호                                │
│   → src/output/terminal.rs 수정                         │
├─────────────────────────────────────────────────────────┤
│ Step 5: Q-02                                            │
│   에러 메시지 개선                                        │
│   → src/error.rs, git/commits.rs 수정                   │
├─────────────────────────────────────────────────────────┤
│ Step 6: Q-03                                            │
│   고스트 파일 정렬                                        │
│   → src/git/ghosts.rs 1줄 추가                          │
├─────────────────────────────────────────────────────────┤
│ Step 7: 전체 테스트 + 검증                                │
│   → cargo test, 실제 레포 실행 확인                       │
└─────────────────────────────────────────────────────────┘
```

### 구현 순서 근거

| 순서 | 이유 |
|------|------|
| Step 1 (F-03+Q-01) 최우선 | `commits.rs`의 핵심 함수를 변경하므로 다른 기능에 영향. 먼저 안정화 필요 |
| Step 2 (F-04) 두 번째 | `MoodStats` 구조체 변경이 scoring과 output에 영향. 빨리 확정 |
| Step 3 (F-01) 세 번째 | `AnalysisResult`에 필드 추가. output 쪽 변경이 크지만 독립적 |
| Step 4 (F-02) 네 번째 | output 전용 변경. 다른 모듈에 영향 없음 |
| Step 5-6 (Q-02, Q-03) 후순위 | 작은 변경, 독립적, 리스크 낮음 |

---

## 10. 변경 파일 요약

| 파일 | Step | 변경 유형 | 변경 규모 |
|------|------|----------|----------|
| `src/git/commits.rs` | 1, 5 | 리팩토링 + 수정 | 대 (diff_trees 신규, collect_file_changes 재작성) |
| `src/git/mod.rs` | 3, 5 | 수정 | 중 (MonthlyActivity 추가, analyze() 수정) |
| `src/git/mood.rs` | 2 | 수정 | 소 (other 필드 추가) |
| `src/git/ghosts.rs` | 6 | 수정 | 소 (정렬 1줄) |
| `src/scoring/mod.rs` | 2 | 수정 | 소 (mood_score에 other 가중치) |
| `src/output/terminal.rs` | 2, 3, 4 | 수정 | 중 (mood 행, 월별 그래프, 칭호 컬럼) |
| `src/output/json.rs` | 2, 3 | 수정 | 소 (other_pct, monthly_activity) |
| `src/error.rs` | 5 | 수정 | 소 (GitOpen 변형, 메시지 개선) |

---

## 11. 테스트 전략

### 11.1 기존 테스트 유지

모든 변경 후 기존 23개 테스트가 통과해야 합니다.

```bash
cargo test
```

### 11.2 추가 테스트

| 대상 | 테스트 | 위치 |
|------|--------|------|
| F-03+Q-01 | 수정 파일이 FileChange에 포함되는지 | `src/git/commits.rs` (단위) |
| F-04 | `other` 비율이 정확한지 | `src/git/mood.rs` (기존 테스트 수정) |
| F-01 | 월별 집계 정확성 | `src/git/mod.rs` (단위) |
| Q-03 | 고스트 파일 정렬 순서 | `src/git/ghosts.rs` (단위) |
| 통합 | JSON 출력에 새 필드 포함 | `tests/cli_tests.rs` |

### 11.3 수동 검증

```bash
# 현재 레포에서 실행
cargo run

# 대형 레포에서 성능 확인
cargo run -- --path /path/to/large-repo --period 6m

# JSON 출력 검증
cargo run -- --json | python3 -m json.tool
```

---

## 12. 비기능 요구사항

| 항목 | 기준 |
|------|------|
| 빌드 | `cargo build --release` 성공 |
| 테스트 | `cargo test` 전수 통과 |
| 경고 | `cargo clippy` 경고 0개 |
| 성능 | 10k 커밋 레포에서 30초 이내 |
| 호환성 | Rust 2021 edition, 기존 CLI 옵션 하위 호환 |
