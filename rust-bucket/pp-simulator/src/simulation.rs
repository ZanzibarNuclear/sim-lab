use crate::components::{Turbine, Generator, Reservoir, WaterFlow};
use crate::monitoring::MonitoringSystem;
use std::collections::HashMap;

/// Main simulator that coordinates all power plant components
pub struct PowerPlantSimulator {
    turbine: Turbine,
    generator: Generator,
    reservoir: Reservoir,
    water_flow: WaterFlow,
    monitoring: MonitoringSystem,
    time_step_hours: f64,
    current_time_hours: f64,
    total_energy_mwh: f64,
    alerts: Vec<String>,
}

impl PowerPlantSimulator {
    pub fn new(
        turbine: Turbine,
        generator: Generator,
        reservoir: Reservoir,
        water_flow: WaterFlow,
        monitoring: MonitoringSystem,
    ) -> Self {
        Self {
            turbine,
            generator,
            reservoir,
            water_flow,
            monitoring,
            time_step_hours: 1.0, // 1-hour time steps
            current_time_hours: 0.0,
            total_energy_mwh: 0.0,
            alerts: Vec::new(),
        }
    }

    pub fn run_simulation(&mut self, num_steps: u32) {
        println!("⏰ Starting simulation for {} time steps ({} hours)", num_steps, num_steps);
        
        // Initialize components
        self.initialize_plant();
        
        for step in 1..=num_steps {
            self.current_time_hours += self.time_step_hours;
            println!("\n🕐 Time Step {} (Hour {:.1})", step, self.current_time_hours);
            
            // Update reservoir with natural inflow
            self.update_reservoir();
            
            // Calculate available head
            let head_height = self.reservoir.get_available_head();
            
            // Update water flow based on reservoir
            self.water_flow.adjust_flow_rate(self.reservoir.outflow_rate_m3s);
            self.water_flow.calculate_pressure(head_height);
            
            // Calculate turbine power
            let mechanical_power = self.turbine.calculate_power(
                self.water_flow.flow_rate_m3s,
                head_height,
            );
            
            // Generate electrical power
            let electrical_power = self.generator.generate_power(mechanical_power);
            
            // Update total energy
            self.total_energy_mwh += electrical_power * self.time_step_hours;
            
            // Monitor and log status
            self.monitor_plant_status();
            
            // Check for alerts
            self.check_alerts();
            
            // Display current status
            self.display_status();
        }
        
        self.display_final_report();
    }

    fn initialize_plant(&mut self) {
        println!("🔧 Initializing power plant components...");
        
        // Synchronize generator
        self.generator.synchronize();
        println!("✅ Generator synchronized to grid");
        
        // Set initial water flow
        self.reservoir.set_outflow_rate(50.0); // 50 m³/s
        println!("💧 Water flow initialized at {:.1} m³/s", self.water_flow.flow_rate_m3s);
        
        // Set reservoir inflow (simulating natural inflow)
        self.reservoir.set_inflow_rate(30.0); // 30 m³/s natural inflow
        println!("🌊 Reservoir inflow set to {:.1} m³/s", self.reservoir.inflow_rate_m3s);
    }

    fn update_reservoir(&mut self) {
        self.reservoir.update_volume(self.time_step_hours);
        
        // Simulate varying inflow based on time (day/night cycle)
        let time_of_day = (self.current_time_hours % 24.0) / 24.0;
        let inflow_variation = 1.0 + 0.2 * (2.0 * std::f64::consts::PI * time_of_day).sin();
        let new_inflow = 30.0 * inflow_variation;
        self.reservoir.set_inflow_rate(new_inflow);
    }

    fn monitor_plant_status(&mut self) {
        let mut readings = HashMap::new();
        
        readings.insert("turbine_power_mw".to_string(), self.turbine.current_power_mw);
        readings.insert("generator_power_mw".to_string(), self.generator.current_power_mw);
        readings.insert("reservoir_level_percent".to_string(), self.reservoir.get_water_level_percentage());
        readings.insert("water_flow_m3s".to_string(), self.water_flow.flow_rate_m3s);
        readings.insert("head_height_m".to_string(), self.reservoir.get_available_head());
        readings.insert("turbine_efficiency".to_string(), self.turbine.efficiency);
        readings.insert("generator_efficiency".to_string(), self.generator.efficiency);
        
        self.monitoring.record_readings(self.current_time_hours, readings);
    }

    fn check_alerts(&mut self) {
        // Check for low reservoir level
        if self.reservoir.get_water_level_percentage() < 20.0 {
            self.alerts.push(format!(
                "⚠️  Low reservoir level: {:.1}%",
                self.reservoir.get_water_level_percentage()
            ));
        }
        
        // Check for high power output
        if self.generator.current_power_mw > self.generator.max_power_mw * 0.95 {
            self.alerts.push(format!(
                "⚠️  High power output: {:.1} MW",
                self.generator.current_power_mw
            ));
        }
        
        // Check for unsafe water flow
        if !self.water_flow.is_flow_safe() {
            self.alerts.push("🚨 Unsafe water flow detected!".to_string());
        }
        
        // Check for low efficiency
        if self.turbine.efficiency < 0.7 {
            self.alerts.push("⚠️  Low turbine efficiency detected".to_string());
        }
    }

    fn display_status(&self) {
        println!("📊 Current Status:");
        println!("  🌀 Turbine: {:.1} MW (Efficiency: {:.1}%)", 
                self.turbine.current_power_mw, self.turbine.efficiency * 100.0);
        println!("  ⚡ Generator: {:.1} MW (Efficiency: {:.1}%)", 
                self.generator.current_power_mw, self.generator.efficiency * 100.0);
        println!("  💧 Reservoir: {:.1}% full ({:.0} m³)", 
                self.reservoir.get_water_level_percentage(), self.reservoir.current_volume_m3);
        println!("  🌊 Water Flow: {:.1} m³/s", self.water_flow.flow_rate_m3s);
        println!("  📏 Head Height: {:.1} m", self.reservoir.get_available_head());
        
        if !self.alerts.is_empty() {
            println!("  🚨 Alerts:");
            for alert in &self.alerts {
                println!("    {}", alert);
            }
        }
    }

    fn display_final_report(&self) {
        println!("\n📈 Final Simulation Report");
        println!("==========================");
        println!("⏱️  Total simulation time: {:.1} hours", self.current_time_hours);
        println!("⚡ Total energy generated: {:.1} MWh", self.total_energy_mwh);
        println!("📊 Average power output: {:.1} MW", 
                self.total_energy_mwh / self.current_time_hours);
        println!("🌊 Final reservoir level: {:.1}%", self.reservoir.get_water_level_percentage());
        println!("🔧 Final turbine efficiency: {:.1}%", self.turbine.efficiency * 100.0);
        println!("⚡ Final generator efficiency: {:.1}%", self.generator.efficiency * 100.0);
        
        if !self.alerts.is_empty() {
            println!("🚨 Total alerts generated: {}", self.alerts.len());
        }
    }

    // Public methods for external control
    pub fn adjust_water_flow(&mut self, new_flow_rate: f64) {
        self.reservoir.set_outflow_rate(new_flow_rate);
        println!("💧 Water flow adjusted to {:.1} m³/s", new_flow_rate);
    }

    pub fn shutdown_turbine(&mut self) {
        self.turbine.shutdown();
        println!("🛑 Turbine shutdown");
    }

    pub fn startup_turbine(&mut self) {
        self.turbine.startup();
        println!("✅ Turbine startup");
    }

    pub fn get_current_power(&self) -> f64 {
        self.generator.current_power_mw
    }

    pub fn get_reservoir_level(&self) -> f64 {
        self.reservoir.get_water_level_percentage()
    }
} 