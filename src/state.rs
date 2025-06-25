use anyhow::anyhow;
use std::{collections::HashMap, fs::File, path::Path};

use crate::{scaling::XpType, setup::data_location};
use anyhow::{Context, Result};
use bincode::{Decode, Encode, config};

#[derive(Encode, Decode, Debug)]
struct State {
    experience: Experience,
    current_stat: XpType,
    repos: HashMap<String, RepoState>,
}

#[derive(Encode, Decode, Debug)]
pub struct Experience {
    pub total: u128,
    pub precision: u128,
    pub output: u128,
    pub pedantry: u128,
    pub knowledge: u128,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct RepoState {
    pub last_recorded_commit: String,
}

impl State {
    fn new() -> Self {
        State {
            experience: Experience {
                total: 0,
                precision: 0,
                output: 0,
                pedantry: 0,
                knowledge: 0,
            },
            current_stat: XpType::Knowledge,
            repos: HashMap::new(),
        }
    }
}

pub fn inc_xp(amt: u128) -> Result<Experience> {
    let mut state = read_state()?;
    state.experience.total += amt;
    match state.current_stat {
        XpType::Total => {}
        XpType::Knowledge => state.experience.knowledge += amt,
        XpType::Precision => state.experience.precision += amt,
        XpType::Output => state.experience.output += amt,
        XpType::Pedantry => state.experience.pedantry += amt,
    }
    write_state(&state)?;
    Ok(state.experience)
}

pub fn read_xp() -> Result<Experience> {
    let state = read_state()?;
    Ok(state.experience)
}

pub fn inc_last_commit(repo_id: &str, new_commit: &str) -> Result<()> {
    let mut state = read_state()?;
    if let Some(repo) = state.repos.get_mut(repo_id) {
        repo.last_recorded_commit = new_commit.to_string();
    }
    write_state(&state)?;
    Ok(())
}

pub fn add_repo(repo_id: String, last_recorded_commit: String) -> Result<()> {
    let mut state = read_state()?;
    state.repos.insert(
        repo_id,
        RepoState {
            last_recorded_commit,
        },
    );
    write_state(&state)
}

pub fn repo_state(repo_id: &str) -> Result<RepoState> {
    let state = read_state()?;
    let r_state = state.repos.get(repo_id);
    if let Some(repo) = r_state {
        Ok(repo.clone())
    } else {
        Err(anyhow!(
            "Repository not found in state. Was setup successful?"
        ))
    }
}

pub fn reset_xp() -> Result<()> {
    let mut state = read_state()?;
    state.experience = Experience {
        total: 0,
        precision: 0,
        output: 0,
        pedantry: 0,
        knowledge: 0,
    };
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

pub fn set_current_stat(stat: XpType) -> Result<()> {
    if stat == XpType::Total {
        return Err(anyhow!("Cannot set current stat to Total"));
    }
    let mut state = read_state()?;
    state.current_stat = stat;
    write_state(&state)?;
    Ok(())
}

pub fn read_current_stat() -> Result<XpType> {
    let state = read_state()?;
    Ok(state.current_stat)
}
