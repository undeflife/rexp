use chrono::prelude::{NaiveDate, NaiveDateTime};
use std::time::SystemTime;

// converting between OLE Automation date aka OADate vs NaiveDateTime.

pub fn from_oadate(n: f64) -> NaiveDateTime {
    let offset = n as i64 - 25569;
    NaiveDateTime::from_timestamp(offset * 86400, 0)
}

pub fn to_oadate(d: NaiveDateTime) -> f64 {
    let epoch = NaiveDate::from_ymd(1899, 12, 30).and_hms(0, 0, 0);
    d.signed_duration_since(epoch).num_days() as f64
}

pub fn today() -> f64 {
    if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        to_oadate(NaiveDateTime::from_timestamp(n.as_secs() as i64, 0))
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::oadate;

    #[test]
    fn test_oadate() {
        let dt = oadate::from_oadate(44237.0);
        assert_eq!(
            dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2021-02-10 00:00:00"
        );
        let d = oadate::to_oadate(dt);
        assert_eq!(d, 44237.0);
    }
}
