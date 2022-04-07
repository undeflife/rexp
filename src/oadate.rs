use time::{
    format_description::{parse, FormatItem},
    macros::{date, time},
    Duration, OffsetDateTime, PrimitiveDateTime, Time,
};

// converting between OLE Automation date aka OADate  vs NaiveDateTime. we have ignored timezone differential
pub fn default_format() -> Vec<FormatItem<'static>> {
    parse("[year]/[month]/[day]").unwrap()
}

pub fn from_oadate(n: f64) -> PrimitiveDateTime {
    // let d = Date::try_from_ymd(1899, 12, 30).unwrap() + Duration::days(n as i64);
    let d = date!(1899 - 12 - 30) + Duration::days(n as i64);
    let t = Time::MIDNIGHT + Duration::milliseconds((n.fract() * 8.64e7) as i64);
    PrimitiveDateTime::new(d, t)
}

pub fn to_oadate(t: PrimitiveDateTime) -> f64 {
    let dt = PrimitiveDateTime::new(date!(1899 - 12 - 30), time!(0:00));
    (t - dt).as_seconds_f64() / 8.64e4
}

pub fn today() -> f64 {
    let d = OffsetDateTime::now_utc();
    to_oadate(PrimitiveDateTime::new(d.date(), d.time()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::format_description;
    #[test]
    fn test_oadate() {
        let dt = from_oadate(44237.1);
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        assert_eq!(dt.format(&format).unwrap(), "2021-02-10 02:23:59");
    }
}
