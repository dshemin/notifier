mod rss;

use crate::domain::{Source, SourceType};
use chrono::{DateTime, FixedOffset, Utc};
use eyre::Result;

pub fn get_last_update_date(src: &Source) -> Result<DateTime<Utc>> {
    match src.typ() {
        SourceType::RSS => rss::get_last_update_date(
            src.check_url(),
            src.datetime_format(),
            FixedOffset::east(src.offset().clone()),
        ),
        SourceType::HTML => unimplemented!(),
    }
}
