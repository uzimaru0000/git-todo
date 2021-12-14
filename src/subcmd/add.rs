use anyhow::Result;
use clap::Parser;

use super::Cmd;

#[derive(Debug, Parser, Clone)]
#[clap(about = "Add todo task")]
pub struct AddCmd {
    #[clap(help = "Todo title")]
    title: String,
}

impl Cmd for AddCmd {
    fn run(&self) -> Result<()> {
        todo!("todo")
    }
}
