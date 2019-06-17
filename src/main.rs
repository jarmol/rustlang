use std::env;    
use chrono::{Local};
use suncalc::solar::trigonos::cosd;
use suncalc::solar::trigonos::acosd;
use suncalc::solar::trigonos::sind; 
use suncalc::solar::trigonos::asind; 
use suncalc::solar::trigonos::tand;
use suncalc::solar::date_jdn;
use suncalc::solar::jd_epoch;
use suncalc::solar::utc_time_jd;

const ZENITH_SUNRISE: f64 = 90.83;

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
    let sun_eq_cntr = sun_equat_centr(epoch); // expected 0.874
    let true_long = sun_true_longit(epoch, sun_eq_cntr); // expected 76.413
    let sun_long = sun_app_long(true_long, epoch);
    let my_declination = declination(23.4359, sun_long); // except 22.74
    let ha_rise = sunrise_ha(latitude, my_declination);
    println!("Epoch 2000 = {:.6}", epoch);
    println!("Sun equat. centre      =   {:.3} °", sun_eq_cntr); // expected 0.874
    println!("Sun true longitude     =  {:.3} °", true_long);
    println!("Sun apparent longitude =  {:.3} °", sun_long);
    println!("Declination            =  {:.3} °", my_declination);
    println!("HA Sunrise             = {:.3} °", ha_rise); // expect 166.75 deg
}

fn sunrise_ha(latit: f64, declinat: f64) -> f64 {
       (acosd(cosd(ZENITH_SUNRISE)/((cosd(latit))*cosd(declinat)) - tand(latit)*tand(declinat)))
}

fn jd_output(jdn: f64, h: i32, mn: i32) {
    println!("UTC time: {}h {}min", h, mn);
    let x = utc_time_jd(jdn, h, mn);
    println!("JD = {:.4}", x);
}

fn sun_true_longit(epoc: f64, sun_eq_ctr: f64) -> f64 {
   sun_eq_ctr + (280.46646 + epoc * (36000.76983 + epoc * 3.032E-4)) % 360.0
}

fn sun_app_long(sun_true_long: f64, epoch: f64) -> f64 {
    sun_true_long - 0.00569 - 0.00478 * sind(125.04 - 1934.136 * epoch)
}

fn sun_equat_centr(epoc: f64) -> f64 {
    let mean_anom: f64 = (357.52911 + epoc * (35999.05029 - 1.537E-4 * epoc)) % 360.0; 
    let arvo: f64 = sind(mean_anom) * (1.914_602 - epoc * (4.817E-3 + 1.4E-5 * epoc))
        + sind(2.0 * mean_anom) * (0.019_993 - 1.01E-4 * epoc)
        + sind(3.0 * mean_anom) * 2.89E-4;
    arvo // returned value
}
fn declination(obl_cor: f64, sun_app_long: f64) -> f64 {
    asind(sind(obl_cor) * sind(sun_app_long))
}

