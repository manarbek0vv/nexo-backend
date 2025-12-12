use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub fn timestamp_to_datetime(ts: Option<Timestamp>) -> DateTime<Utc> {
    ts.map(|t| {
        DateTime::from_timestamp(t.seconds, t.nanos as u32)
            .unwrap_or_else(|| Utc::now())
    })
    .unwrap_or_else(|| Utc::now())
}