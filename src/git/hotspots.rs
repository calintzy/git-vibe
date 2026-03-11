use serde::Serialize;
use std::collections::HashMap;
use crate::git::commits::CommitData;

#[derive(Debug, Clone, Serialize)]
pub struct Hotspot {
    pub path: String,
    pub commit_count: u32,
    pub heat_emoji: &'static str,
}

fn heat_emoji(count: u32) -> &'static str {
    if count >= 50 {
        "🌋"
    } else if count >= 30 {
        "🔥"
    } else if count >= 15 {
        "♨️"
    } else {
        "🔆"
    }
}

/// 커밋 수 기준 핫스팟 파일 탐지
pub fn detect_hotspots(commits: &[CommitData], top_n: usize) -> Vec<Hotspot> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    for commit in commits {
        for file in &commit.files_changed {
            *counts.entry(file.path.clone()).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<(String, u32)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    sorted
        .into_iter()
        .take(top_n)
        .map(|(path, commit_count)| Hotspot {
            heat_emoji: heat_emoji(commit_count),
            path,
            commit_count,
        })
        .collect()
}
