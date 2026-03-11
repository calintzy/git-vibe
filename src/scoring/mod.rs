pub mod grades;

use serde::Serialize;
use grades::{VibeGrade, grade_from_score};
use crate::git::AnalysisResult;

#[derive(Debug, Clone, Serialize)]
pub struct VibeScore {
    pub total: f64,
    pub grade: VibeGrade,
    pub commit_frequency: f64,
    pub hotspot_score: f64,
    pub ghost_score: f64,
    pub mood_score: f64,
    pub bus_factor_score: f64,
    pub trend_score: f64,
}

fn clamp(v: f64) -> f64 {
    v.clamp(0.0, 100.0)
}

/// 커밋 빈도 점수 계산
fn calc_frequency(commits_per_week: f64) -> f64 {
    if commits_per_week == 0.0 {
        0.0
    } else if commits_per_week <= 3.0 {
        50.0
    } else if commits_per_week <= 10.0 {
        80.0
    } else if commits_per_week <= 20.0 {
        100.0
    } else {
        90.0
    }
}

/// 버스 팩터 점수 계산
fn calc_bus_factor_score(bus_factor: u32) -> f64 {
    match bus_factor {
        0 => 0.0,
        1 => 20.0,
        2 => 50.0,
        3 => 75.0,
        _ => 100.0,
    }
}

/// 트렌드 점수: 분석 기간을 시간 3등분하여 최근 vs 과거 커밋 수 비교
fn calc_trend_score(analysis: &AnalysisResult) -> f64 {
    let commits = &analysis.commits;
    if commits.len() < 3 {
        return 60.0;
    }

    // 커밋 타임스탬프에서 전체 기간 계산
    let newest = commits.iter().map(|c| c.timestamp).max().unwrap();
    let oldest = commits.iter().map(|c| c.timestamp).min().unwrap();
    let total_duration = newest - oldest;
    let third_duration = total_duration / 3;

    if third_duration.num_seconds() == 0 {
        return 60.0;
    }

    let boundary_recent = newest - third_duration;
    let boundary_old = oldest + third_duration;

    // 최근 1/3 기간의 커밋 수
    let recent_count = commits.iter().filter(|c| c.timestamp >= boundary_recent).count() as f64;
    // 과거 1/3 기간의 커밋 수
    let old_count = commits.iter().filter(|c| c.timestamp <= boundary_old).count() as f64;

    if old_count == 0.0 {
        return 85.0;
    }

    let ratio = recent_count / old_count;

    if ratio > 1.2 {
        85.0 // 상승
    } else if ratio >= 0.8 {
        60.0 // 안정
    } else {
        30.0 // 하락
    }
}

/// 전체 바이브 점수 계산
pub fn calculate_vibe_score(analysis: &AnalysisResult) -> VibeScore {
    // 기간(주 수) 계산
    let period_weeks = {
        let period = &analysis.period_label;
        let len = period.len();
        if len >= 2 {
            let (num_str, unit) = period.split_at(len - 1);
            let num: f64 = num_str.parse().unwrap_or(12.0);
            match unit {
                "d" => num / 7.0,
                "w" => num,
                "m" => num * 4.33,
                "y" => num * 52.0,
                _ => 52.0,
            }
        } else {
            52.0
        }
    };

    let commits_per_week = if period_weeks > 0.0 {
        analysis.total_commits as f64 / period_weeks
    } else {
        0.0
    };

    let commit_frequency = calc_frequency(commits_per_week);

    // 핫스팟 스코어: 평균 커밋수의 3배 이상 변경된 파일 비율
    let hotspot_score = if analysis.total_files > 0 && analysis.total_commits > 0 {
        let avg_commits_per_file = analysis.total_commits as f64 / analysis.total_files as f64;
        let threshold = avg_commits_per_file * 3.0;
        let hot_files = analysis.hotspots.iter()
            .filter(|h| h.commit_count as f64 >= threshold)
            .count() as f64;
        clamp(100.0 - (hot_files / analysis.total_files as f64 * 100.0))
    } else {
        100.0
    };

    let ghost_score = if analysis.total_files > 0 {
        clamp(100.0 - (analysis.ghosts.len() as f64 / analysis.total_files as f64 * 100.0))
    } else {
        100.0
    };

    let mood = &analysis.mood;
    let mood_score = clamp(
        mood.happy * 100.0
            + mood.cleanup * 70.0
            + mood.other * 50.0
            + mood.stressed * 30.0
            + mood.scary * 10.0,
    );

    let bus_factor_score = calc_bus_factor_score(analysis.bus_factor);

    let trend_score = calc_trend_score(analysis);

    let total = clamp(
        commit_frequency * 0.20
            + hotspot_score * 0.20
            + ghost_score * 0.15
            + mood_score * 0.20
            + bus_factor_score * 0.15
            + trend_score * 0.10,
    );

    let grade = grade_from_score(total);

    VibeScore {
        total,
        grade,
        commit_frequency,
        hotspot_score,
        ghost_score,
        mood_score,
        bus_factor_score,
        trend_score,
    }
}
