use anyhow::Result;
use clap::Parser;
use git_todo::context::Context;

fn main() -> Result<()> {
    let context = Context::parse();
    context.run()
}
