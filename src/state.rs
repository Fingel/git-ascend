use std::{collections::HashMap, fs::File, path::Path};

use crate::setup::data_location;
use anyhow::{Context, Result};
use bincode::{Decode, Encode, config};

#[derive(Encode, Decode, Debug)]
struct State {
    xp: usize,
    repos: HashMap<String, RepoState>,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct RepoState {
    pub last_commit: String,
}

impl State {
    fn new() -> Self {
        State {
            xp: 0,
            repos: HashMap::new(),
        }
    }
}

pub fn inc_xp(xp: usize) -> Result<usize> {
    let mut state = read_state()?;
    state.xp += xp;
    write_state(&state)?;
    Ok(state.xp)
}

pub fn read_xp() -> Result<usize> {
    let state = read_state()?;
    Ok(state.xp)
}

pub fn inc_last_commit(repo_id: &str, new_commit: &str) -> Result<()> {
    let mut state = read_state()?;
    if let Some(repo) = state.repos.get_mut(repo_id) {
        repo.last_commit = new_commit.to_string();
    }
    write_state(&state)?;
    Ok(())
}

pub fn add_repo(repo_id: String, last_commit: String) -> Result<()> {
    let mut state = read_state()?;
    state.repos.insert(repo_id, RepoState { last_commit });
    write_state(&state)
}

pub fn repo_state(repo_id: &str) -> Result<Option<RepoState>> {
    let state = read_state()?;
    let r_state = state.repos.get(repo_id);
    if let Some(repo) = r_state {
        Ok(Some(repo.clone()))
    } else {
        Ok(None)
    }
}

pub fn reset_xp() -> Result<()> {
    let mut state = read_state()?;
    state.xp = 0;
    write_state(&state)?;
    Ok(())
}

fn write_state(state: &State) -> Result<()> {
    let save_path = Path::new(&data_location()).join("state.bin");
    let mut file = File::create(save_path).context("Could not create state file")?;
    bincode::encode_into_std_write(state, &mut file, config::standard())?;
    Ok(())
}

fn read_state() -> Result<State> {
    let save_path = Path::new(&data_location()).join("state.bin");
    if !save_path.exists() {
        write_state(&State::new())?;
    }
    let mut file = File::open(save_path).context("Could not open state file")?;
    let state = bincode::decode_from_std_read(&mut file, config::standard())?;
    Ok(state)
}
