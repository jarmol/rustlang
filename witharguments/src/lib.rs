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

  pub fn time_equation(epoc: f64) -> f64 {
      let mean_long_sun: f64 = (280.46646 + epoc * (36000.76983 + epoc * 3.032E-4)) % 360.0; 
      let mean_anom: f64 = (357.52911 + epoc * (35999.05029 - 1.537E-4 * epoc)) % 360.0;
      let eccent_orbit = 0.0167; 
      let y_var = 0.043;
      let pal_arvo: f64 = 4.0*(y_var*sind(2.0*(mean_long_sun))
      - 2.0*eccent_orbit
      *sind(mean_anom)
      + 4.0*eccent_orbit*y_var
      *sind(mean_anom)
      *cosd(2.0*mean_long_sun)
      - 0.5*y_var*y_var
      *sind(4.0*mean_long_sun)
      - 1.25*eccent_orbit*eccent_orbit
      *sind(2.0*mean_anom)).to_degrees();

      pal_arvo
  }

   pub fn sunrise_ha(latit: f64, declinat: f64) -> f64 {
      (acosd(cosd(ZENITH_SUNRISE)/(cosd(latit)*cosd(declinat)) - tand(latit)*tand(declinat)))
   }

   pub fn noon_time(epoc: f64, longit: f64, tz: f64) -> f64 {
       let eq_of_time = time_equation(epoc);
      (720.0 - 4.0*longit - eq_of_time + tz*60.0)/1440.0 // noontime[minutes]/24[hours]
   }

   pub fn true_solar_time(hh: u32, mm: u32, ss: u32, longit: f64, epoc: f64, tz: f64) -> f64 {
       let f = hh as f64 / 24.0 + mm as f64 / 1440.0 + ss as f64 / 86400.0;
       let time_equat: f64 = time_equation(epoc);   // except 1.183;
       (f*1440.0 + time_equat + 4.0*longit - 60.0*tz) % 1440.0
   }

   pub fn hour_angle(true_sol_time: f64) -> f64 {
       let ha = if true_sol_time < 0.0 {true_sol_time/4.0 + 180.0}
       else {true_sol_time/4.0 - 180.0};
       ha
    }

// Azimuth angle
   pub fn calc_f_azim(ha: f64, declin: f64, latit: f64) -> f64 {
       let y: f64 = sind(ha);
       let x: f64 = cosd(ha)*sind(latit) - tand(declin)*cosd(latit); 
       180.0 + y.atan2(x).to_degrees()
   }

   pub fn  solar_zenith_angle(latit: f64, sun_declin: f64, hr_angle: f64) -> f64 {
       acosd(sind(latit)*sind(sun_declin)+cosd(latit)*cosd(sun_declin)*cosd(hr_angle))    // 43.108 degrees expected
    } 

   pub fn atmospheric_refraction(h: f64) -> f64 {
       let x = h.to_radians().tan();
       let mut refract: f64 = 0.0;
       if h > 85.0 {refract = 0.0;}
       else if h > 5.0 {refract = 58.1/x - 0.07/x.powf(3.0) + 0.000086/x.powf(5.0);}
       else if h > -0.575 {refract = 1735.0 + h*(-518.2 + h*(103.4 + h*(-12.79 + h*0.711)));}
       else if h < -0.575 {refract = -20.772 / x; };

       refract/3600.0   // returned value
   }

   pub fn refr_correct_altitude(zenith: f64, height: f64) -> f64 {
       let refract = atmospheric_refraction(height);
       90.0 - zenith + refract
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

pub fn jd_epoch(jdn: f64, h: u32, mns: u32) -> f64 {
    let jd = utc_time_jd(jdn, h, mns);
    (jd - 2_451_545.0) / 36525.0
  }

pub fn utc_time_jd(jdn: f64, h: u32, mns: u32) -> f64 {
    let hdelta: f64 = (f64::from(h) + f64::from(mns) / 60.0) / 24.0;
    let jd2: f64 = hdelta + jdn - 0.5; 
    jd2 // return value of function
  }
 }

