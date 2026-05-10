//! Hades Governor Daemon
//! 
//! Background process that monitors GPU telemetry and applies
//! thermal back-pressure to weight streaming operations.
//! 
//! Usage:
//! ```bash
//! hades-governor --interval 500 --throttle-temp 72 --throttle-power 150
//! ```

use hades_kernel::{ThermalGovernor, ThermalPolicy, THERMAL_THROTTLE_TEMP_C, POWER_THROTTLE_WATTS};
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    info!("Hades Governor starting...");
    
    // Parse command line args (simple parsing)
    let args: Vec<String> = std::env::args().collect();
    
    let mut interval_ms = 500;
    let mut throttle_temp = THERMAL_THROTTLE_TEMP_C;
    let mut throttle_power = POWER_THROTTLE_WATTS;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--interval" | "-i" => {
                if i + 1 < args.len() {
                    interval_ms = args[i + 1].parse().unwrap_or(500);
                    i += 1;
                }
            }
            "--throttle-temp" | "-t" => {
                if i + 1 < args.len() {
                    throttle_temp = args[i + 1].parse().unwrap_or(THERMAL_THROTTLE_TEMP_C);
                    i += 1;
                }
            }
            "--throttle-power" | "-p" => {
                if i + 1 < args.len() {
                    throttle_power = args[i + 1].parse().unwrap_or(POWER_THROTTLE_WATTS);
                    i += 1;
                }
            }
            "--help" | "-h" => {
                println!("Hades Governor - Thermal Back-Pressure Daemon");
                println!();
                println!("Usage: hades-governor [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -i, --interval <ms>       Sampling interval in milliseconds (default: 500)");
                println!("  -t, --throttle-temp <C>   Throttle temperature in Celsius (default: 72)");
                println!("  -p, --throttle-power <W>  Throttle power in Watts (default: 150)");
                println!("  -h, --help                Show this help message");
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }
    
    // Create thermal governor with custom policy
    let policy = ThermalPolicy {
        throttle_temp_c: throttle_temp,
        throttle_power_w: throttle_power,
        sample_interval: Duration::from_millis(interval_ms),
        ..Default::default()
    };
    
    let mut governor = ThermalGovernor::with_policy(policy.clone())?;
    
    info!("Thermal policy:");
    info!("  Sample interval: {}ms", interval_ms);
    info!("  Throttle temperature: {}°C", throttle_temp);
    info!("  Throttle power: {}W", throttle_power);
    info!("  Critical temperature: {}°C", policy.critical_temp_c);
    
    let mut tick = interval(Duration::from_millis(interval_ms));
    
    info!("Starting thermal monitoring loop...");
    
    loop {
        tick.tick().await;
        
        match governor.sample() {
            Ok(telemetry) => {
                let state = governor.state();
                let throttle = governor.throttle_ratio();
                
                // Log state changes
                if telemetry.temperature_c > 0.0 {
                    info!(
                        "GPU: {:.1}°C | {}W | {:.0}% VRAM | Throttle: {:.0}%",
                        telemetry.temperature_c,
                        telemetry.power_watts,
                        telemetry.utilization * 100.0,
                        throttle * 100.0
                    );
                }
                
                // Handle critical state
                if state == hades_kernel::thermal::GovernorState::Critical {
                    error!("CRITICAL: Thermal emergency - all operations throttled to 0%");
                    // In production, could trigger system-level alerts
                }
            }
            Err(e) => {
                warn!("Failed to sample telemetry: {}", e);
                // Continue anyway - telemetry failure shouldn't crash daemon
            }
        }
    }
}
