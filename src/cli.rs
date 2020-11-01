use crate::domain::Source;
use crate::source::get_last_update_date;
use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use eyre::Result;
use prettytable::Table;

pub fn process() -> Result<()> {
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
                            Arg::with_name("target_url").takes_value(true),
                            Arg::with_name("check_url").takes_value(true),
                            Arg::with_name("type")
                                .possible_values(&["rss", "html"])
                                .takes_value(true),
                            Arg::with_name("datetime_format").takes_value(true),
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
        ("source", Some(matches)) => process_source(matches),
        ("check", Some(_)) => {
            for s in Source::repo()?.list()? {
                let curr = get_last_update_date(s)?;

                if let Some(prev) = s.last_at() {
                    if prev < curr {
                        println!("Source \"{}\" has updates! Check it here {}", s.name(), s.target_url())
                    }
                }

                if let Err(err) = Source::repo()?.save(&s.with_last_checked_at(curr)) {
                    eprintln!("Cannot save {}: {}", s.name(), err)
                }
            }
            Ok(())
        }
        _ => unreachable!(),
    }
}

fn process_source(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("add", Some(matches)) => Source::repo()?.save(&Source::new(
            matches.value_of("name").unwrap().into(),
            matches.value_of("target_url").unwrap().into(),
            matches.value_of("check_url").unwrap().into(),
            matches.value_of("type").unwrap().into(),
            matches.value_of("datetime_format").unwrap().into(),
            matches.value_of("offset").unwrap().parse().unwrap(),
        )),
        ("list", Some(_)) => {
            let mut table = Table::new();

            table.add_row(row![bFg => "NAME", "URL", "OFFSET", "TYPE", "LAST AT"]);
            Source::repo()?.list()?.iter().for_each(|v| {
                table.add_row(row![
                    v.name(),
                    v.check_url(),
                    v.offset(),
                    v.typ(),
                    match v.last_at() {
                        Some(date) => date.to_string(),
                        None => String::new(),
                    },
                ]);
            });
            table.printstd();
            Ok(())
        }
        ("show", Some(_)) => {
            println!("Show of sources");
            Ok(())
        }
        ("update", Some(_)) => {
            println!("Update of sources");
            Ok(())
        }
        ("delete", Some(_)) => {
            println!("Delete of sources");
            Ok(())
        }
        _ => unreachable!(),
    }
}
