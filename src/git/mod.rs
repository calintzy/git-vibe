pub mod commits;
pub mod hotspots;
pub mod ghosts;
pub mod mood;
pub mod authors;

use std::path::Path;
use serde::Serialize;
use crate::error::VibeError;
use std::collections::HashMap;
use commits::CommitData;
use hotspots::Hotspot;
use ghosts::GhostFile;
use mood::MoodStats;
use authors::AuthorStats;

#[derive(Debug, Clone, Serialize)]
pub struct MonthlyActivity {
    pub label: String,
    pub commit_count: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    pub repo_name: String,
    pub period_label: String,
    pub total_commits: u32,
    pub total_files: u32,
    pub hotspots: Vec<Hotspot>,
    pub ghosts: Vec<GhostFile>,
    pub mood: MoodStats,
    pub authors: Vec<AuthorStats>,
    pub bus_factor: u32,
    pub monthly_activity: Vec<MonthlyActivity>,
    pub commits: Vec<CommitData>,
}

/// 저장소를 분석하여 결과 반환
pub fn analyze(path: &Path, period: &str) -> Result<AnalysisResult, VibeError> {
    // 기간 파싱
    let since = commits::parse_period(period)?;

    // 저장소 이름 추출
    let repo_name = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // 커밋 수집
    let commit_list = commits::collect_commits(path, since)?;
    if commit_list.is_empty() {
        return Err(VibeError::NoCommits);
    }

    // 전체 파일 목록
    let all_files = commits::list_all_files(path).unwrap_or_default();
    let total_files = all_files.len() as u32;

    // 각 분석 실행
    let hotspots = hotspots::detect_hotspots(&commit_list, 10);
    let ghosts = ghosts::detect_ghosts(&commit_list, &all_files, 365);
    let mood = mood::analyze_mood(&commit_list);
    let author_stats = authors::collect_author_stats(&commit_list);
    let bus_factor = authors::calculate_bus_factor(&author_stats);
    let total_commits = commit_list.len() as u32;
    let monthly_activity = calc_monthly_activity(&commit_list);

    Ok(AnalysisResult {
        repo_name,
        period_label: period.to_string(),
        total_commits,
        total_files,
        hotspots,
        ghosts,
        mood,
        authors: author_stats,
        bus_factor,
        monthly_activity,
        commits: commit_list,
    })
}

/// 커밋을 월별로 그룹핑하여 활동 통계 반환
fn calc_monthly_activity(commits: &[CommitData]) -> Vec<MonthlyActivity> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for commit in commits {
        let label = commit.timestamp.format("%Y-%m").to_string();
        *counts.entry(label).or_insert(0) += 1;
    }
    let mut result: Vec<MonthlyActivity> = counts
        .into_iter()
        .map(|(label, commit_count)| MonthlyActivity { label, commit_count })
        .collect();
    result.sort_by(|a, b| a.label.cmp(&b.label));
    result
}
