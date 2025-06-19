use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use std::fs::{self, File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn setup(repo_path: &str) -> Result<()> {
    println!("Setting up repository at {}", repo_path);
    create_post_commit_hook(repo_path)?;
    create_data_directory()?;
    Ok(())
}

pub fn check_setup(repo_path: &str) -> bool {
    let post_commit = format!("{}/.git/hooks/post-commit", repo_path);
    let post_commit_path = Path::new(&post_commit);
    if let Ok(data_dir) = data_location() {
        let data_dir_path = Path::new(&data_dir);
        post_commit_path.exists() && data_dir_path.exists()
    } else {
        false
    }
}

fn data_location() -> Result<String> {
    let path = ProjectDirs::from("io", "m51", "git-quest")
        .ok_or(anyhow!("Could not determine $HOME location"))?;
    let data_dir = path.data_dir();

    Ok(data_dir.to_string_lossy().into_owned())
}

fn create_post_commit_hook(repo_path: &str) -> Result<()> {
    let post_commit = format!("{}/.git/hooks/post-commit", repo_path);
    let post_commit_path = Path::new(&post_commit);

    if post_commit_path.exists() {
        // Don't do anything if the hook already exists, we don't want to clobber it
        println!("Post-commit hook already exists.");
    } else {
        println!("Creating post-commit hook in .git/hooks/post-commit");
        let hook_content = "#!/bin/sh\n# Git Quest post-commit hook\ngit quest\n";
        let mut file = File::create(post_commit_path).context("Error creating post-commit hook")?;
        file.write_all(hook_content.as_bytes())?;
        fs::set_permissions(post_commit_path, Permissions::from_mode(0o755))?;
    }
    Ok(())
}

fn create_data_directory() -> Result<()> {
    let data_dir = data_location()?;
    let data_dir_path = Path::new(&data_dir);

    if !data_dir_path.exists() {
        println!("Creating data directory in {}", data_dir);
        fs::create_dir_all(data_dir_path)?;
    }
    Ok(())
}
