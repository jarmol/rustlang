use std::env;    
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Local};
use suncalcargs::julian::date_jdn;
use suncalcargs::julian::jd_epoch;
use suncalcargs::julian::utc_time_jd;
use suncalcargs::solar::sun_app_long;
use suncalcargs::solar::sunrise_ha;
use suncalcargs::solar::declination;
use suncalcargs::solar::noon_time;
use suncalcargs::solar::true_solar_time;
use suncalcargs::solar::hour_angle;
use suncalcargs::solar::solar_zenith_angle;
use suncalcargs::solar::atmospheric_refraction;
use suncalcargs::solar::refr_correct_altitude;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program:    {}", args[0]);
    println!("Read {} arguments", args.len() - 1);
    let mut v1:u32 = 12;
    let mut v2:u32 = 22;

// Handle arguments hr, mn, ss
    if args.len() > 2 {
       println!("Argument 1: {} hr", args[1]);
       println!("Argument 2: {} mn", args[2]);
       v1 = args[1].parse::<u32>().unwrap();
       v2 = args[2].parse::<u32>().unwrap();
   }

    let (latitude, longitude, time_zone) = (65.85, 24.18, 2.0);
    println!("Location: latitude {} °, longitude {} °, time zone {} h", latitude, longitude, time_zone);
    let local_time = Local::now();

    let mut v3:u32 = 7; // day
    if v1 < time_zone as u32 {
       v1 += 24;
       v3 -= 1;
    }

    let (year, month, day) = (2019, 6, v3);
    let (hr, mn, ss) = (v1, v2, 00);
    let date_time: NaiveDateTime = NaiveDate::from_ymd(year, month, day).and_hms(hr, mn, ss);
    let my_jdn = f64::from(date_jdn(year, month as i32, day as i32));
    println!("Time now: {}", local_time.to_rfc2822());
    println!("Calculation date and time is {}.", date_time);

    println!("JDN = {}", my_jdn);
    let hr_utc: u32 = hr - time_zone as u32;
    jd_output(my_jdn, hr_utc as u32, mn as u32);

    let epoch = jd_epoch(my_jdn, hr_utc as u32, mn);
    let sun_long = sun_app_long(epoch);
    let my_declination = declination(23.4359, sun_long);
    let ha_rise       =  sunrise_ha(latitude, my_declination);
    let daylen        =  get_hrmn(ha_rise/180.0);
    let noon_fraction =  noon_time(epoch, longitude, time_zone);
    let noon_time     =  get_hrmn(noon_fraction);
    let rise_fraction =  rise_set_time(noon_fraction, ha_rise);
    let rise_time     =  get_hrmn(rise_fraction);
    let set_fraction  =  rise_set_time(noon_fraction, -ha_rise);
    let set_time      =  get_hrmn(set_fraction);
    let true_sol_time =  true_solar_time(hr, mn, ss, longitude, epoch, time_zone);
    let hr_angle = hour_angle(true_sol_time);
    let sun_zenith = solar_zenith_angle(latitude, my_declination, hr_angle);
    let sun_max_altitude: f64 = 90.0 - sun_zenith;
    let atmosfer_refract = atmospheric_refraction(sun_max_altitude); 
    let correct_height   = refr_correct_altitude(sun_zenith, sun_max_altitude);

 // println!("Epoch 2000                     =   {:.6}", epoch);
    println!("Declination                    =   {:.3} °", my_declination);
 // println!("HA Sunrise                     =   {:.3} °", ha_rise);
    println!("True solar time                =   {:.3} min", true_sol_time);
    println!("Hour angle                     = {:.3} °", hr_angle);
    println!("Solar zenith                   =   {:.3} °", sun_zenith);
    println!("Sun altitude                   =    {:.3} °", sun_max_altitude);
    println!("Atmospheric refraction         =    {:.3} °", atmosfer_refract);
    println!("Refraction corrected elevation =    {:.3} °", correct_height);
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

  fn jd_output(jdn: f64, h: u32, mn: u32) {
     println!("UTC time: {}h {}min", h, mn);
     let x = utc_time_jd(jdn, h, mn);
     println!("JD = {:.4}", x);
  }
