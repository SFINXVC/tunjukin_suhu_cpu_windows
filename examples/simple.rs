//! Simple example demonstrating how to use the tunjukin_suhu_cpu_windows library.
//! 
//! Run this example with:
//! ```
//! cargo run --example simple
//! ```

use tunjukin_suhu_cpu_windows::CpuTemperature;

fn main() {
    println!("CPU Temperature Reader");
    println!("=====================");
    
    match CpuTemperature::get() {
        Ok(temp) => {
            println!("✓ Successfully retrieved CPU temperature:");
            println!("  Celsius: {:.2}°C", temp.celsius);
            println!("  Fahrenheit: {:.2}°F", temp.fahrenheit);
        }
        Err(error) => {
            eprintln!("✗ Error reading CPU temperature: {}", error);
            eprintln!("\nTroubleshooting tips:");
            eprintln!("- Ensure you're running on Windows");
            eprintln!("- Try running as administrator");
            eprintln!("- Check if PowerShell is available");
        }
    }
}
