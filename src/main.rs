use crate::git::collect_stats_since;
mod git;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = ".";
    let test_commit = "f4d8ea5fec43c835879ffc2d3c62337ef07f333a";
    let stats = collect_stats_since(repo_path, test_commit)?;
    println!("Found {} commits since the specified commit", stats.len());

    for stat in &stats {
        println!(
            "{} | {} | +{} -{} | {} | {}",
            &stat.sha[..8],
            stat.author,
            stat.lines_added,
            stat.lines_deleted,
            stat.message.lines().next().unwrap_or(""),
            stat.timestamp
        );
    }

    let total_added: usize = stats.iter().map(|s| s.lines_added).sum();
    let total_deleted: usize = stats.iter().map(|s| s.lines_deleted).sum();

    println!("\nTotals: +{} -{} lines", total_added, total_deleted);

    Ok(())
}
