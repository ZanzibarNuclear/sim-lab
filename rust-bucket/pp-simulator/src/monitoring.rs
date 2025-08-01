use std::collections::HashMap;

/// Represents a single reading from a sensor or component
#[derive(Debug, Clone)]
pub struct Reading {
    pub timestamp: f64,
    pub value: f64,
    pub unit: String,
}

/// Represents the monitoring system that tracks all plant parameters
pub struct MonitoringSystem {
    readings: HashMap<String, Vec<Reading>>,
    alerts: Vec<Alert>,
    performance_metrics: PerformanceMetrics,
}

/// Represents an alert or warning condition
#[derive(Debug, Clone)]
pub struct Alert {
    pub timestamp: f64,
    pub severity: AlertSeverity,
    pub message: String,
    pub parameter: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Tracks performance metrics over time
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_energy_mwh: f64,
    pub peak_power_mw: f64,
    pub average_power_mw: f64,
    pub efficiency_trend: Vec<f64>,
    pub uptime_percentage: f64,
    pub total_alerts: u32,
}

impl MonitoringSystem {
    pub fn new() -> Self {
        Self {
            readings: HashMap::new(),
            alerts: Vec::new(),
            performance_metrics: PerformanceMetrics {
                total_energy_mwh: 0.0,
                peak_power_mw: 0.0,
                average_power_mw: 0.0,
                efficiency_trend: Vec::new(),
                uptime_percentage: 100.0,
                total_alerts: 0,
            },
        }
    }

    pub fn record_readings(&mut self, timestamp: f64, readings: HashMap<String, f64>) {
        for (parameter, value) in readings {
            let reading = Reading {
                timestamp,
                value,
                unit: self.get_unit_for_parameter(&parameter),
            };
            
            self.readings
                .entry(parameter.clone())
                .or_insert_with(Vec::new)
                .push(reading);
            
            // Update performance metrics
            self.update_performance_metrics(&parameter, value);
        }
    }

    fn get_unit_for_parameter(&self, parameter: &str) -> String {
        match parameter {
            "turbine_power_mw" | "generator_power_mw" => "MW".to_string(),
            "reservoir_level_percent" => "%".to_string(),
            "water_flow_m3s" => "m³/s".to_string(),
            "head_height_m" => "m".to_string(),
            "turbine_efficiency" | "generator_efficiency" => "".to_string(),
            _ => "".to_string(),
        }
    }

    fn update_performance_metrics(&mut self, parameter: &str, value: f64) {
        match parameter {
            "generator_power_mw" => {
                if value > self.performance_metrics.peak_power_mw {
                    self.performance_metrics.peak_power_mw = value;
                }
            }
            "turbine_efficiency" => {
                self.performance_metrics.efficiency_trend.push(value);
                // Keep only last 100 efficiency readings
                if self.performance_metrics.efficiency_trend.len() > 100 {
                    self.performance_metrics.efficiency_trend.remove(0);
                }
            }
            _ => {}
        }
    }

    pub fn add_alert(&mut self, timestamp: f64, severity: AlertSeverity, message: String, parameter: String, value: f64) {
        let alert = Alert {
            timestamp,
            severity,
            message,
            parameter,
            value,
        };
        
        self.alerts.push(alert);
        self.performance_metrics.total_alerts += 1;
    }

    pub fn get_latest_reading(&self, parameter: &str) -> Option<&Reading> {
        self.readings
            .get(parameter)
            .and_then(|readings| readings.last())
    }

    pub fn get_readings_for_parameter(&self, parameter: &str) -> Option<&Vec<Reading>> {
        self.readings.get(parameter)
    }

    pub fn calculate_average(&self, parameter: &str) -> Option<f64> {
        self.readings
            .get(parameter)
            .map(|readings| {
                let sum: f64 = readings.iter().map(|r| r.value).sum();
                sum / readings.len() as f64
            })
    }

    pub fn calculate_trend(&self, parameter: &str) -> Option<f64> {
        let readings = self.readings.get(parameter)?;
        if readings.len() < 2 {
            return None;
        }
        
        // Simple linear trend calculation
        let first_value = readings.first()?.value;
        let last_value = readings.last()?.value;
        let time_span = readings.last()?.timestamp - readings.first()?.timestamp;
        
        if time_span > 0.0 {
            Some((last_value - first_value) / time_span)
        } else {
            None
        }
    }

    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        report.push_str("📊 Performance Report\n");
        report.push_str("===================\n");
        
        // Power generation metrics
        if let Some(avg_power) = self.calculate_average("generator_power_mw") {
            report.push_str(&format!("⚡ Average Power Output: {:.1} MW\n", avg_power));
        }
        
        if let Some(_peak_power) = self.get_latest_reading("generator_power_mw") {
            report.push_str(&format!("📈 Peak Power Output: {:.1} MW\n", self.performance_metrics.peak_power_mw));
        }
        
        // Efficiency metrics
        if let Some(avg_turbine_eff) = self.calculate_average("turbine_efficiency") {
            report.push_str(&format!("🌀 Average Turbine Efficiency: {:.1}%\n", avg_turbine_eff * 100.0));
        }
        
        if let Some(avg_gen_eff) = self.calculate_average("generator_efficiency") {
            report.push_str(&format!("⚡ Average Generator Efficiency: {:.1}%\n", avg_gen_eff * 100.0));
        }
        
        // Reservoir metrics
        if let Some(avg_reservoir) = self.calculate_average("reservoir_level_percent") {
            report.push_str(&format!("💧 Average Reservoir Level: {:.1}%\n", avg_reservoir));
        }
        
        // Water flow metrics
        if let Some(avg_flow) = self.calculate_average("water_flow_m3s") {
            report.push_str(&format!("🌊 Average Water Flow: {:.1} m³/s\n", avg_flow));
        }
        
        // Alert summary
        report.push_str(&format!("🚨 Total Alerts: {}\n", self.performance_metrics.total_alerts));
        
        // Trend analysis
        if let Some(power_trend) = self.calculate_trend("generator_power_mw") {
            let trend_direction = if power_trend > 0.0 { "↗️ Increasing" } else if power_trend < 0.0 { "↘️ Decreasing" } else { "→ Stable" };
            report.push_str(&format!("📈 Power Trend: {}\n", trend_direction));
        }
        
        report
    }

    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<&Alert> {
        self.alerts
            .iter()
            .filter(|alert| alert.severity == severity)
            .collect()
    }

    pub fn get_recent_alerts(&self, hours: f64) -> Vec<&Alert> {
        let cutoff_time = if let Some(latest) = self.alerts.last() {
            latest.timestamp - hours
        } else {
            return Vec::new();
        };
        
        self.alerts
            .iter()
            .filter(|alert| alert.timestamp >= cutoff_time)
            .collect()
    }

    pub fn export_data(&self) -> HashMap<String, Vec<Reading>> {
        self.readings.clone()
    }

    pub fn clear_old_data(&mut self, hours_to_keep: f64) {
        let cutoff_time = if let Some(latest) = self.alerts.last() {
            latest.timestamp - hours_to_keep
        } else {
            return;
        };
        
        // Clear old readings
        for readings in self.readings.values_mut() {
            readings.retain(|reading| reading.timestamp >= cutoff_time);
        }
        
        // Clear old alerts
        self.alerts.retain(|alert| alert.timestamp >= cutoff_time);
    }
}

impl Default for MonitoringSystem {
    fn default() -> Self {
        Self::new()
    }
} 