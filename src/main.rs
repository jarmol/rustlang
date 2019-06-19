use std::env;    
use chrono::{Local};
use suncalc::julian::date_jdn;
use suncalc::julian::jd_epoch;
use suncalc::julian::utc_time_jd;
use suncalc::solar::sun_app_long;
use suncalc::solar::sunrise_ha;
use suncalc::solar::declination;
use suncalc::solar::time_equation;
use suncalc::solar::noon_time;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program: {}", args[0]);

    let latitude:  f64 = 65.85;
    let longitude: f64 = 24.18;
    let time_zone: f64 =  2.0;

    let local_time = Local::now();
    let (year, month, day) = (2019, 6, 7);
    let my_jdn = f64::from(date_jdn(year, month, day));
    println!("Time now: {}", local_time.to_rfc2822());
    println!("Calculation Date: {}.{}.{}", day, month, year);

    println!("JDN = {}", my_jdn);
    jd_output(my_jdn, 10, 22);

    let epoch = jd_epoch(my_jdn, 10, 22);
    let sun_long = sun_app_long(epoch);
    let my_declination = declination(23.4359, sun_long); // except 22.74
    let ha_rise = sunrise_ha(latitude, my_declination);
    let noon_fraction = noon_time(epoch, longitude, time_zone);
    let noon_hours: u32 = (24.0*noon_fraction) as u32;
    let noon_minutes : u32 = (1440.0*noon_fraction % 60.0) as u32;

    println!("Epoch 2000 = {:.6}", epoch);
    println!("Declination            =  {:.3} °", my_declination);
    println!("HA Sunrise             = {:.3} °", ha_rise); // expect 166.75 deg
    println!("Time equation          = {:.3} min", time_equation(epoch)); // expect -0.858 minutes
    println!("Day length             =  {:.0} h {:.0} min", 2.0*ha_rise/15.0, 8.0*ha_rise % 60.0);
    println!("Noon time              = {} h {} min", noon_hours, noon_minutes);

fn jd_output(jdn: f64, h: i32, mn: i32) {
    println!("UTC time: {}h {}min", h, mn);
    let x = utc_time_jd(jdn, h, mn);
    println!("JD = {:.4}", x);
}

//  fn declination(obl_cor: f64, sun_app_long: f64) -> f64 {
//    asind(sind(obl_cor) * sind(sun_app_long))
//  }
}
