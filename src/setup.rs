use crate::ascii::{BANG, BANNER, display_side_by_side};
use crate::git::{GitRepo, git_username};
use crate::state::add_repo;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs::{self, File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::LazyLock;

static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("io", "m51", "git-ascend").expect("Could not determine $HOME location")
});

pub fn setup(repo_path: &str) -> Result<()> {
    println!("Setting up repository at {repo_path}");
    create_data_directory()?;
    create_post_commit_hook(repo_path)?;
    register_repository(repo_path)?;
    println!("Setup complete! Make a commit or run 'git ascend help' for more options.");
    Ok(())
}

pub fn first_run() -> bool {
    let data_dir = data_location();
    let data_dir_path = Path::new(&data_dir);
    !data_dir_path.exists()
}

pub fn check_setup(repo_path: &str) -> bool {
    let post_commit = format!("{repo_path}/.git/hooks/post-commit");
    let post_commit_path = Path::new(&post_commit);
    let data_dir = data_location();
    let data_dir_path = Path::new(&data_dir);
    post_commit_path.exists() && data_dir_path.exists()
}

pub fn data_location() -> String {
    let data_dir = PROJECT_DIRS.data_dir();
    data_dir.to_string_lossy().into_owned()
}

fn register_repository(repo_path: &str) -> Result<()> {
    let repo = GitRepo::new(repo_path)?;
    let first_commit = repo.first_commit_hash()?;
    let last_commit = repo.head_commit_hash()?;
    add_repo(first_commit, last_commit)?;
    Ok(())
}
fn create_post_commit_hook(repo_path: &str) -> Result<()> {
    let post_commit = format!("{repo_path}/.git/hooks/post-commit");
    let post_commit_path = Path::new(&post_commit);

    if post_commit_path.exists() {
        // Don't do anything if the hook already exists, we don't want to clobber it
        println!(
            "Post-commit hook already exists. Manually add 'git ascend' to the hook if it's not there already."
        );
    } else {
        let hook_content = "#!/bin/sh\n# Git Ascend post-commit hook\ngit ascend\n";
        let mut file = File::create(post_commit_path)
            .context("Error creating post-commit hook. Is this a git repository?")?;
        file.write_all(hook_content.as_bytes())?;
        fs::set_permissions(post_commit_path, Permissions::from_mode(0o755))?;
        println!("Created post-commit hook in .git/hooks/post-commit");
    }
    Ok(())
}

fn create_data_directory() -> Result<()> {
    let data_dir = data_location();
    let data_dir_path = Path::new(&data_dir);

    if !data_dir_path.exists() {
        println!("Creating data directory at {data_dir}");
        fs::create_dir_all(data_dir_path)?;
    }
    Ok(())
}

pub fn welcome_message() {
    let username = git_username().unwrap_or(String::from("developer"));
    let banner = display_side_by_side(&[BANNER, BANG], 1);
    println!();
    println!(
        "You are but a lowly 0x developer struggling to write a simple Hello World!
program,and failing completely at FizzBuzz. But it doesn't have to be this way
forever. With hard work and practice, you may one day reach the rank of a 10x,
100x, or even a 1000x developer. How far will you go? Today is the day you
begin to find out, for today is the first day of your"
    );
    println!("{banner}");
    println!("Welcome to the beginning of your ascension, {username}!");
    println!();
    println!("Run \x1b[1mgit ascend setup\x1b[0m to begin your journey.");
}
