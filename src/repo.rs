use std::{
    env,
    fs::{File, OpenOptions},
    path::PathBuf,
};

use anyhow::Result;
use git2::Repository;

pub fn open_repo() -> Result<Repository> {
    let path = env::current_dir()?;
    let repo = Repository::open(path)?;

    Ok(repo)
}

pub fn open_todo_file(repo: &Repository, opt: &mut OpenOptions) -> Result<File> {
    let file = opt.open(todo_file_path(repo))?;

    Ok(file)
}

#[inline]
fn todo_file_path(repo: &Repository) -> PathBuf {
    repo.path().join("TODO")
}
