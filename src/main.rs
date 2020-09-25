#[macro_use]
extern crate prettytable;

mod cli;
mod domain;
mod source;

use eyre::Result;

fn main() -> Result<()> {
    cli::process()
}
