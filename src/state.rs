use std::{collections::HashMap, fs, path::Path};

use crate::setup::data_location;
use anyhow::Result;
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
    println!("REPO ID: {}", repo_id);
    println!("{:?}", state.repos);
    let r_state = state.repos.get(repo_id);
    if let Some(repo) = r_state {
        println!("FOUND REPO");
        Ok(Some(repo.clone()))
    } else {
        println!("COULD NOT FIND REPO");
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
    let encoded = bincode::encode_to_vec(state, config::standard())?;
    let save_path = Path::new(&data_location()).join("state.bin");
    fs::write(save_path, encoded)?;
    Ok(())
}

fn read_state() -> Result<State> {
    let save_path = Path::new(&data_location()).join("state.bin");
    if !save_path.exists() {
        write_state(&State::new())?;
    }
    let encoded = fs::read(save_path)?;
    let (state, _): (State, usize) = bincode::decode_from_slice(&encoded, config::standard())?;
    Ok(state)
}
