use std::env;    
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, NaiveTime, Local};
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
use suncalcargs::solar::calc_f_azim;
use suncalcargs::solar::atmospheric_refraction;
use suncalcargs::solar::refr_correct_altitude;

struct Times {
    hour: u32,
    min:  u32,
    sec:  u32
}

struct Dates {
    year:  u32,
    month: u32,
    day:   u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program:    {}", args[0]);
    println!("Read {} arguments", args.len() - 1);
    let mut calc_time = Times {hour: 12, min: 22, sec: 6}; 
    let mut calc_date = Dates {year: 2019, month: 6, day: 7}; 

// Handle arguments hr, mn, ss
    if args.len() > 2 {
       println!("Argument 1: {} hr", args[1]);
       println!("Argument 2: {} mn", args[2]);
       println!("Argument 3: {} ss", args[3]);

       calc_time.hour = args[1].parse::<u32>().unwrap();
       calc_time.min  = args[2].parse::<u32>().unwrap();
       calc_time.sec  = args[3].parse::<u32>().unwrap();
   }

// Handle arguments year, month, day
    if args.len() > 5 {
       println!("Argument 4: {} year",  args[4]);
       println!("Argument 5: {} month", args[5]);
       println!("Argument 6: {} day",   args[6]);

       calc_date.year   = args[4].parse::<u32>().unwrap();
       calc_date.month  = args[5].parse::<u32>().unwrap();
       calc_date.day    = args[6].parse::<u32>().unwrap();
   }

    let (latitude, longitude, time_zone) = (65.85, 24.18, 2.0);
    println!("Location: latitude {} °, longitude {} °, time zone {} h", latitude, longitude, time_zone);
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    
    let mut my_day:u32 = 7; // day
    if calc_time.hour < time_zone as u32 {my_day -= 1 };

    let hr_utc: u32 = if calc_time.hour < time_zone as u32 {
      calc_time.hour + 24 - time_zone as u32 }
      else {calc_time.hour - time_zone as u32};

    let (year, month, day) = (calc_date.year, calc_date.month, my_day);
    let date_time: NaiveDateTime = NaiveDate::from_ymd(year as i32, month as u32, day as u32)
    .and_hms(calc_time.hour, calc_time.min, calc_time.sec);
    let my_jdn = f64::from(date_jdn(year as i32, month as i32, day as i32));
    println!("Local time now: {}", local_time.to_rfc2822());
    println!("Universal time now: {}", utc_time);
    println!("Calculation time is {}", date_time);

    println!("JDN = {}", my_jdn);
    jd_output(my_jdn, hr_utc as u32, calc_time.min as u32);

    let epoch = jd_epoch(my_jdn, hr_utc as u32, calc_time.min);
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
    let true_sol_time =  true_solar_time(calc_time.hour, calc_time.min, calc_time.sec, longitude, epoch, time_zone);
    let hr_angle = hour_angle(true_sol_time);
    let sun_zenith = solar_zenith_angle(latitude, my_declination, hr_angle);
    let sun_max_altitude: f64 = 90.0 - sun_zenith;
    let solar_azimuth: f64 = calc_f_azim(hr_angle, my_declination, latitude);
    let atmosfer_refract = atmospheric_refraction(sun_max_altitude); 
    let correct_height   = refr_correct_altitude(sun_zenith, sun_max_altitude);

 // println!("Epoch 2000                     =   {:.6}", epoch);
    println!("Declination                    =   {:.3} °", my_declination);
 // println!("HA Sunrise                     =   {:.3} °", ha_rise);
    println!("True solar time                =   {:.3} min", true_sol_time);
    println!("Hour angle                     = {:.3} °", hr_angle);
    println!("Solar zenith                   =   {:.3} °", sun_zenith);
    println!("Solar azimuth                  =   {:.3} °", solar_azimuth);
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

