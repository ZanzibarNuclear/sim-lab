use std::f64::consts::PI;

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }

    pub fn area(&self) -> f64 {
        self.radius.powf(2.0) * PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_unit_circle() {
        let circle = Circle::new(1.0);
        assert_eq!(circle.area(), PI);
    }

    #[test]
    fn test_area_decimal_radius() {
        let circle = Circle::new(3.0);
        assert_eq!(circle.area(), 9.0 * PI);
    }
}
