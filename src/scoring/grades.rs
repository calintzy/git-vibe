use std::fmt;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum VibeGrade {
    Zen,
    Chill,
    Active,
    Tense,
    Stressed,
    Chaotic,
    Abandoned,
}

impl fmt::Display for VibeGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VibeGrade::Zen => "🧘 Zen",
            VibeGrade::Chill => "😎 Chill",
            VibeGrade::Active => "💪 Active",
            VibeGrade::Tense => "😬 Tense",
            VibeGrade::Stressed => "😰 Stressed",
            VibeGrade::Chaotic => "🔥 Chaotic",
            VibeGrade::Abandoned => "💀 Abandoned",
        };
        write!(f, "{}", s)
    }
}

impl VibeGrade {
    pub fn emoji(&self) -> &'static str {
        match self {
            VibeGrade::Zen => "🧘",
            VibeGrade::Chill => "😎",
            VibeGrade::Active => "💪",
            VibeGrade::Tense => "😬",
            VibeGrade::Stressed => "😰",
            VibeGrade::Chaotic => "🔥",
            VibeGrade::Abandoned => "💀",
        }
    }
}

/// 점수(0.0~100.0)를 바이브 등급으로 변환
pub fn grade_from_score(score: f64) -> VibeGrade {
    match score as u32 {
        90..=100 => VibeGrade::Zen,
        80..=89 => VibeGrade::Chill,
        70..=79 => VibeGrade::Active,
        60..=69 => VibeGrade::Tense,
        50..=59 => VibeGrade::Stressed,
        40..=49 => VibeGrade::Chaotic,
        _ => VibeGrade::Abandoned,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_abandoned_low() {
        assert_eq!(grade_from_score(0.0), VibeGrade::Abandoned);
    }

    #[test]
    fn test_grade_abandoned_boundary() {
        assert_eq!(grade_from_score(39.0), VibeGrade::Abandoned);
    }

    #[test]
    fn test_grade_chaotic_low() {
        assert_eq!(grade_from_score(40.0), VibeGrade::Chaotic);
    }

    #[test]
    fn test_grade_chaotic_high() {
        assert_eq!(grade_from_score(49.0), VibeGrade::Chaotic);
    }

    #[test]
    fn test_grade_stressed_low() {
        assert_eq!(grade_from_score(50.0), VibeGrade::Stressed);
    }

    #[test]
    fn test_grade_stressed_high() {
        assert_eq!(grade_from_score(59.0), VibeGrade::Stressed);
    }

    #[test]
    fn test_grade_tense_low() {
        assert_eq!(grade_from_score(60.0), VibeGrade::Tense);
    }

    #[test]
    fn test_grade_tense_high() {
        assert_eq!(grade_from_score(69.0), VibeGrade::Tense);
    }

    #[test]
    fn test_grade_active_low() {
        assert_eq!(grade_from_score(70.0), VibeGrade::Active);
    }

    #[test]
    fn test_grade_active_high() {
        assert_eq!(grade_from_score(79.0), VibeGrade::Active);
    }

    #[test]
    fn test_grade_chill_low() {
        assert_eq!(grade_from_score(80.0), VibeGrade::Chill);
    }

    #[test]
    fn test_grade_chill_high() {
        assert_eq!(grade_from_score(89.0), VibeGrade::Chill);
    }

    #[test]
    fn test_grade_zen_low() {
        assert_eq!(grade_from_score(90.0), VibeGrade::Zen);
    }

    #[test]
    fn test_grade_zen_high() {
        assert_eq!(grade_from_score(100.0), VibeGrade::Zen);
    }
}
