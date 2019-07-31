//extern crate chrono;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Datelike, DateTime, Timelike, FixedOffset, TimeZone, Utc};

// CONVERSION OF THE LOCAL TIME TO UTC TIME
// Look at this: 
// https://github.com/chronotope/chrono

fn main() {
// The date you parsed
   let date = NaiveDate::from_ymd(2019, 6, 7); 
// The known time 
   let time = NaiveTime::from_hms(12, 22, 6);
   convert_local_to_utc(date, time, 2);
   let time = NaiveTime::from_hms(1, 15, 30);
   convert_local_to_utc(date, time, 2);
   let date = NaiveDate::from_ymd(2019, 6, 1);
   convert_local_to_utc(date, time, 2);
   let date = NaiveDate::from_ymd(2019, 6, 7);
   let time = NaiveTime::from_hms(12, 30, 00);
   convert_local_to_utc(date, time, -6);
   let time = NaiveTime::from_hms(18, 30, 00);
   convert_local_to_utc(date, time, -6);
   let date = NaiveDate::from_ymd(2019, 1, 1);
   let time = NaiveTime::from_hms(1, 59, 59);
   convert_local_to_utc(date, time, 2);
}

fn convert_local_to_utc(adate: NaiveDate, atime: NaiveTime, tz: i32) {
// Naive date time, with no time zone information
// The known tz hours time offset in seconds
   let tz_offset = FixedOffset::east(tz * 3600);
   let datetime = NaiveDateTime::new(adate, atime);
   let dt_with_tz: DateTime<FixedOffset> = tz_offset.from_local_datetime(&datetime).unwrap();
// If you need to convert it to a DateTime<Utc>, you can do this:
   let dt_with_tz_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());
   println!("UTC + {} {} to {}", tz, datetime, dt_with_tz_utc);
   println!("UTC date: {:.?}", get_date_elements(dt_with_tz_utc));
   println!("UTC time: {:.?}", get_time_elements(dt_with_tz_utc)); 
}

fn get_date_elements(dt: DateTime<Utc>) -> (i32, u32, u32) {
   (dt.year(), dt.month(), dt.day())
}

fn get_time_elements(dt: DateTime<Utc>) -> (u32, u32, u32) {
   (dt.hour(), dt.minute(), dt.second())
}
