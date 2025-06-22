use anyhow::{Context, Result, anyhow};
use git2::{Commit, Config, Repository};

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn new(repo_path: &str) -> Result<Self> {
        let repo = Repository::open(repo_path).context("Failed to open git repository")?;
        Ok(GitRepo { repo })
    }

    pub fn id(&self) -> Result<String> {
        self.first_commit_hash()
    }

    pub fn first_commit_hash(&self) -> Result<String> {
        let head = self.repo.head().context("Could not get HEAD reference. If this is a fresh repo, ensure there is at least one commit.")?;
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
            let commit = self.repo.find_commit(current_oid)?;
            if commit.parent_count() == 0 {
                return Ok(commit.id().to_string());
            }
            // Follow the first parent (main line of development)
            current_oid = commit.parent_id(0)?;
        }
    }

    pub fn commits_since(&self, commit: &str) -> Result<Vec<CommitStats>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk
            .push_range(&format!("{}..HEAD", commit))
            .context("Specified commit not found in this repository")?;

        revwalk
            .map(|oid| {
                let oid = oid?;
                let commit = self.repo.find_commit(oid)?;
                let (lines_added, lines_deleted) =
                    calculate_commit_diff_stats(&self.repo, &commit)?;

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

    pub fn head_commit_hash(&self) -> Result<String> {
        let head = self.repo.head().context("Could not get HEAD reference")?;
        let head_oid = head.target().context("HEAD has no target")?;
        Ok(self.repo.find_commit(head_oid)?.id().to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommitStats {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub timestamp: i64,
    pub lines_added: u32,
    pub lines_deleted: u32,
}

/// Returns insertions, deletions for a given commit
fn calculate_commit_diff_stats(
    repo: &Repository,
    commit: &Commit,
) -> Result<(u32, u32), git2::Error> {
    let commit_tree = commit.tree()?;
    let parent_count = commit.parent_count();

    if parent_count == 0 {
        // Initial commit - diff against empty tree
        let diff = repo.diff_tree_to_tree(None, Some(&commit_tree), None)?;
        let stats = diff.stats()?;
        Ok((stats.insertions() as u32, stats.deletions() as u32))
    } else if parent_count == 1 {
        // Regular commit - use first parent
        let parent = commit.parent(0)?;
        let parent_tree = parent.tree()?;

        let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)?;
        let stats = diff.stats()?;
        Ok((stats.insertions() as u32, stats.deletions() as u32))
    } else {
        // Merge Commit
        Ok((0, 0))
    }
}

pub fn git_username() -> Result<String> {
    let config = Config::open_default().context("Failed to get global git config")?;
    let user_name = config
        .get_string("user.name")
        .context("Failed to get user.name from git config")?;
    Ok(user_name)
}
