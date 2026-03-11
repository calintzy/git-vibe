use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use crate::git::commits::CommitData;

#[derive(Debug, Clone, Serialize)]
pub struct GhostFile {
    pub path: String,
    pub last_touched: DateTime<Utc>,
    pub days_ago: u64,
}

/// 오랫동안 수정되지 않은 파일(고스트) 탐지
pub fn detect_ghosts(
    commits: &[CommitData],
    all_files: &[String],
    threshold_days: u64,
) -> Vec<GhostFile> {
    // 파일별 마지막 커밋 시간 집계
    let mut last_touched: HashMap<String, DateTime<Utc>> = HashMap::new();

    for commit in commits {
        for file in &commit.files_changed {
            let entry = last_touched.entry(file.path.clone()).or_insert(commit.timestamp);
            if commit.timestamp > *entry {
                *entry = commit.timestamp;
            }
        }
    }

    let now = Utc::now();
    let threshold = chrono::Duration::days(threshold_days as i64);

    let mut ghosts: Vec<GhostFile> = all_files
        .iter()
        .filter_map(|file| {
            let touched = last_touched.get(file).copied().unwrap_or(DateTime::UNIX_EPOCH);
            let age = now - touched;
            if age > threshold {
                let days_ago = age.num_days() as u64;
                Some(GhostFile {
                    path: file.clone(),
                    last_touched: touched,
                    days_ago,
                })
            } else {
                None
            }
        })
        .collect();

    // 가장 오래된 파일이 먼저 표시되도록 정렬
    ghosts.sort_by(|a, b| b.days_ago.cmp(&a.days_ago));
    ghosts
}
