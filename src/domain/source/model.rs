use crate::domain::source::repository::SourceRepository;
use crate::domain::source::source_type::SourceType;
use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    name: String,
    target_url: String,
    check_url: String,
    typ: SourceType,

    /// Date time offset of the source.
    /// Necessary for parsing dates from that source
    offset: i32,

    datetime_format: String,
    last_at: Option<DateTime<Utc>>,
}

impl Source {
    pub fn new(
        name: String,
        target_url: String,
        check_url: String,
        typ: SourceType,
        datetime_format: String,
        offset: i32,
    ) -> Source {
        Source {
            name,
            target_url,
            check_url,
            typ,
            offset,
            datetime_format,
            last_at: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn target_url(&self) -> &String {
        &self.target_url
    }
    pub fn check_url(&self) -> &String {
        &self.check_url
    }
    pub fn typ(&self) -> &SourceType {
        &self.typ
    }
    pub fn offset(&self) -> &i32 {
        &self.offset
    }
    pub fn datetime_format(&self) -> &String {
        &self.datetime_format
    }
    pub fn last_at(&self) -> Option<DateTime<Utc>> {
        self.last_at
    }

    pub fn with_last_checked_at(&self, date: DateTime<Utc>) -> Source {
        Source {
            name: self.name.clone(),
            target_url: self.target_url.clone(),
            check_url: self.check_url.clone(),
            typ: self.typ,
            offset: self.offset,
            datetime_format: self.datetime_format.clone(),
            last_at: Some(date),
        }
    }

    pub fn repo() -> Result<Box<SourceRepository>> {
        // fixme here we will create a new repo every time when this method will call. Should think about that ...
        Ok(Box::new(SourceRepository::new("./list.yml".to_string())?))
    }
}
