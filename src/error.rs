use thiserror::Error;

#[derive(Error, Debug)]
pub enum VibeError {
    #[error("Git 저장소를 열 수 없습니다: {0}\n  힌트: git-vibe를 Git 저장소 안에서 실행하거나 --path 옵션을 사용하세요")]
    GitOpen(String),
    #[error("Git 분석 중 오류: {0}")]
    Git(String),
    #[error("잘못된 기간 형식: {0}\n  사용법: 3m (3개월), 6m (6개월), 1y (1년), 30d (30일)")]
    InvalidPeriod(String),
    #[error("지정한 기간에 커밋이 없습니다\n  힌트: --period 옵션으로 더 긴 기간을 지정해 보세요 (예: --period 1y)")]
    NoCommits,
    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),
}
