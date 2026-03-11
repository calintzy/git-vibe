mod error;
mod cli;
mod git;
mod scoring;
mod output;

use clap::Parser;
use cli::Cli;
use std::path::PathBuf;
use std::process;

fn main() {
    let cli = Cli::parse();

    let path = cli.path.clone();

    match git::analyze(&path, &cli.period) {
        Ok(analysis) => {
            let score = scoring::calculate_vibe_score(&analysis);

            if cli.badge {
                let out_path = cli.output.clone()
                    .unwrap_or_else(|| PathBuf::from("vibe-badge.svg"));
                if let Err(e) = output::render_badge(&score, &out_path) {
                    eprintln!("배지 생성 오류: {}", e);
                    process::exit(1);
                }
                println!("배지 생성 완료: {}", out_path.display());
            } else if cli.share {
                let ext = if cli.format == "png" { "png" } else { "svg" };
                let out_path = cli.output.clone()
                    .unwrap_or_else(|| PathBuf::from(format!("vibe-report.{}", ext)));
                if let Err(e) = output::render_share(&score, &analysis, &out_path, &cli.format) {
                    eprintln!("이미지 생성 오류: {}", e);
                    process::exit(1);
                }
                println!("이미지 생성 완료: {}", out_path.display());
            } else if let Err(e) = output::render(&score, &analysis, cli.json, cli.leaderboard) {
                eprintln!("출력 오류: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("오류: {}", e);
            process::exit(1);
        }
    }
}
