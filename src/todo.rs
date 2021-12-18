use std::{collections::HashMap, fs::OpenOptions, io::Write};

use anyhow::{Context, Result};
use git2::Repository;

use crate::repo::open_todo_file;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn prase(s: &str) -> TodoList {
        TodoList(
            s.split('\n')
                .map(|x| x.trim())
                .filter(|&x| !x.is_empty())
                .zip(0..)
                .map(|(title, id)| (id, Self::new(title.trim().to_string())))
                .collect(),
        )
    }
}

#[derive(Clone)]
pub struct TodoList(pub HashMap<i32, Todo>);

impl TodoList {
    pub fn write_file(&self, repo: &Repository) -> Result<()> {
        let mut opt = OpenOptions::new();
        opt.write(true).truncate(true);
        let mut file = open_todo_file(repo, &mut opt)?;

        let mut todos = self.clone().0.into_iter().collect::<Vec<_>>();
        todos.sort_by(|x, y| x.0.cmp(&y.0));
        let todo_data = todos
            .into_iter()
            .map(|(_, x)| x.title)
            .collect::<Vec<_>>()
            .join("\n");
        file.write_all(todo_data.as_bytes())?;

        Ok(())
    }

    pub fn remove(&mut self, id: i32) -> Result<Todo> {
        self.0
            .remove(&id)
            .with_context(|| format!("{} is not found", id))
    }
}
