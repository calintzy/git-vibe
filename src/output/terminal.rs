use colored::Colorize;
use comfy_table::{Table, presets::UTF8_FULL};
use crate::git::AnalysisResult;
use crate::scoring::VibeScore;

fn progress_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

pub fn render_terminal(score: &VibeScore, analysis: &AnalysisResult, leaderboard: bool) {
    // 헤더
    println!();
    println!("{}", format!("🎭 Vibe Check: {}", analysis.repo_name).bold().cyan());
    println!("{}", "━".repeat(50).cyan());

    // 전체 점수
    let grade_str = score.grade.to_string();
    println!(
        "Overall Vibe: {} ({}/100)",
        grade_str.bold(),
        format!("{:.0}", score.total).bold()
    );
    println!(
        "📊 Period: {} | Commits: {} | Files: {}",
        analysis.period_label,
        analysis.total_commits,
        analysis.total_files
    );

    // 핫스팟
    println!();
    println!("{}", "🔥 Hotspots (most changed files)".bold().cyan());
    if analysis.hotspots.is_empty() {
        println!("  No hotspots detected ✨");
    } else {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec!["File", "Changes", "Heat"]);
        for h in analysis.hotspots.iter().take(10) {
            table.add_row(vec![
                h.path.clone(),
                h.commit_count.to_string(),
                h.heat_emoji.to_string(),
            ]);
        }
        println!("{table}");
    }

    // 고스트 파일
    println!();
    println!("{}", "👻 Ghost Files (untouched > 1 year)".bold().cyan());
    if analysis.ghosts.is_empty() {
        println!("  No ghost files found 🎉");
    } else {
        for g in analysis.ghosts.iter().take(10) {
            println!("  {} ({} days ago)", g.path, g.days_ago);
        }
    }

    // 커밋 분위기
    println!();
    println!("{}", "🎭 Commit Mood".bold().cyan());
    let mood = &analysis.mood;
    let moods = [
        ("Happy 😊", mood.happy),
        ("Stressed 😰", mood.stressed),
        ("Cleanup 🧹", mood.cleanup),
        ("Scary 💀", mood.scary),
        ("Other 🤷", mood.other),
    ];
    for (label, ratio) in &moods {
        let pct = ratio * 100.0;
        let bar = progress_bar(pct, 20);
        println!("  {:<16} {:>5.1}% {}", label, pct, bar);
    }

    // 버스 팩터
    println!();
    println!("{}", "🏋️ Bus Factor".bold().cyan());
    let bf = analysis.bus_factor;
    let bf_comment = match bf {
        0 => "❓ No data".to_string(),
        1 => format!("{} {}", bf, "🚨 Single point of failure!".red()),
        2 => format!("{} {}", bf, "⚠️ Risky".yellow()),
        3 => format!("{} {}", bf, "👍 Healthy".green()),
        _ => format!("{} {}", bf, "💪 Resilient".green()),
    };
    println!("  {}", bf_comment);

    // 활동 트렌드 (월별 바 그래프)
    println!();
    println!("{}", "📈 Activity Trend".bold().cyan());
    if analysis.monthly_activity.is_empty() {
        let trend = if score.trend_score > 70.0 {
            "📈 Rising".green().to_string()
        } else if score.trend_score >= 40.0 {
            "➡️ Stable".yellow().to_string()
        } else {
            "📉 Declining".red().to_string()
        };
        println!("  {}", trend);
    } else {
        let max_count = analysis.monthly_activity.iter()
            .map(|m| m.commit_count)
            .max()
            .unwrap_or(1);
        let bar_width = 20usize;
        let activity = &analysis.monthly_activity;
        for (i, month) in activity.iter().enumerate() {
            let filled = if max_count > 0 {
                ((month.commit_count as f64 / max_count as f64) * bar_width as f64).round() as usize
            } else {
                0
            };
            let bar = "█".repeat(filled);
            // 마지막 월에 추세 이모지 표시
            let trend_emoji = if i == activity.len() - 1 && activity.len() >= 2 {
                let prev = activity[i - 1].commit_count as f64;
                let curr = month.commit_count as f64;
                if prev > 0.0 {
                    let ratio = curr / prev;
                    if ratio >= 1.3 { " 📈" } else if ratio <= 0.7 { " 📉" } else { "" }
                } else {
                    ""
                }
            } else {
                ""
            };
            println!("  {}: {:>3} {}{}", month.label, month.commit_count, bar, trend_emoji);
        }
    }

    // 명예의 전당
    println!();
    println!("{}", "🏆 Hall of Fame".bold().cyan());
    if analysis.authors.is_empty() {
        println!("  No author data available");
    } else {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec!["", "Author", "Commits", "Added", "Removed"]);
        let limit = if leaderboard { analysis.authors.len() } else { 5 };
        for (i, a) in analysis.authors.iter().take(limit).enumerate() {
            let badge = match i {
                0 => "👑",
                1 => "🦾",
                2 => "🥉",
                _ => "🌱",
            };
            table.add_row(vec![
                badge.to_string(),
                a.name.clone(),
                a.commits.to_string(),
                a.lines_added.to_string(),
                a.lines_removed.to_string(),
            ]);
        }
        println!("{table}");
    }

    println!();
}
