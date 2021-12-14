use add::AddCmd;
use clap::Subcommand;
use done::DoneCmd;
use list::ListCmd;
use rm::RemoveCmd;

mod add;
mod done;
mod list;
mod rm;

pub trait Cmd {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    Add(AddCmd),
    List(ListCmd),
    Done(DoneCmd),
    #[clap(name = "rm")]
    Remove(RemoveCmd),
}

impl Cmd for SubCmd {
    fn run(&self) -> anyhow::Result<()> {
        match &self {
            SubCmd::Add(add) => add.run(),
            SubCmd::List(list) => list.run(),
            SubCmd::Done(done) => done.run(),
            SubCmd::Remove(rm) => rm.run(),
        }
    }
}
