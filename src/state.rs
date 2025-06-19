use std::{fs, path::Path};

use crate::setup::data_location;
use anyhow::Result;
use bincode::{Decode, Encode, config};

#[derive(Encode, Decode, Debug)]
struct State {
    xp: usize,
}

impl State {
    fn new() -> Self {
        State { xp: 0 }
    }
}

pub fn inc_xp(xp: usize) -> Result<usize> {
    let mut state = read_state()?;
    state.xp += xp;
    write_state(&state)?;
    Ok(state.xp)
}

pub fn reset_xp() -> Result<()> {
    write_state(&State::new())?;
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
