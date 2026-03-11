use serde::Serialize;
use crate::git::commits::CommitData;

#[derive(Debug, Clone, Serialize)]
pub struct MoodStats {
    pub happy: f64,
    pub stressed: f64,
    pub cleanup: f64,
    pub scary: f64,
    pub other: f64,
    pub total_commits: u32,
}

const HAPPY_KEYWORDS: &[&str] = &[
    "feat", "add", "improve", "enhance", "new", "create", "implement", "support",
];

const STRESSED_KEYWORDS: &[&str] = &[
    "fix", "hotfix", "urgent", "bug", "patch", "resolve",
];

const CLEANUP_KEYWORDS: &[&str] = &[
    "refactor", "clean", "reorganize", "simplify", "rename", "tidy", "format", "style",
];

const SCARY_KEYWORDS: &[&str] = &[
    "revert", "rollback", "remove", "delete", "drop", "deprecate",
];

fn classify_message(message: &str) -> Option<&'static str> {
    let lower = message.to_lowercase();
    // conventional commit prefix 또는 첫 단어 추출
    let first_word = lower
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(':');

    if HAPPY_KEYWORDS.iter().any(|&k| first_word.starts_with(k)) {
        return Some("happy");
    }
    if STRESSED_KEYWORDS.iter().any(|&k| first_word.starts_with(k)) {
        return Some("stressed");
    }
    if CLEANUP_KEYWORDS.iter().any(|&k| first_word.starts_with(k)) {
        return Some("cleanup");
    }
    if SCARY_KEYWORDS.iter().any(|&k| first_word.starts_with(k)) {
        return Some("scary");
    }
    None
}

#[cfg(test)]
fn make_commit(message: &str) -> CommitData {
    CommitData {
        id: "abc123".to_string(),
        author: "Test".to_string(),
        email: "test@test.com".to_string(),
        message: message.to_string(),
        timestamp: chrono::Utc::now(),
        files_changed: vec![],
    }
}

/// 커밋 메시지를 분석하여 분위기 통계 반환
pub fn analyze_mood(commits: &[CommitData]) -> MoodStats {
    let total = commits.len() as u32;
    if total == 0 {
        return MoodStats { happy: 0.0, stressed: 0.0, cleanup: 0.0, scary: 0.0, other: 0.0, total_commits: 0 };
    }

    let mut happy = 0u32;
    let mut stressed = 0u32;
    let mut cleanup = 0u32;
    let mut scary = 0u32;

    for commit in commits {
        match classify_message(&commit.message) {
            Some("happy") => happy += 1,
            Some("stressed") => stressed += 1,
            Some("cleanup") => cleanup += 1,
            Some("scary") => scary += 1,
            _ => {}
        }
    }

    let classified = happy + stressed + cleanup + scary;
    let other = total - classified;
    let t = total as f64;
    MoodStats {
        happy: happy as f64 / t,
        stressed: stressed as f64 / t,
        cleanup: cleanup as f64 / t,
        scary: scary as f64 / t,
        other: other as f64 / t,
        total_commits: total,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feat_is_happy() {
        let commits = vec![make_commit("feat: add login")];
        let mood = analyze_mood(&commits);
        assert_eq!(mood.happy, 1.0);
        assert_eq!(mood.stressed, 0.0);
        assert_eq!(mood.cleanup, 0.0);
        assert_eq!(mood.scary, 0.0);
        assert_eq!(mood.other, 0.0);
    }

    #[test]
    fn test_fix_is_stressed() {
        let commits = vec![make_commit("fix: crash on startup")];
        let mood = analyze_mood(&commits);
        assert_eq!(mood.stressed, 1.0);
        assert_eq!(mood.happy, 0.0);
    }

    #[test]
    fn test_refactor_is_cleanup() {
        let commits = vec![make_commit("refactor: simplify auth")];
        let mood = analyze_mood(&commits);
        assert_eq!(mood.cleanup, 1.0);
        assert_eq!(mood.happy, 0.0);
    }

    #[test]
    fn test_revert_is_scary() {
        let commits = vec![make_commit("revert: bad deploy")];
        let mood = analyze_mood(&commits);
        assert_eq!(mood.scary, 1.0);
        assert_eq!(mood.happy, 0.0);
    }

    #[test]
    fn test_empty_commits_all_zeros() {
        let mood = analyze_mood(&[]);
        assert_eq!(mood.happy, 0.0);
        assert_eq!(mood.stressed, 0.0);
        assert_eq!(mood.cleanup, 0.0);
        assert_eq!(mood.scary, 0.0);
        assert_eq!(mood.other, 0.0);
        assert_eq!(mood.total_commits, 0);
    }

    #[test]
    fn test_mixed_commits_ratios() {
        let commits = vec![
            make_commit("feat: new feature"),
            make_commit("fix: some bug"),
            make_commit("fix: another bug"),
            make_commit("chore: update deps"),
        ];
        let mood = analyze_mood(&commits);
        assert_eq!(mood.total_commits, 4);
        assert!((mood.happy - 0.25).abs() < f64::EPSILON);
        assert!((mood.stressed - 0.5).abs() < f64::EPSILON);
        assert!((mood.other - 0.25).abs() < f64::EPSILON);
    }
}
