use chrono::offset::TimeZone;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // Utc.timestamp(seconds, millisconds)
    Utc.timestamp(start.timestamp() + i64::pow(10, 9), 0)
}
