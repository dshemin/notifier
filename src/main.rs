#[macro_use]
extern crate prettytable;

mod cli;
mod domain;
mod infrastructure;

use crate::infrastructure::{ConnectionFactory, YamlSourceRepository};

fn main() {
    cli::process(
        &mut YamlSourceRepository::new("./list.yml".into()),
        ConnectionFactory {},
    )
}
