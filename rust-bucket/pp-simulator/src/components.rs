use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a hydro turbine that converts water flow to mechanical energy
#[derive(Debug, Clone)]
pub struct Turbine {
    pub name: String,
    pub max_power_mw: f64,
    pub efficiency: f64,
    pub current_power_mw: f64,
    pub is_operational: bool,
    pub last_maintenance: u64,
}

impl Turbine {
    pub fn new(name: &str, max_power_mw: f64, efficiency: f64) -> Self {
        Self {
            name: name.to_string(),
            max_power_mw,
            efficiency,
            current_power_mw: 0.0,
            is_operational: true,
            last_maintenance: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn calculate_power(&mut self, water_flow_rate: f64, head_height: f64) -> f64 {
        if !self.is_operational {
            self.current_power_mw = 0.0;
            return 0.0;
        }

        // Simplified power calculation: P = η * ρ * g * Q * H
        // where η = efficiency, ρ = water density, g = gravity, Q = flow rate, H = head
        let water_density = 1000.0; // kg/m³
        let gravity = 9.81; // m/s²
        
        let theoretical_power = water_density * gravity * water_flow_rate * head_height;
        self.current_power_mw = (theoretical_power * self.efficiency) / 1_000_000.0; // Convert to MW
        
        // Ensure power doesn't exceed maximum
        if self.current_power_mw > self.max_power_mw {
            self.current_power_mw = self.max_power_mw;
        }
        
        self.current_power_mw
    }

    pub fn shutdown(&mut self) {
        self.is_operational = false;
        self.current_power_mw = 0.0;
    }

    pub fn startup(&mut self) {
        self.is_operational = true;
    }

    pub fn schedule_maintenance(&mut self) {
        self.last_maintenance = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// Represents an electrical generator that converts mechanical energy to electrical energy
#[derive(Debug, Clone)]
pub struct Generator {
    pub name: String,
    pub max_power_mw: f64,
    pub efficiency: f64,
    pub current_power_mw: f64,
    pub voltage_kv: f64,
    pub frequency_hz: f64,
    pub is_synchronized: bool,
}

impl Generator {
    pub fn new(name: &str, max_power_mw: f64, efficiency: f64) -> Self {
        Self {
            name: name.to_string(),
            max_power_mw,
            efficiency,
            current_power_mw: 0.0,
            voltage_kv: 11.0, // Standard generator voltage
            frequency_hz: 50.0, // Standard frequency
            is_synchronized: false,
        }
    }

    pub fn generate_power(&mut self, mechanical_power_mw: f64) -> f64 {
        if !self.is_synchronized {
            self.current_power_mw = 0.0;
            return 0.0;
        }

        self.current_power_mw = mechanical_power_mw * self.efficiency;
        
        // Ensure power doesn't exceed maximum
        if self.current_power_mw > self.max_power_mw {
            self.current_power_mw = self.max_power_mw;
        }
        
        self.current_power_mw
    }

    pub fn synchronize(&mut self) {
        self.is_synchronized = true;
    }

    pub fn desynchronize(&mut self) {
        self.is_synchronized = false;
        self.current_power_mw = 0.0;
    }

    pub fn adjust_frequency(&mut self, target_frequency: f64) {
        self.frequency_hz = target_frequency;
    }
}

/// Represents a water reservoir with storage capacity and current level
#[derive(Debug, Clone)]
pub struct Reservoir {
    pub name: String,
    pub max_capacity_m3: f64,
    pub current_volume_m3: f64,
    pub inflow_rate_m3s: f64,
    pub outflow_rate_m3s: f64,
    pub height_m: f64,
}

impl Reservoir {
    pub fn new(name: &str, max_capacity_m3: f64, current_volume_m3: f64) -> Self {
        Self {
            name: name.to_string(),
            max_capacity_m3,
            current_volume_m3,
            inflow_rate_m3s: 0.0,
            outflow_rate_m3s: 0.0,
            height_m: 100.0, // Default height
        }
    }

    pub fn update_volume(&mut self, time_step_hours: f64) {
        let time_step_seconds = time_step_hours * 3600.0;
        let net_flow = self.inflow_rate_m3s - self.outflow_rate_m3s;
        let volume_change = net_flow * time_step_seconds;
        
        self.current_volume_m3 += volume_change;
        
        // Ensure volume stays within bounds
        if self.current_volume_m3 > self.max_capacity_m3 {
            self.current_volume_m3 = self.max_capacity_m3;
        } else if self.current_volume_m3 < 0.0 {
            self.current_volume_m3 = 0.0;
        }
    }

    pub fn set_inflow_rate(&mut self, rate_m3s: f64) {
        self.inflow_rate_m3s = rate_m3s;
    }

    pub fn set_outflow_rate(&mut self, rate_m3s: f64) {
        self.outflow_rate_m3s = rate_m3s;
    }

    pub fn get_water_level_percentage(&self) -> f64 {
        (self.current_volume_m3 / self.max_capacity_m3) * 100.0
    }

    pub fn get_available_head(&self) -> f64 {
        // Simplified head calculation based on water level
        let water_level_ratio = self.current_volume_m3 / self.max_capacity_m3;
        self.height_m * water_level_ratio
    }
}

/// Represents water flow characteristics and control
#[derive(Debug, Clone)]
pub struct WaterFlow {
    pub flow_rate_m3s: f64,
    pub pressure_pa: f64,
    pub temperature_celsius: f64,
    pub turbidity_ntu: f64,
}

impl WaterFlow {
    pub fn new(flow_rate_m3s: f64, turbidity_ntu: f64) -> Self {
        Self {
            flow_rate_m3s,
            pressure_pa: 101325.0, // Standard atmospheric pressure
            temperature_celsius: 15.0, // Typical water temperature
            turbidity_ntu,
        }
    }

    pub fn adjust_flow_rate(&mut self, new_rate_m3s: f64) {
        self.flow_rate_m3s = new_rate_m3s;
    }

    pub fn calculate_pressure(&mut self, head_height: f64) {
        // Simplified pressure calculation: P = ρ * g * h
        let water_density = 1000.0; // kg/m³
        let gravity = 9.81; // m/s²
        self.pressure_pa = water_density * gravity * head_height;
    }

    pub fn is_flow_safe(&self) -> bool {
        // Check if flow rate is within safe operating limits
        self.flow_rate_m3s > 0.0 && self.flow_rate_m3s <= 100.0
    }
} 