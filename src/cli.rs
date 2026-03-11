use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "git-vibe", about = "🎭 Vibe check your codebase", version)]
pub struct Cli {
    /// 분석할 Git 저장소 경로
    #[arg(short = 'p', long = "path", default_value = ".")]
    pub path: PathBuf,

    /// 분석 기간 (예: 3m, 6m, 12m, 1y)
    #[arg(short = 'P', long = "period", default_value = "12m")]
    pub period: String,

    /// JSON 형식으로 출력
    #[arg(long = "json")]
    pub json: bool,

    /// 기여자 리더보드 표시
    #[arg(long = "leaderboard")]
    pub leaderboard: bool,

    /// 공유용 SVG/PNG 이미지 생성
    #[arg(long = "share")]
    pub share: bool,

    /// 이미지 출력 형식 (svg, png)
    #[arg(long = "format", default_value = "svg")]
    pub format: String,

    /// 출력 파일 경로
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// README 배지 SVG 생성
    #[arg(long = "badge")]
    pub badge: bool,
}
