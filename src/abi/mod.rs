use prost_types::Timestamp;

pub mod content;
pub mod post;
pub mod service;
pub mod user;

pub(crate) fn now_ts() -> Timestamp {
    let now = chrono::Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}
