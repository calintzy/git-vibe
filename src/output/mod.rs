pub mod json;
pub mod terminal;
pub mod svg;
pub mod badge;

use std::path::Path;
use crate::error::VibeError;
use crate::git::AnalysisResult;
use crate::scoring::VibeScore;

pub fn render(
    score: &VibeScore,
    analysis: &AnalysisResult,
    json: bool,
    leaderboard: bool,
) -> Result<(), VibeError> {
    if json {
        let output = self::json::render_json(score, analysis)?;
        println!("{}", output);
    } else {
        self::terminal::render_terminal(score, analysis, leaderboard);
    }
    Ok(())
}

/// 공유용 이미지 생성
pub fn render_share(
    score: &VibeScore,
    analysis: &AnalysisResult,
    path: &Path,
    format: &str,
) -> Result<(), VibeError> {
    let svg_content = svg::render_svg(score, analysis);

    match format {
        "png" => svg::save_as_png(&svg_content, path),
        _ => std::fs::write(path, &svg_content).map_err(VibeError::Io),
    }
}

/// 배지 SVG 생성
pub fn render_badge(
    score: &VibeScore,
    path: &Path,
) -> Result<(), VibeError> {
    let svg_content = badge::render_badge(score);
    std::fs::write(path, &svg_content).map_err(VibeError::Io)
}
