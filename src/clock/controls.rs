use chrono::{DateTime, TimeZone, Utc};

pub fn time() -> DateTime<Utc> {
    Utc.ymd(2222, 2, 22).and_hms(2, 22, 22)
}

pub fn rfc3339() -> String {
    String::from("2222-02-22T02:22:22+00:00")
}
