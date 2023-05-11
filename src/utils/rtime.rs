use chrono::{DateTime, Utc, FixedOffset};

pub fn utc_to_eastern_eight(date: DateTime<Utc>) -> DateTime<FixedOffset> {
    // Create a fixed offset timezone for China Standard Time (+8 hours)
    let cst_offset = FixedOffset::east_opt(8 * 3600).unwrap();

    // Convert the UTC date to the China Standard Time zone
    date.with_timezone(&cst_offset)
}

// convert datetime to string
pub fn dt_to_string(date: DateTime<FixedOffset>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}