use std::{
    env,
    fs::OpenOptions,
    io::{Read, Write},
};

use anyhow::{Context, Ok, Result};
use clap::Parser;
use git2::{IndexAddOption, Repository};

use crate::todo::Todo;

use super::Cmd;

#[derive(Parser, Debug, Clone)]
#[clap(about = "Done task")]
pub struct DoneCmd {
    #[clap(help = "task id")]
    id: i32,
}

impl Cmd for DoneCmd {
    fn run(&self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        let repo = Repository::open(path)?;

        let todo_path = repo.path().join("TODO");
        let mut todo_file = OpenOptions::new().read(true).open(&todo_path)?;

        let mut content = String::new();
        todo_file.read_to_string(&mut content)?;
        let mut todos = Todo::prase(&content);

        let todo = todos
            .remove(&self.id)
            .with_context(|| format!("{} is not found", self.id))?;

        commit(&repo, &todo.title)?;

        let mut todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&todo_path)?;
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

fn commit(repo: &Repository, msg: &str) -> Result<()> {
    let signature = repo.signature()?;

    let tree = {
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;
        let oid = index.write_tree()?;
        repo.find_tree(oid)
    }?;

    let parent = {
        let head = repo.head()?;
        head.peel_to_commit()
    }?;

    repo.commit(Some("HEAD"), &signature, &signature, msg, &tree, &[&parent])?;
    Ok(())
}
