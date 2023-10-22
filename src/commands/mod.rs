use poise::Command;

use crate::State;

mod help;
mod ping;

pub fn get_commands() -> Vec<Command<State, anyhow::Error>> {
    vec![help::help(), ping::ping()]
}
