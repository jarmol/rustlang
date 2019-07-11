extern crate chrono;
use chrono::{NaiveDate, NaiveTime, Local, DateTime, FixedOffset, Utc};
//use chrono::Utc;

fn main() {
     let mut day_fract: f64 = 0.25;
     println!("fract {:.9} => {:.?}", day_fract, get_hrmn(day_fract));
     day_fract = 0.250_347_223;
     println!("fract {:.9} => {:.?}", day_fract, get_hrmn(day_fract));
     day_fract = 0.250521;
     println!("fract {:.9} => {:.?}", day_fract, get_hrmn(day_fract));
     day_fract = 0.375;
     println!("fract {:.9} => {:.?}", day_fract, get_hrmn(day_fract));
     day_fract = 0.5;
     println!("fract {:.9} => {:.?}", day_fract, get_hrmn(day_fract));
     println!("Local date and time {} {} -> UTC date and time {:?}", NaiveDate::from_ymd(2019, 6, 7),
     NaiveTime::from_hms(1, 50, 0), utc_time_from_local(7, 1, 50, 2.0));
     println!("Local date and time {} {} -> UTC date and time {:?}", NaiveDate::from_ymd(2019, 6, 7),
     NaiveTime::from_hms(2, 05, 0), utc_time_from_local(7, 2, 05, 2.0));

     let local_time = Local::now(); // timezone  = 3 h = DLST
     let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
     let helsinki_timezone_normal = FixedOffset::east(2 * 3600);  // Manually forced wintertime tz 2 h
     println!("Local time now is {}", local_time);
     println!("UTC time now is {}", utc_time);
     println!(
        "Time in Helsinki now is {}",
        utc_time.with_timezone(&helsinki_timezone_normal));
  }

 // My new function, UTC time from local time near to midnight
fn utc_time_from_local(day: u32, hr: u32, mn: u32, tzone: f64) -> (NaiveDate, NaiveTime) {
        let utc_tuplet = if hr < tzone as u32 {(NaiveDate::from_ymd(2019, 6, day - 1), NaiveTime::from_hms(hr + 24 - tzone as u32, mn, 0))}
        else {(NaiveDate::from_ymd(2019, 6, day), NaiveTime::from_hms(hr - tzone as u32, mn, 00))};
        
     utc_tuplet   
    }

 fn get_hrmn(dayfract: f64) -> NaiveTime {
       let day_hours:   u32 = (24.0*dayfract) as u32;
       let day_minutes: u32 = (1440.0*dayfract % 60.0) as u32;
       let day_seconds: u32 = (86400.0*dayfract % 60.0) as u32;
    
       NaiveTime::from_hms(day_hours, day_minutes, day_seconds)
    }
