use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use eyre::{Report, Result};
use syndication::Feed;

pub fn get_last_update_date(
    url: &str,
    datetime_format: &str,
    datetime_offset: FixedOffset,
) -> Result<DateTime<Utc>> {
    match reqwest::blocking::get(url) {
        Ok(resp) => {
            let published_at: String = match resp.text().unwrap().parse::<Feed>().unwrap() {
                Feed::Atom(feed) => feed.entries().first().unwrap().published().unwrap().into(),
                Feed::RSS(feed) => feed.items().first().unwrap().pub_date().unwrap().into(),
            };

            let date = NaiveDateTime::parse_from_str(published_at.as_str(), datetime_format)?;
            Ok(datetime_offset
                .from_local_datetime(&date)
                .unwrap()
                .with_timezone(&Utc))
        }
        Err(err) => Err(Report::new(err)),
    }
}
