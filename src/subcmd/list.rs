use std::{env, fs::OpenOptions, io::Read};

use clap::Parser;
use git2::Repository;

use crate::todo::Todo;

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "List todo task")]
pub struct ListCmd {}

impl Cmd for ListCmd {
    fn run(&self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        let repo = Repository::open(path)?;

        let todo_path = repo.path().join("TODO");
        let mut todo_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(todo_path)?;

        let mut content = String::new();
        todo_file.read_to_string(&mut content)?;
        let mut todos = Todo::prase(&content).into_iter().collect::<Vec<_>>();
        todos.sort_by(|x, y| x.0.cmp(&y.0));
        todos
            .into_iter()
            .for_each(|(id, todo)| println!("#{} {}", id, todo.title));

        Ok(())
    }
}
