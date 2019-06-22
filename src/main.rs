use std::env;    
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Local};
use suncalc::julian::date_jdn;
use suncalc::julian::jd_epoch;
use suncalc::julian::utc_time_jd;
use suncalc::solar::sun_app_long;
use suncalc::solar::sunrise_ha;
use suncalc::solar::declination;
use suncalc::solar::noon_time;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program: {}", args[0]);
    let (latitude, longitude, time_zone) = (65.85, 24.18, 2.0);
    println!("Location: latitude {} °, longitude {} °", latitude, longitude);
    let local_time = Local::now();
    let (year, month, day) = (2019, 6, 7);
    let (hr, mn, ss) = (12, 22, 06);
    let date_time: NaiveDateTime = NaiveDate::from_ymd(year, month, day).and_hms(hr, mn, ss);
    let my_jdn = f64::from(date_jdn(year, month as i32, day as i32));
    println!("Time now: {}", local_time.to_rfc2822());
    println!("Calculation date and time is {}.", date_time);

    println!("JDN = {}", my_jdn);
    let hr_utc = hr - time_zone as u32;
    jd_output(my_jdn, hr_utc as i32, mn as i32);

    let epoch = jd_epoch(my_jdn, hr_utc as i32, 22);
    let sun_long = sun_app_long(epoch);
    let my_declination = declination(23.4359, sun_long); // except 22.74
    let ha_rise       =  sunrise_ha(latitude, my_declination);
    let daylen        =  get_hrmn(ha_rise/180.0);
    let noon_fraction =  noon_time(epoch, longitude, time_zone);
    let noon_time     =  get_hrmn(noon_fraction);
    let rise_fraction =  rise_set_time(noon_fraction, ha_rise);
    let rise_time     =  get_hrmn(rise_fraction);
    let set_fraction  =  rise_set_time(noon_fraction, -ha_rise);
    let set_time      =  get_hrmn(set_fraction);

 // println!("Epoch 2000 = {:.6}", epoch);
    println!("Declination            = {:.3} °", my_declination);
 // println!("HA Sunrise             = {:.3} °", ha_rise); // expect 166.75 deg
    println!("Day length             = {:.?}", daylen);
    println!("Sunrise time           = {:.?} ", rise_time);
    println!("Noon time              = {:.?}", noon_time);
    println!("Sunset time            = {:.?}", set_time);
  } // End of main

    fn get_hrmn(dayfract: f64) -> NaiveTime {
       let day_hours:   u32 = (   24.0*dayfract       ) as u32;
       let day_minutes: u32 = ( 1440.0*dayfract % 60.0) as u32;
       let day_seconds: u32 = (86400.0*dayfract % 60.0) as u32;
       NaiveTime::from_hms(day_hours, day_minutes, day_seconds)
    }

    fn rise_set_time(noon_time: f64, ha_angle: f64) -> f64 {
       noon_time - ha_angle/360.0
    }

  fn jd_output(jdn: f64, h: i32, mn: i32) {
     println!("UTC time: {}h {}min", h, mn);
     let x = utc_time_jd(jdn, h, mn);
     println!("JD = {:.4}", x);
  }

