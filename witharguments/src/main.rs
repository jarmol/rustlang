use std::env; 
use chrono::{DateTime, Utc, NaiveDate, Datelike, NaiveDateTime, NaiveTime, Timelike, FixedOffset};
use chrono::offset::TimeZone;
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
    year:  i32,
    month: u32,
    day:   u32
}

struct Location {
   latitude:  f64,
   longitude: f64,
   timezone:  f64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program:    {}", args[0]);
    println!("Read {} arguments", args.len() - 1);
    let mut calc_time = Times {hour: 12, min: 22, sec: 6}; 
    let mut calc_date = Dates {year: 2019, month: 6, day: 7}; 
    let mut my_location = Location {latitude: 65.85, longitude: 24.18, timezone: 2.0};

// Handle arguments hr, mn, ss
    if args.len() > 2 {
       let names = ["hours", "min", "sec"];
       println!("Arguments: {}, {}, {}", names[0], names[1], names[2]);

    for i in 1..4 {
         println!("Argument {}: {} {}", i, args[i], names[i - 1]);}

       calc_time.hour = args[1].parse::<u32>().unwrap();
       calc_time.min  = args[2].parse::<u32>().unwrap();
       calc_time.sec  = args[3].parse::<u32>().unwrap();
   }

// Handle arguments year, month, day
    if args.len() > 4 {
       let names = ["year", "month", "day"];
       println!("\nArguments: {}, {}, {}", names[0], names[1], names[2]);

    for i in 4..7 {
        if args.len() > i { println!("Argument {}: {} {}", i, args[i], names[i - 4])}
        else {panic!("Missing argument {}", names[i - 4]);};}

       calc_date.year   = args[4].parse::<i32>().unwrap();
       calc_date.month  = args[5].parse::<u32>().unwrap();
       calc_date.day    = args[6].parse::<u32>().unwrap();
   }

// Handle arguments latitude, longitude and timezone
    if args.len() > 7 {
       let names = ["latitude", "longitude", "timezone"];
       println!("\nArguments: {}, {}, {}", names[0], names[1], names[2]);

     for i in 7..10 {
       if args.len() > i {println!("Argument {}: {} {}", i, args[i], names[i - 7]);}  
     }

       my_location.latitude = args[7].parse::<f64>().unwrap();

       if args.len() > 8 { my_location.longitude = args[8].parse::<f64>().unwrap();}
       else {panic!("Longitude missing!");}

       if args.len() > 9 { my_location.timezone = args[9].parse::<f64>().unwrap();}
       else {panic!("Timezone missing!");}
     }

    let (latitude, longitude, time_zone) = (my_location.latitude, my_location.longitude, my_location.timezone);
//    println!("Location: latitude {} °, longitude {} °, time zone {} h", latitude, longitude, time_zone);
//    let local_time = Local::now();
    let date = NaiveDate::from_ymd(calc_date.year, calc_date.month, calc_date.day);
    let time = NaiveTime::from_hms(calc_time.hour, calc_time.min, calc_time.sec);
    let get_utc_dt = convert_local_to_utc(date, time, time_zone);
    let day_utc   = get_utc_dt.2;
    let month_utc = get_utc_dt.1;
    let year_utc  = get_utc_dt.0;

    let hr_utc    = get_utc_dt.3;
//    let mn_utc    = get_utc_dt.4;
//    println!("Testing day.month.year  UTC {}.{}. {} tz {} h", day_utc, month_utc, year_utc, time_zone);
//    println!(" time UTC {}:{}", hr_utc, calc_time.min);

    let (year, month, day) = (calc_date.year, calc_date.month, calc_date.day);
    let date_time: NaiveDateTime = NaiveDate::from_ymd(year as i32, month as u32, day as u32)
    .and_hms(calc_time.hour, calc_time.min, calc_time.sec);
    let utc_date_time: NaiveDateTime = NaiveDate::from_ymd(year_utc as i32, month_utc, day_utc as u32)
    .and_hms(hr_utc,         calc_time.min, calc_time.sec);
 
   let my_jdn = f64::from(date_jdn(year_utc as i32, month_utc as i32, day_utc as i32));
 //   println!("Local time now: {}", local_time.to_rfc2822());
 //   println!("Universal time now: {}", utc_time);
    println!("Calculation local time {}", date_time);
    println!("Calculation   UTC time {}", utc_date_time);  
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
    println!("Declination                    =   {:7.3} °", my_declination);
 // println!("HA Sunrise                     =   {:7.3} °", ha_rise);
    println!("True solar time                =   {:7.3} min", true_sol_time);
    println!("Hour angle                     =   {:7.3} °", hr_angle);
    println!("Solar zenith                   =   {:7.3} °", sun_zenith);
    println!("Solar azimuth                  =   {:7.3} °", solar_azimuth);
    println!("Sun altitude                   =   {:7.3} °", sun_max_altitude);
    println!("Atmospheric refraction         =   {:7.3} °", atmosfer_refract);
    println!("Refraction corrected elevation =   {:7.3} °", correct_height);
    let filler = "          ";
    println!("Day length        {}   =    {:.?}", filler, daylen);
    println!("Sunrise time      {}   =    {:.?}", filler, rise_time);
    println!("Noon time         {}   =    {:.?}", filler, noon_time);
    println!("Sunset time       {}   =    {:.?}", filler, set_time);
  } // End of main

    fn get_hrmn(dayfract: f64) -> NaiveTime {
       let day_hours:   u32 = (   24.0*dayfract % 24.0) as u32;
       let day_minutes: u32 = ( 1440.0*dayfract % 60.0) as u32;
       let day_seconds: u32 = (86400.0*dayfract % 60.0) as u32;
// format!("Test {}:{}:{}", day_hours, day_minutes, day_seconds)
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

fn convert_local_to_utc(adate: NaiveDate, atime: NaiveTime, tz: f64) -> (i32, u32, u32, u32, u32, u32) {
// Naive date time, with no time zone information
// The known tz hours time offset in seconds
   let tz_offset = FixedOffset::east(tz as i32 * 3600);
   let datetime = NaiveDateTime::new(adate, atime);
   let dt_with_tz: DateTime<FixedOffset> = tz_offset.from_local_datetime(&datetime).unwrap();
// If you need to convert it to a DateTime<Utc>, you can do this:
   let dt_with_tz_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());
   println!("UTC + {} {} to {}", tz, datetime, dt_with_tz_utc);
   let atuplet = get_date_elements(dt_with_tz_utc);
   let btuplet = get_time_elements(dt_with_tz_utc);
   let ctuplet = (atuplet.0, atuplet.1, atuplet.2, btuplet.0, btuplet.1, btuplet.2);
   println!("UTC date: {:.?}", atuplet);
   println!("UTC time: {:.?}", btuplet);
   ctuplet
}

fn get_date_elements(dt: DateTime<Utc>) -> (i32, u32, u32) {
   (dt.year(), dt.month(), dt.day())
}

fn get_time_elements(dt: DateTime<Utc>) -> (u32, u32, u32) {
   (dt.hour(), dt.minute(), dt.second())
}
