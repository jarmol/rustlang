pub mod solar {
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
}

