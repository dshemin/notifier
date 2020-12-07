use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    RSS,
    HTML,
}

impl From<&str> for SourceType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rss" => SourceType::RSS,
            "html" => SourceType::HTML,
            _ => unreachable!(),
        }
    }
}

impl ToString for SourceType {
    fn to_string(&self) -> String {
        match self {
            SourceType::RSS => "rss".to_owned(),
            SourceType::HTML => "html".to_owned(),
        }
    }
}
