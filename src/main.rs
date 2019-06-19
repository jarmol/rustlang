use std::env;    
use chrono::{Local};
use suncalc::julian::date_jdn;
use suncalc::julian::jd_epoch;
use suncalc::julian::utc_time_jd;
use suncalc::solar::sun_app_long;
use suncalc::solar::sunrise_ha;
use suncalc::solar::declination;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program: {}", args[0]);

    let latitude: f64 = 65.85;

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
    println!("Epoch 2000 = {:.6}", epoch);
    println!("Declination            =  {:.3} °", my_declination);
    println!("HA Sunrise             = {:.3} °", ha_rise); // expect 166.75 deg
    println!("Day length             =  {:.0} h {:.0} min", 2.0*ha_rise/15.0, 8.0*ha_rise % 60.0);

fn jd_output(jdn: f64, h: i32, mn: i32) {
    println!("UTC time: {}h {}min", h, mn);
    let x = utc_time_jd(jdn, h, mn);
    println!("JD = {:.4}", x);
}

//  fn declination(obl_cor: f64, sun_app_long: f64) -> f64 {
//    asind(sind(obl_cor) * sind(sun_app_long))
//  }
}
