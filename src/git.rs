use anyhow::{Context, Result, anyhow};
use git2::{Commit, Repository};

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn new(repo_path: &str) -> Result<Self> {
        let repo = Repository::open(repo_path).context("Failed to open git repository")?;
        Ok(GitRepo { repo })
    }

    pub fn id(&self) -> Result<String> {
        first_commit_hash(&self.repo)
    }

    pub fn first_commit_hash(&self) -> Result<String> {
        first_commit_hash(&self.repo)
    }

    pub fn commits_since(&self, commit: &str) -> Result<Vec<CommitStats>> {
        collect_stats_since(&self.repo, commit)
    }

    pub fn head_commit_hash(&self) -> Result<String> {
        latest_commit_hash(&self.repo)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommitStats {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
    pub lines_added: usize,
    pub lines_deleted: usize,
}

fn collect_stats_since(repo: &Repository, from_commit: &str) -> Result<Vec<CommitStats>> {
    let mut revwalk = repo.revwalk()?;
    revwalk
        .push_range(&format!("{}..HEAD", from_commit))
        .context("Specified commit not found in this repository")?;

    revwalk
        .map(|oid| {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            let (lines_added, lines_deleted) = calculate_commit_diff_stats(repo, &commit)?;

            Ok(CommitStats {
                sha: commit.id().to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                timestamp: commit.time().seconds(),
                lines_added,
                lines_deleted,
            })
        })
        .collect()
}

fn latest_commit_hash(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Could not get HEAD reference")?;
    let head_oid = head.target().context("HEAD has no target")?;
    Ok(repo.find_commit(head_oid)?.id().to_string())
}

fn first_commit_hash(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Could not get HEAD reference")?;
    let head_oid = head.target().context("HEAD has no target")?;

    // Walk backwards from HEAD to find the root commit
    let mut current_oid = head_oid;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 100_000;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err(anyhow!(
                "Exceeded maximum iterations ({}) while searching for first commit.",
                MAX_ITERATIONS
            ));
        }
        let commit = repo.find_commit(current_oid)?;
        if commit.parent_count() == 0 {
            return Ok(commit.id().to_string());
        }
        // Follow the first parent (main line of development)
        current_oid = commit.parent_id(0)?;
    }
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
