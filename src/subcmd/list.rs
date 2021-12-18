use std::{fs::OpenOptions, io::Read};

use clap::Parser;

use crate::{
    repo::{open_repo, open_todo_file},
    todo::Todo,
};

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "List todo task")]
pub struct ListCmd {}

impl Cmd for ListCmd {
    fn run(&self) -> anyhow::Result<()> {
        let repo = open_repo()?;

        let mut opt = OpenOptions::new();
        opt.read(true).append(true).create(true);
        let mut todo_file = open_todo_file(&repo, &mut opt)?;

        let mut content = String::new();
        todo_file.read_to_string(&mut content)?;
        let mut todos = Todo::prase(&content).0.into_iter().collect::<Vec<_>>();
        todos.sort_by(|x, y| x.0.cmp(&y.0));
        todos
            .into_iter()
            .for_each(|(id, todo)| println!("#{} {}", id, todo.title));

        Ok(())
    }
}
