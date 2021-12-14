use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn prase(s: &str) -> HashMap<i32, Self> {
        s.split('\n')
            .map(|x| x.trim())
            .filter(|&x| !x.is_empty())
            .zip(0..)
            .map(|(title, id)| (id, Self::new(title.trim().to_string())))
            .collect()
    }
}
