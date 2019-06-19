  pub mod solar {
  use crate::trigonos::sind;
  use crate::trigonos::tand;
  use crate::trigonos::cosd;
  use crate::trigonos::acosd;
  use crate::trigonos::asind;
  const ZENITH_SUNRISE: f64 = 90.83;

  pub fn sun_true_longit(epoc: f64) -> f64 {
       let sun_eq_cntr = sun_equat_centr(epoc); // expected 0.874 
       sun_eq_cntr + (280.46646 + epoc * (36000.76983 + epoc * 3.032E-4)) % 360.0
  }

  pub fn sun_app_long(epoch: f64) -> f64 {
         let sun_true_long = sun_true_longit(epoch);
         sun_true_long - 0.00569 - 0.00478 * sind(125.04 - 1934.136 * epoch)
    } 

  pub fn sun_equat_centr(epoc: f64) -> f64 {
    //  let sind = |x: f64|  x.to_radians().sin();
      let mean_anom: f64 = (357.52911 + epoc * (35999.05029 - 1.537E-4 * epoc)) % 360.0; 
      let arvo: f64 = sind(mean_anom) * (1.914_602 - epoc * (4.817E-3 + 1.4E-5 * epoc))
        + sind(2.0 * mean_anom) * (0.019_993 - 1.01E-4 * epoc)
        + sind(3.0 * mean_anom) * 2.89E-4;
    arvo // returned value
  }

  pub fn declination(obl_cor: f64, sun_app_long: f64) -> f64 {
      asind(sind(obl_cor) * sind(sun_app_long))
  }

   pub fn sunrise_ha(latit: f64, declinat: f64) -> f64 {
      (acosd(cosd(ZENITH_SUNRISE)/(cosd(latit)*cosd(declinat)) - tand(latit)*tand(declinat)))
   }
 }

  pub mod trigonos {

    pub fn sind(x: f64) -> f64 {
        x.to_radians().sin()
    }

    pub fn asind(x: f64) -> f64 {
        x.asin().to_degrees()
    }

    pub fn cosd(x: f64) -> f64 {
        x.to_radians().cos()
    }

    pub fn acosd(x: f64) -> f64 {
        x.acos().to_degrees()
    }

    pub fn tand(x: f64) -> f64 {
        x.to_radians().tan()
    }

    pub fn atand(x: f64) -> f64 {
        x.atan().to_degrees()
    }

  }

pub mod julian {
pub fn date_jdn(year: i32, month: i32, day: i32) -> i32 {
    let jdn2: i32 = (1461 * (year + 4800 + (month - 14) / 12)) / 4
        + (367 * (month - 2 - 12 * ((month - 14) / 12))) / 12 
        - (3 * ((year + 4900 + (month - 14) / 12) / 100)) / 4
        + day
        - 32075;
    jdn2 // return value
}

pub fn jd_epoch(jdn: f64, h: i32, mns: i32) -> f64 {
    let jd = utc_time_jd(jdn, h, mns);
    (jd - 2_451_545.0) / 36525.0
  }

pub fn utc_time_jd(jdn: f64, h: i32, mns: i32) -> f64 {
    let hdelta: f64 = (f64::from(h) + f64::from(mns) / 60.0) / 24.0;
    let jd2: f64 = hdelta + jdn - 0.5; 
    jd2 // return value of function
  }
 }

