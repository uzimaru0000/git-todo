use std::{fs::OpenOptions, io::Read};

use anyhow::{Ok, Result};
use clap::Parser;
use git2::{IndexAddOption, Repository};

use crate::{
    repo::{open_repo, open_todo_file},
    todo::Todo,
};

use super::Cmd;

#[derive(Parser, Debug, Clone)]
#[clap(about = "Done task")]
pub struct DoneCmd {
    #[clap(help = "task id")]
    id: i32,
}

impl Cmd for DoneCmd {
    fn run(&self) -> anyhow::Result<()> {
        let repo = open_repo()?;

        let mut opt = OpenOptions::new();
        opt.read(true);
        let mut todo_file = open_todo_file(&repo, &mut opt)?;

        let mut content = String::new();
        todo_file.read_to_string(&mut content)?;
        let mut todos = Todo::prase(&content);

        let todo = todos.remove(self.id)?;
        commit(&repo, &todo.title)?;
        todos.write_file(&repo)?;

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
