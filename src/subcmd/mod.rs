use add::AddCmd;
use clap::Subcommand;

pub mod add;

pub trait Cmd {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    Add(AddCmd),
}

impl Cmd for SubCmd {
    fn run(&self) -> anyhow::Result<()> {
        match &self {
            SubCmd::Add(add) => add.run(),
        }
    }
}
