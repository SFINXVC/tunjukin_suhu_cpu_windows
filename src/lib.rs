//! # CPU Temperature Library for Windows
//!
//! A simple and efficient library for reading CPU temperature on Windows systems
//! using WMI (Windows Management Instrumentation) queries.
//!
//! This library provides a clean interface to get CPU temperature readings from
//! the Windows thermal zone sensors through PowerShell WMI queries.
//!
//! ## Features
//!
//! - Simple API with just one main struct and method
//! - Automatic temperature conversion (Celsius and Fahrenheit)
//! - Cross-platform Windows support (requires PowerShell)
//! - Error handling for robust applications
//! - Zero-config: works out of the box
//!
//! ## Requirements
//!
//! - Windows operating system
//! - PowerShell available in PATH
//! - Administrator privileges may be required for some systems
//!
//! ## Quick Start
//!
//! ```no_run
//! use tunjukin_suhu_cpu_windows::CpuTemperature;
//!
//! // Get the current CPU temperature
//! match CpuTemperature::get() {
//!     Ok(temp) => {
//!         println!("CPU Temperature: {:.2}°C / {:.2}°F", temp.celsius, temp.fahrenheit);
//!     }
//!     Err(e) => {
//!         eprintln!("Error reading temperature: {}", e);
//!     }
//! }
//! ```
//!
//! ## Error Handling
//!
//! This library returns detailed error messages for common issues:
//! - PowerShell execution failures
//! - WMI query errors
//! - Temperature sensor unavailability
//! - Parsing errors

use std::process::Command;
use regex::Regex;

/// Represents a CPU temperature reading with values in both Celsius and Fahrenheit.
///
/// This struct contains the temperature values converted from the raw thermal zone
/// sensor data obtained through Windows WMI queries.
///
/// # Fields
///
/// * `celsius` - Temperature in degrees Celsius
/// * `fahrenheit` - Temperature in degrees Fahrenheit
///
/// # Example
///
/// ```no_run
/// use tunjukin_suhu_cpu_windows::CpuTemperature;
///
/// let temp = CpuTemperature::get()?;
/// println!("Temperature: {:.1}°C ({:.1}°F)", temp.celsius, temp.fahrenheit);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CpuTemperature {
    /// Temperature in degrees Celsius
    pub celsius: f64,
    /// Temperature in degrees Fahrenheit  
    pub fahrenheit: f64,
}

impl CpuTemperature {
    /// Retrieves the current CPU temperature from Windows thermal zone sensors.
    ///
    /// This method executes a PowerShell WMI query to get temperature data from
    /// `MSAcpi_ThermalZoneTemperature` and returns the first available temperature
    /// reading converted to both Celsius and Fahrenheit.
    ///
    /// # Returns
    ///
    /// * `Ok(CpuTemperature)` - Successfully retrieved temperature
    /// * `Err(String)` - Error message describing what went wrong
    ///
    /// # Errors
    ///
    /// This method can fail for several reasons:
    /// - PowerShell is not available or fails to execute
    /// - WMI query returns no temperature sensors
    /// - Temperature data cannot be parsed
    /// - Insufficient permissions to access thermal sensors
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tunjukin_suhu_cpu_windows::CpuTemperature;
    ///
    /// match CpuTemperature::get() {
    ///     Ok(temp) => {
    ///         println!("Current CPU temperature:");
    ///         println!("  Celsius: {:.2}°C", temp.celsius);
    ///         println!("  Fahrenheit: {:.2}°F", temp.fahrenheit);
    ///     }
    ///     Err(error) => {
    ///         eprintln!("Failed to read temperature: {}", error);
    ///     }
    /// }
    /// ```
    ///
    /// # Platform Requirements
    ///
    /// - Windows operating system
    /// - PowerShell available in system PATH
    /// - May require administrator privileges on some systems
    pub fn get() -> Result<Self, String> {
        let output = Self::run_wmi_query()?;
        let celsius = Self::parse_temperature(&output)?;
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        
        Ok(CpuTemperature {
            celsius,
            fahrenheit,
        })
    }

    /// Executes the PowerShell WMI query to retrieve thermal zone temperature data.
    ///
    /// This internal method runs the WMI query using PowerShell and returns the
    /// raw output for further processing.
    fn run_wmi_query() -> Result<String, String> {
        let cmd = r#"Get-WmiObject MSAcpi_ThermalZoneTemperature -Namespace 'root/wmi' | Format-List"#;

        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", cmd])
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}. Ensure PowerShell is installed and accessible.", e))?;

        if !output.status.success() {
            return Err(format!(
                "WMI query failed with exit code: {}. You may need to run as administrator.",
                output.status
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Parses the PowerShell WMI output to extract temperature values.
    ///
    /// This method processes the Format-List output from the WMI query and
    /// extracts the first available CurrentTemperature value, converting it
    /// from the raw format (0.1 Kelvin units) to Celsius.
    fn parse_temperature(output: &str) -> Result<f64, String> {
        let re_temp = Regex::new(r"(?m)^\s*CurrentTemperature\s*:\s*(\d+)")
            .map_err(|e| format!("Regex compilation failed: {}", e))?;

        // Look for temperature values in the output
        for cap in re_temp.captures_iter(output) {
            if let Some(temp_str) = cap.get(1) {
                if let Ok(raw_value) = temp_str.as_str().parse::<f64>() {
                    // Convert from 0.1 Kelvin to Celsius
                    let celsius = (raw_value / 10.0) - 273.15;
                    
                    // Sanity check: temperature should be reasonable for CPU
                    if celsius > -50.0 && celsius < 150.0 {
                        return Ok(celsius);
                    }
                }
            }
        }

        if output.trim().is_empty() {
            Err("No temperature data received from WMI query. Check if thermal sensors are available.".to_string())
        } else {
            Err("No valid temperature readings found in WMI output. The thermal zone sensors may not be accessible.".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversion() {
        let temp = CpuTemperature {
            celsius: 25.0,
            fahrenheit: 77.0,
        };
        
        assert_eq!(temp.celsius, 25.0);
        assert_eq!(temp.fahrenheit, 77.0);
    }

    #[test]
    fn test_parse_temperature_valid() {
        let sample_output = r#"
CurrentTemperature   : 3120

InstanceName         : ACPI\ThermalZone\TZ00_0
        "#;
        
        let result = CpuTemperature::parse_temperature(sample_output);
        assert!(result.is_ok());
        
        let celsius = result.unwrap();
        assert!((celsius - 38.85).abs() < 0.01); // 3120/10 - 273.15 = 38.85°C
    }

    #[test]
    fn test_parse_temperature_empty() {
        let result = CpuTemperature::parse_temperature("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_temperature_invalid() {
        let sample_output = "No temperature data here";
        let result = CpuTemperature::parse_temperature(sample_output);
        assert!(result.is_err());
    }
}