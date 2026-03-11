use crate::scoring::VibeScore;
use crate::scoring::grades::VibeGrade;

/// 등급별 배지 배경색
fn badge_color(grade: &VibeGrade) -> &'static str {
    match grade {
        VibeGrade::Zen => "#4c1",
        VibeGrade::Chill => "#97CA00",
        VibeGrade::Active => "#dfb317",
        VibeGrade::Tense => "#fe7d37",
        VibeGrade::Stressed => "#e05d44",
        VibeGrade::Chaotic => "#e05d44",
        VibeGrade::Abandoned => "#9f9f9f",
    }
}

/// XML 특수문자 이스케이프
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

/// shields.io 스타일 배지 SVG 생성
pub fn render_badge(score: &VibeScore) -> String {
    let grade_str = score.grade.to_string();
    let value_text = format!("{} ({:.0})", grade_str, score.total);
    let color = badge_color(&score.grade);

    let label = "vibe";
    let label_width = 40u32;
    let value_width = (value_text.len() as u32 * 7) + 10;
    let total_width = label_width + value_width;
    let label_x = label_width / 2;
    let value_x = label_width + value_width / 2;

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total}" height="20" viewBox="0 0 {total} 20">
  <linearGradient id="s" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <clipPath id="r"><rect width="{total}" height="20" rx="3" fill="#fff"/></clipPath>
  <g clip-path="url(#r)">
    <rect width="{lw}" height="20" fill="#555"/>
    <rect x="{lw}" width="{vw}" height="20" fill="{color}"/>
    <rect width="{total}" height="20" fill="url(#s)"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,sans-serif" font-size="11">
    <text x="{lx}" y="15" fill="#010101" fill-opacity=".3">{label}</text>
    <text x="{lx}" y="14">{label}</text>
    <text x="{vx}" y="15" fill="#010101" fill-opacity=".3">{value}</text>
    <text x="{vx}" y="14">{value}</text>
  </g>
</svg>"##,
        total = total_width,
        lw = label_width,
        vw = value_width,
        color = color,
        lx = label_x,
        vx = value_x,
        label = label,
        value = xml_escape(&value_text),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::grades::VibeGrade;

    #[test]
    fn test_badge_color_mapping() {
        assert_eq!(badge_color(&VibeGrade::Zen), "#4c1");
        assert_eq!(badge_color(&VibeGrade::Chaotic), "#e05d44");
        assert_eq!(badge_color(&VibeGrade::Abandoned), "#9f9f9f");
    }

    #[test]
    fn test_render_badge_contains_structure() {
        let score = VibeScore {
            total: 82.0,
            grade: VibeGrade::Chill,
            commit_frequency: 80.0,
            hotspot_score: 72.0,
            ghost_score: 85.0,
            mood_score: 65.0,
            bus_factor_score: 50.0,
            trend_score: 60.0,
        };

        let svg = render_badge(&score);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("vibe"));
        assert!(svg.contains("Chill"));
        assert!(svg.contains("#97CA00"));
        assert!(svg.contains("82"));
    }

    #[test]
    fn test_badge_xml_escape() {
        assert_eq!(xml_escape("a<b>&c"), "a&lt;b&gt;&amp;c");
    }
}
