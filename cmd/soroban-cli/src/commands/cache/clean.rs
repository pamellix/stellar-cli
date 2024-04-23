use std::fs;

use super::super::config::locator;
use crate::commands::config::data;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] locator::Error),
    #[error(transparent)]
    Data(#[from] data::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, clap::Parser, Clone)]
#[group(skip)]
pub struct Cmd {}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        let binding = data::project_dir()?;
        let dir = binding.data_dir();
        fs::remove_dir_all(dir)?;
        Ok(())
    }
}