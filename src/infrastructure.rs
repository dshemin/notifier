extern crate serde_yaml;

use crate::domain::{Connection, Source, SourceIterator, SourceRepository, SourceType};
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use eyre::{Report, Result};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fs::File;
use std::ops::Deref;
use syndication::Feed;

pub struct YamlSourceRepository {
    path: String,
    map: HashMap<String, Source>,
}

impl YamlSourceRepository {
    pub fn new(path: String) -> YamlSourceRepository {
        YamlSourceRepository {
            path,
            map: HashMap::new(),
        }
    }

    fn load(&mut self) -> Result<()> {
        self.map = serde_yaml::from_reader(File::open(&self.path)?)?;
        Ok(())
    }

    fn dump(&self) -> Result<()> {
        serde_yaml::to_writer(File::open(&self.path)?, &self.map)?;
        Ok(())
    }
}

impl<'a> SourceRepository for YamlSourceRepository {
    type Iter = Iter<'a, String, Source>;
    type Mapper = fn((&String, &Source)) -> Source;

    fn save(&mut self, s: &Source) -> Result<(), Report> {
        self.load()?;
        self.map.insert(s.name().clone(), (*s).clone());
        Ok(())
    }

    fn list(&mut self) -> Result<SourceIterator<Self::Iter, Self::Mapper>> {
        self.load()?;
        Ok(SourceIterator::new(self.map.iter(), |(_, v)| v.clone()))
    }
}

pub struct ConnectionFactory();

impl ConnectionFactory {
    pub fn create(&self, s: &Source) -> Result<Box<dyn Connection>> {
        match s.typ() {
            SourceType::RSS => Ok(Box::new(RSSConnection {
                url: s.url().clone(),
                datetime_offset: FixedOffset::east(s.offset().clone()),
            })),
            SourceType::HTML => unimplemented!(), // todo add support of HTML sources
        }
    }
}

pub struct RSSConnection {
    url: String,
    // datetime_format: String,
    datetime_offset: FixedOffset,
}

impl Connection for RSSConnection {
    fn get_new(&self) -> Result<DateTime<Utc>> {
        reqwest::blocking::get(&self.url)
            .map(|resp| {
                let published_at: String = match resp.text().unwrap().parse::<Feed>().unwrap() {
                    Feed::Atom(feed) => feed.entries().first().unwrap().published().unwrap().into(),
                    Feed::RSS(feed) => feed.items().first().unwrap().pub_date().unwrap().into(),
                };

                let date = NaiveDateTime::parse_from_str(published_at.deref(), "%F %T").unwrap();
                self.datetime_offset
                    .from_local_datetime(&date)
                    .unwrap()
                    .with_timezone(&Utc)
            })
            .map_err(|err| Report::new(err))
    }
}
