use crate::domain::{Source, SourceRepository};
use crate::infrastructure::ConnectionFactory;
use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use prettytable::Table;

pub fn process<R: SourceRepository>(repo: &mut R, factory: ConnectionFactory) {
    let matches = App::new("notifier")
        .about("Search and notify when something is change in the your sources")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("source")
                .about("Manage sources")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommands(vec![
                    SubCommand::with_name("add")
                        .about("Add new source")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .args(&[
                            Arg::with_name("name").takes_value(true),
                            Arg::with_name("url").takes_value(true),
                            Arg::with_name("type")
                                .possible_values(&["rss", "html"])
                                .takes_value(true),
                            Arg::with_name("offset").takes_value(true),
                        ]),
                    App::new("list").about("List all sources"),
                    App::new("show").about("Show source details"),
                    App::new("update").about("Update source"),
                    App::new("delete").about("Delete source"),
                ]),
            SubCommand::with_name("check").about("Check sources for new information"),
        ])
        .get_matches();

    match matches.subcommand() {
        ("source", Some(matches)) => {
            process_source(matches, repo);
        }
        ("check", Some(_)) => {
            match repo.list() {
                Ok(sources) => {
                    sources.for_each(|s| {
                        let curr = factory.create(&s).unwrap().get_new().unwrap();

                        if let Some(prev) = s.last_checked_at() {
                            if prev < curr {
                                println!("Source \"{}\" has updates!", s.name())
                            }
                        }

                        if let Err(err) = repo.save(&s.with_last_checked_at(curr)) {
                            eprintln!("Cannot save {}: {}", s.name(), err)
                        }
                    });
                }
                _ => eprintln!("{}", "Cannot get list of sources".red()),
            };
        }
        _ => unreachable!(),
    }
}

fn process_source<R: SourceRepository>(matches: &ArgMatches, repo: &mut R) {
    match matches.subcommand() {
        ("add", Some(matches)) => {
            match repo.save(&Source::new(
                matches.value_of("name").unwrap().into(),
                matches.value_of("url").unwrap().into(),
                matches.value_of("type").unwrap().into(),
                matches.value_of("offset").unwrap().parse().unwrap(),
            )) {
                Ok(_) => println!("{}", "New source was added!".green()),
                _ => eprintln!("{}", "Cannot add new source".red()),
            }
        }
        ("list", Some(_)) => match repo.list() {
            Ok(sources) => {
                let mut table = Table::new();

                table.add_row(row![bFg => "NAME", "URL", "OFFSET", "TYPE", "CHECKED AT"]);
                sources.for_each(|v| {
                    table.add_row(row![
                        v.name(),
                        v.url(),
                        v.offset(),
                        v.typ(),
                        match v.last_checked_at() {
                            Some(date) => date.to_string(),
                            None => String::new(),
                        },
                    ]);
                });
                table.printstd();
            }
            _ => eprintln!("{}", "Cannot get list of sources".red()),
        },
        ("show", Some(_)) => println!("Show of sources"),
        ("update", Some(_)) => println!("Update of sources"),
        ("delete", Some(_)) => println!("Delete of sources"),
        _ => unreachable!(),
    }
}
