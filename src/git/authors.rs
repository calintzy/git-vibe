use serde::Serialize;
use std::collections::HashMap;
use crate::git::commits::CommitData;

#[derive(Debug, Clone, Serialize)]
pub struct AuthorStats {
    pub name: String,
    pub email: String,
    pub commits: u32,
    pub lines_added: u32,
    pub lines_removed: u32,
}

/// 커밋 데이터에서 저자별 통계 집계
pub fn collect_author_stats(commits: &[CommitData]) -> Vec<AuthorStats> {
    let mut map: HashMap<String, AuthorStats> = HashMap::new();

    for commit in commits {
        let entry = map.entry(commit.author.clone()).or_insert(AuthorStats {
            name: commit.author.clone(),
            email: commit.email.clone(),
            commits: 0,
            lines_added: 0,
            lines_removed: 0,
        });

        entry.commits += 1;
        for file in &commit.files_changed {
            entry.lines_added += file.additions;
            entry.lines_removed += file.deletions;
        }
    }

    let mut result: Vec<AuthorStats> = map.into_values().collect();
    result.sort_by(|a, b| b.commits.cmp(&a.commits));
    result
}

/// 전체 라인의 80%를 커버하는 최소 저자 수(버스 팩터) 계산
pub fn calculate_bus_factor(authors: &[AuthorStats]) -> u32 {
    if authors.is_empty() {
        return 0;
    }

    let total: u32 = authors.iter().map(|a| a.lines_added + a.lines_removed).sum();
    if total == 0 {
        // 라인 수가 없으면 커밋 수 기준
        let total_commits: u32 = authors.iter().map(|a| a.commits).sum();
        if total_commits == 0 {
            return 0;
        }
        let threshold = (total_commits as f64 * 0.8).ceil() as u32;
        let mut sorted: Vec<u32> = authors.iter().map(|a| a.commits).collect();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        let mut cumulative = 0u32;
        for (i, &val) in sorted.iter().enumerate() {
            cumulative += val;
            if cumulative >= threshold {
                return (i + 1) as u32;
            }
        }
        return authors.len() as u32;
    }

    let threshold = (total as f64 * 0.8).ceil() as u32;

    let mut sorted: Vec<u32> = authors
        .iter()
        .map(|a| a.lines_added + a.lines_removed)
        .collect();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    let mut cumulative = 0u32;
    for (i, &val) in sorted.iter().enumerate() {
        cumulative += val;
        if cumulative >= threshold {
            return (i + 1) as u32;
        }
    }

    authors.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_author(name: &str, commits: u32, lines_added: u32, lines_removed: u32) -> AuthorStats {
        AuthorStats {
            name: name.to_string(),
            email: format!("{}@test.com", name),
            commits,
            lines_added,
            lines_removed,
        }
    }

    #[test]
    fn test_bus_factor_single_author() {
        let authors = vec![make_author("alice", 10, 100, 0)];
        assert_eq!(calculate_bus_factor(&authors), 1);
    }

    #[test]
    fn test_bus_factor_two_authors_equal() {
        let authors = vec![
            make_author("alice", 5, 50, 0),
            make_author("bob", 5, 50, 0),
        ];
        // 각 50% → 한 명이 50%, 둘이 합쳐야 80% 이상
        assert_eq!(calculate_bus_factor(&authors), 2);
    }

    #[test]
    fn test_bus_factor_dominant_author() {
        // alice가 90% → 버스 팩터 1
        let authors = vec![
            make_author("alice", 9, 90, 0),
            make_author("bob", 1, 10, 0),
        ];
        assert_eq!(calculate_bus_factor(&authors), 1);
    }

    #[test]
    fn test_bus_factor_three_authors() {
        // 40/35/25 → alice+bob = 75%, alice+bob+carol = 100%
        // 80% 이상이 되려면 세 명 필요
        let authors = vec![
            make_author("alice", 40, 40, 0),
            make_author("bob", 35, 35, 0),
            make_author("carol", 25, 25, 0),
        ];
        // alice(40) + bob(35) = 75 < 80, 세 명이 되어야 100 >= 80
        assert_eq!(calculate_bus_factor(&authors), 3);
    }

    #[test]
    fn test_bus_factor_empty() {
        assert_eq!(calculate_bus_factor(&[]), 0);
    }

    #[test]
    fn test_collect_author_stats_aggregates_commits() {
        use crate::git::commits::{CommitData, FileChange};

        let commits = vec![
            CommitData {
                id: "1".to_string(),
                author: "alice".to_string(),
                email: "alice@test.com".to_string(),
                message: "feat: first".to_string(),
                timestamp: chrono::Utc::now(),
                files_changed: vec![FileChange { path: "a.rs".to_string(), additions: 10, deletions: 2 }],
            },
            CommitData {
                id: "2".to_string(),
                author: "alice".to_string(),
                email: "alice@test.com".to_string(),
                message: "fix: second".to_string(),
                timestamp: chrono::Utc::now(),
                files_changed: vec![FileChange { path: "b.rs".to_string(), additions: 5, deletions: 1 }],
            },
            CommitData {
                id: "3".to_string(),
                author: "bob".to_string(),
                email: "bob@test.com".to_string(),
                message: "feat: bob commit".to_string(),
                timestamp: chrono::Utc::now(),
                files_changed: vec![],
            },
        ];

        let stats = collect_author_stats(&commits);
        assert_eq!(stats.len(), 2);
        let alice = stats.iter().find(|a| a.name == "alice").unwrap();
        assert_eq!(alice.commits, 2);
        assert_eq!(alice.lines_added, 15);
        assert_eq!(alice.lines_removed, 3);
    }
}
