use std::{env, fs::OpenOptions, io::Write};

use anyhow::Result;
use clap::Parser;
use git2::Repository;

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "Add todo task")]
pub struct AddCmd {
    #[clap(help = "Todo title")]
    title: String,
}

impl Cmd for AddCmd {
    fn run(&self) -> Result<()> {
        let path = env::current_dir()?;
        let repo = Repository::open(path)?;

        let todo_path = repo.path().join("TODO");
        let mut todo_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(todo_path)?;

        todo_file.write(self.title.as_bytes())?;
        todo_file.write(b"\n")?;
        todo_file.flush()?;

        Ok(())
    }
}
