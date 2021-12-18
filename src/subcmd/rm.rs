use std::{collections::HashMap, fs::OpenOptions, io::Read};

use anyhow::Context;
use clap::Parser;

use crate::{
    repo::{open_repo, open_todo_file},
    todo::{Todo, TodoList},
};

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "Remove task")]
pub struct RemoveCmd {
    #[clap(help = "task id")]
    id: Option<i32>,
    #[clap(long)]
    all: bool,
}

impl Cmd for RemoveCmd {
    fn run(&self) -> anyhow::Result<()> {
        let repo = open_repo()?;

        if self.all {
            TodoList(HashMap::new()).write_file(&repo)?;
            return Ok(());
        }

        let id = self
            .id
            .with_context(|| "error: The following required arguments were not provided: <ID>")?;

        let mut opt = OpenOptions::new();
        opt.read(true);
        let mut todo_file = open_todo_file(&repo, &mut opt)?;

        let mut content = String::new();
        todo_file.read_to_string(&mut content)?;

        let mut todos = Todo::prase(&content);
        todos.remove(id)?;
        todos.write_file(&repo)?;

        Ok(())
    }
}
