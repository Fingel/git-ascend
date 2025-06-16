use git2::{Commit, Repository};

#[derive(Debug, Clone)]
pub struct CommitStats {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
    pub lines_added: usize,
    pub lines_deleted: usize,
}

pub fn collect_commit_stats_since(
    repo_path: &str,
    since_commit: &str,
) -> Result<Vec<CommitStats>, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let target_oid = repo.revparse_single(since_commit)?.id();

    let mut stats = Vec::new();
    let mut found_target = false;

    for oid in revwalk {
        let oid = oid?;

        if oid == target_oid {
            found_target = true;
            break;
        }

        let commit = repo.find_commit(oid)?;
        let (lines_added, lines_deleted) = calculate_commit_diff_stats(&repo, &commit)?;

        let commit_stats = CommitStats {
            sha: commit.id().to_string(),
            message: commit.message().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("Unknown").to_string(),
            timestamp: commit.time().seconds(),
            lines_added,
            lines_deleted,
        };

        stats.push(commit_stats);
    }
    if !found_target {
        return Err(git2::Error::from_str(&format!(
            "Target commit '{}' not found in history from HEAD",
            since_commit
        )));
    }

    Ok(stats)
}

fn calculate_commit_diff_stats(
    repo: &Repository,
    commit: &Commit,
) -> Result<(usize, usize), git2::Error> {
    let commit_tree = commit.tree()?;
    let parent_count = commit.parent_count();

    if parent_count == 0 {
        // Initial commit - diff against empty tree
        let diff = repo.diff_tree_to_tree(None, Some(&commit_tree), None)?;
        let stats = diff.stats()?;
        Ok((stats.insertions(), stats.deletions()))
    } else if parent_count == 1 {
        // Regular commit - use first parent
        let parent = commit.parent(0)?;
        let parent_tree = parent.tree()?;

        let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)?;
        let stats = diff.stats()?;
        Ok((stats.insertions(), stats.deletions()))
    } else {
        // Merge Commit
        Ok((0, 0))
    }
}
