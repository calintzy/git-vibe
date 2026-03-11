use serde::Serialize;
use crate::error::VibeError;
use crate::git::AnalysisResult;
use crate::scoring::VibeScore;

#[derive(Serialize)]
struct HotspotJson {
    path: String,
    commit_count: u32,
    heat: &'static str,
}

#[derive(Serialize)]
struct GhostJson {
    path: String,
    days_ago: u64,
}

#[derive(Serialize)]
struct MoodJson {
    happy_pct: f64,
    stressed_pct: f64,
    cleanup_pct: f64,
    scary_pct: f64,
    other_pct: f64,
}

#[derive(Serialize)]
struct AuthorJson {
    name: String,
    commits: u32,
    lines_added: u32,
    lines_removed: u32,
}

#[derive(Serialize)]
struct MonthlyActivityJson {
    month: String,
    commits: u32,
}

#[derive(Serialize)]
struct SubScores {
    commit_frequency: f64,
    hotspot_score: f64,
    ghost_score: f64,
    mood_score: f64,
    bus_factor_score: f64,
    trend_score: f64,
}

#[derive(Serialize)]
struct JsonReport {
    repo_name: String,
    period: String,
    vibe_score: f64,
    vibe_grade: String,
    sub_scores: SubScores,
    hotspots: Vec<HotspotJson>,
    ghost_files: Vec<GhostJson>,
    mood: MoodJson,
    bus_factor: u32,
    monthly_activity: Vec<MonthlyActivityJson>,
    authors: Vec<AuthorJson>,
}

pub fn render_json(score: &VibeScore, analysis: &AnalysisResult) -> Result<String, VibeError> {
    let report = JsonReport {
        repo_name: analysis.repo_name.clone(),
        period: analysis.period_label.clone(),
        vibe_score: score.total,
        vibe_grade: score.grade.to_string(),
        sub_scores: SubScores {
            commit_frequency: score.commit_frequency,
            hotspot_score: score.hotspot_score,
            ghost_score: score.ghost_score,
            mood_score: score.mood_score,
            bus_factor_score: score.bus_factor_score,
            trend_score: score.trend_score,
        },
        hotspots: analysis
            .hotspots
            .iter()
            .map(|h| HotspotJson {
                path: h.path.clone(),
                commit_count: h.commit_count,
                heat: h.heat_emoji,
            })
            .collect(),
        ghost_files: analysis
            .ghosts
            .iter()
            .map(|g| GhostJson {
                path: g.path.clone(),
                days_ago: g.days_ago,
            })
            .collect(),
        mood: MoodJson {
            happy_pct: (analysis.mood.happy * 100.0).round(),
            stressed_pct: (analysis.mood.stressed * 100.0).round(),
            cleanup_pct: (analysis.mood.cleanup * 100.0).round(),
            scary_pct: (analysis.mood.scary * 100.0).round(),
            other_pct: (analysis.mood.other * 100.0).round(),
        },
        bus_factor: analysis.bus_factor,
        monthly_activity: analysis
            .monthly_activity
            .iter()
            .map(|m| MonthlyActivityJson {
                month: m.label.clone(),
                commits: m.commit_count,
            })
            .collect(),
        authors: analysis
            .authors
            .iter()
            .map(|a| AuthorJson {
                name: a.name.clone(),
                commits: a.commits,
                lines_added: a.lines_added,
                lines_removed: a.lines_removed,
            })
            .collect(),
    };

    serde_json::to_string_pretty(&report)
        .map_err(|e| VibeError::Git(format!("JSON 직렬화 오류: {}", e)))
}
