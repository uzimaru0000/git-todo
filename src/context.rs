use crate::subcmd::{Cmd, SubCmd};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, author, about)]
pub struct Context {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

impl Context {
    pub fn run(&self) -> Result<()> {
        self.subcmd.run()
    }
}
