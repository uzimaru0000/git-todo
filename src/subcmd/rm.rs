use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use git2::Repository;

use crate::todo::Todo;

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "Remove task")]
pub struct RemoveCmd {
    #[clap(help = "task id")]
    id: Option<i32>,
    #[clap(long)]
    all: bool,
}

impl RemoveCmd {
    fn clear(path: &PathBuf) -> anyhow::Result<File> {
        let file = OpenOptions::new().write(true).truncate(true).open(path)?;
        Ok(file)
    }
}

impl Cmd for RemoveCmd {
    fn run(&self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        let repo = Repository::open(path)?;
        let todo_path = repo.path().join("TODO");

        if self.all {
            Self::clear(&todo_path)?;
            return Ok(());
        }

        let id = self
            .id
            .with_context(|| "error: The following required arguments were not provided: <ID>")?;

        let mut todo_file = OpenOptions::new().read(true).open(&todo_path)?;

        let mut content = String::new();
        todo_file.read_to_string(&mut content)?;

        let mut todos = Todo::prase(&content);
        todos
            .remove(&id)
            .with_context(|| format!("{} is not found", id))?;

        let mut todo_file = Self::clear(&todo_path)?;
        let mut todos = todos.into_iter().collect::<Vec<_>>();
        todos.sort_by(|x, y| x.0.cmp(&y.0));
        let todo_data = todos
            .into_iter()
            .map(|(_, x)| x.title)
            .collect::<Vec<_>>()
            .join("\n");
        todo_file.write_all(todo_data.as_bytes())?;

        Ok(())
    }
}
