// amj_date.rs

use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

fn get_datetime() -> OffsetDateTime {
	let now = SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_secs();
	OffsetDateTime::from_unix_timestamp(now as i64).unwrap()
}

pub fn get_date() -> String {
	get_datetime().date().to_string()
}

pub fn get_annee() -> String {
	get_datetime().year().to_string()
}
