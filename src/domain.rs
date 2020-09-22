use chrono::prelude::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum SourceType {
    RSS,
    HTML,
}

impl FromStr for SourceType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rss" => Ok(SourceType::RSS),
            "html" => Ok(SourceType::HTML),
            _ => Err("no match"),
        }
    }
}

impl From<&str> for SourceType {
    fn from(s: &str) -> Self {
        match s {
            "rss" => SourceType::RSS,
            "html" => SourceType::HTML,
            _ => unreachable!(),
        }
    }
}

impl ToString for SourceType {
    fn to_string(&self) -> String {
        match self {
            SourceType::RSS => "rss".into(),
            SourceType::HTML => "html".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    name: String,
    url: String,
    typ: SourceType,

    /// Date time offset of the source.
    /// Necessary for parsing dates from that source
    offset: i32,
    last_checked_at: Option<DateTime<Utc>>,
}

impl Source {
    pub fn new(name: String, url: String, typ: SourceType, offset: i32) -> Source {
        Source {
            name,
            url,
            typ,
            offset,
            last_checked_at: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn url(&self) -> &String {
        &self.url
    }
    pub fn typ(&self) -> &SourceType {
        &self.typ
    }
    pub fn offset(&self) -> &i32 {
        &self.offset
    }
    pub fn last_checked_at(&self) -> Option<DateTime<Utc>> {
        self.last_checked_at
    }

    pub fn with_last_checked_at(&self, date: DateTime<Utc>) -> Source {
        Source {
            name: self.name.clone(),
            url: self.url.clone(),
            typ: self.typ,
            offset: self.offset,
            last_checked_at: Some(date),
        }
    }
}

pub struct SourceIterator<I, M> {
    iter: I,
    mapper: M,
}

impl<I, M> SourceIterator<I, M> {
    pub fn new(iter: I, mapper: M) -> SourceIterator<I, M> {
        SourceIterator { iter, mapper }
    }
}

impl<I, M> Iterator for SourceIterator<I, M>
where
    I: Iterator,
    M: FnMut(I::Item) -> Source,
{
    type Item = Source;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.mapper)
    }
}

pub trait SourceRepository {
    type Iter: Iterator;
    type Mapper: FnMut(<<Self as SourceRepository>::Iter as Iterator>::Item) -> Source;

    fn save(&mut self, s: &Source) -> Result<()>;
    fn list(&mut self) -> Result<SourceIterator<Self::Iter, Self::Mapper>>;
}

pub trait Connection {
    fn get_new(&self) -> Result<DateTime<Utc>>;
}
