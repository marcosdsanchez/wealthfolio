use chrono::{DateTime, Duration, NaiveDateTime, Utc};

/// Clamp offset to ±14 hours (840 minutes)
const MAX_OFFSET_MINUTES: i32 = 840;

fn clamp_offset(offset_minutes: i32) -> i32 {
    offset_minutes.clamp(-MAX_OFFSET_MINUTES, MAX_OFFSET_MINUTES)
}

/// Converts a UTC DateTime to a Local NaiveDateTime using the provided offset in minutes.
/// Convention: Local = UTC + offset_minutes
pub fn utc_to_local_with_offset(utc: DateTime<Utc>, offset_minutes: i32) -> NaiveDateTime {
    let offset = clamp_offset(offset_minutes);
    (utc + Duration::minutes(offset as i64)).naive_utc()
}

/// Converts a Local NaiveDateTime to a UTC DateTime using the provided offset in minutes.
/// Convention: UTC = Local - offset_minutes
pub fn local_to_utc_with_offset(local: NaiveDateTime, offset_minutes: i32) -> DateTime<Utc> {
    let offset = clamp_offset(offset_minutes);
    DateTime::<Utc>::from_naive_utc_and_offset(local - Duration::minutes(offset as i64), Utc)
}
