use std::{fs::OpenOptions, io::Write};

use anyhow::Result;
use clap::Parser;

use crate::repo::{open_repo, open_todo_file};

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "Add todo task")]
pub struct AddCmd {
    #[clap(help = "Todo title")]
    title: String,
}

impl Cmd for AddCmd {
    fn run(&self) -> Result<()> {
        let repo = open_repo()?;

        let mut opt = OpenOptions::new();
        opt.append(true).create(true);
        let mut todo_file = open_todo_file(&repo, &mut opt)?;

        todo_file.write(self.title.as_bytes())?;
        todo_file.write(b"\n")?;
        todo_file.flush()?;

        Ok(())
    }
}
