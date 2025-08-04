mod components;
mod simulation;
mod monitoring;

use components::{Turbine, Generator, Reservoir, WaterFlow};
use simulation::PowerPlantSimulator;
use monitoring::MonitoringSystem;

fn main() {
    println!("🌊 Hydro Power Plant Simulator Starting...");
    
    // Initialize power plant components
    let turbine = Turbine::new("Main Turbine", 100.0, 0.85);
    let generator = Generator::new("Main Generator", 95.0, 0.92);
    let reservoir = Reservoir::new("Upper Reservoir", 100_000_000.0, 90_000_000.0);
    let water_flow = WaterFlow::new(50.0, 0.1);
    
    // Create monitoring system
    let monitoring = MonitoringSystem::new();
    
    // Initialize simulator
    let mut simulator = PowerPlantSimulator::new(
        turbine,
        generator,
        reservoir,
        water_flow,
        monitoring,
    );
    
    // Run simulation
    println!("\n🚀 Starting simulation...");
    simulator.run_simulation(10); // Run for 10 time steps
}
