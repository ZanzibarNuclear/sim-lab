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

pub struct RotatingWheel {
    radius: f64,
    angular_velocity: f64, // radians per second
    current_angle: f64,    // current position in radians
    time: f64,            // current simulation time in seconds
}

impl RotatingWheel {
    pub fn new(radius: f64, angular_velocity: f64) -> Self {
        RotatingWheel {
            radius,
            angular_velocity,
            current_angle: 0.0,
            time: 0.0,
        }
    }

    pub fn get_position(&self) -> f64 {
        self.current_angle
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn step(&mut self, delta_time: f64) {
        self.current_angle += self.angular_velocity * delta_time;
        self.time += delta_time;
        
        // Normalize angle to [0, 2π)
        self.current_angle = self.current_angle % (2.0 * PI);
        if self.current_angle < 0.0 {
            self.current_angle += 2.0 * PI;
        }
    }

    pub fn get_point_on_circumference(&self, angle_offset: f64) -> (f64, f64) {
        let total_angle = self.current_angle + angle_offset;
        let x = self.radius * total_angle.cos();
        let y = self.radius * total_angle.sin();
        (x, y)
    }
}

pub struct SimulationConfig {
    pub total_time: f64,        // total simulation time in seconds
    pub polling_interval: f64,  // how often to record position (seconds)
    pub angular_velocity: f64,  // radians per second
    pub radius: f64,           // wheel radius
}

pub struct SimulationResult {
    pub time_points: Vec<f64>,
    pub positions: Vec<f64>,
    pub points_on_circumference: Vec<(f64, f64)>,
}

pub fn run_rotation_simulation(config: SimulationConfig) -> SimulationResult {
    let mut wheel = RotatingWheel::new(config.radius, config.angular_velocity);
    let mut time_points = Vec::new();
    let mut positions = Vec::new();
    let mut points_on_circumference = Vec::new();
    
    let mut next_poll_time = 0.0;
    let time_step = config.polling_interval.min(0.01); // Use smaller of polling interval or 10ms
    
    // Record initial state
    time_points.push(wheel.get_time());
    positions.push(wheel.get_position());
    points_on_circumference.push(wheel.get_point_on_circumference(0.0));
    next_poll_time += config.polling_interval;
    
    while wheel.get_time() < config.total_time {
        wheel.step(time_step);
        
        // Record data at polling intervals
        if wheel.get_time() >= next_poll_time {
            time_points.push(wheel.get_time());
            positions.push(wheel.get_position());
            points_on_circumference.push(wheel.get_point_on_circumference(0.0));
            next_poll_time += config.polling_interval;
        }
    }
    
    SimulationResult {
        time_points,
        positions,
        points_on_circumference,
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

    #[test]
    fn test_rotating_wheel_creation() {
        let wheel = RotatingWheel::new(5.0, 2.0);
        assert_eq!(wheel.radius, 5.0);
        assert_eq!(wheel.angular_velocity, 2.0);
        assert_eq!(wheel.get_position(), 0.0);
        assert_eq!(wheel.get_time(), 0.0);
    }

    #[test]
    fn test_wheel_rotation() {
        let mut wheel = RotatingWheel::new(1.0, 1.0); // 1 radian per second
        
        // After 1 second, should have rotated 1 radian
        wheel.step(1.0);
        assert!((wheel.get_position() - 1.0).abs() < 1e-10);
        assert!((wheel.get_time() - 1.0).abs() < 1e-10);
        
        // After another 0.5 seconds, should have rotated 0.5 more radians
        wheel.step(0.5);
        assert!((wheel.get_position() - 1.5).abs() < 1e-10);
        assert!((wheel.get_time() - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_angle_normalization() {
        let mut wheel = RotatingWheel::new(1.0, 1.0);
        
        // Rotate more than 2π radians
        wheel.step(3.0 * PI);
        
        // Should be normalized to [0, 2π)
        let position = wheel.get_position();
        assert!(position >= 0.0);
        assert!(position < 2.0 * PI);
        assert!((position - (3.0 * PI - 2.0 * PI)).abs() < 1e-10);
    }

    #[test]
    fn test_point_on_circumference() {
        let wheel = RotatingWheel::new(2.0, 0.0); // No rotation
        
        // Point at angle 0 should be (2, 0)
        let (x, y) = wheel.get_point_on_circumference(0.0);
        assert!((x - 2.0).abs() < 1e-10);
        assert!((y - 0.0).abs() < 1e-10);
        
        // Point at angle π/2 should be (0, 2)
        let (x, y) = wheel.get_point_on_circumference(PI / 2.0);
        assert!((x - 0.0).abs() < 1e-10);
        assert!((y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotation_simulation() {
        let config = SimulationConfig {
            total_time: 1.0,
            polling_interval: 0.25,
            angular_velocity: 1.0, // 1 radian per second
            radius: 1.0,
        };
        
        let result = run_rotation_simulation(config);
        
        // Should have 5 data points (0, 0.25, 0.5, 0.75, 1.0 seconds)
        assert_eq!(result.time_points.len(), 5);
        assert_eq!(result.positions.len(), 5);
        assert_eq!(result.points_on_circumference.len(), 5);
        
        // Check first and last positions
        assert!((result.positions[0] - 0.0).abs() < 1e-10);
        assert!((result.positions[4] - 1.0).abs() < 1e-10);
        
        // Check time points
        assert!((result.time_points[0] - 0.0).abs() < 1e-10);
        assert!((result.time_points[4] - 1.0).abs() < 1e-10);
    }
}
