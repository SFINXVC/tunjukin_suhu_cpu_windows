# tunjukin_suhu_cpu_windows

[![Crates.io](https://img.shields.io/crates/v/tunjukin_suhu_cpu_windows.svg)](https://crates.io/crates/tunjukin_suhu_cpu_windows)
[![Documentation](https://docs.rs/tunjukin_suhu_cpu_windows/badge.svg)](https://docs.rs/tunjukin_suhu_cpu_windows)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/yourusername/tunjukin_suhu_cpu_windows)

A simple and efficient Rust library for reading CPU temperature on Windows systems using WMI (Windows Management Instrumentation) queries.

## Features

- ✅ **Simple API** - Just one struct and one method to get temperature
- ✅ **Dual Temperature Units** - Returns both Celsius and Fahrenheit
- ✅ **Zero Configuration** - Works out of the box on Windows
- ✅ **Error Handling** - Detailed error messages for troubleshooting
- ✅ **Lightweight** - Minimal dependencies (only `regex`)
- ✅ **Well Documented** - Complete documentation and examples

## Requirements

- Windows operating system
- PowerShell available in system PATH
- Administrator privileges may be required on some systems

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tunjukin_suhu_cpu_windows = "0.1.0"
```

## Quick Start

```rust
use tunjukin_suhu_cpu_windows::CpuTemperature;

fn main() {
    match CpuTemperature::get() {
        Ok(temp) => {
            println!("CPU Temperature: {:.2}°C / {:.2}°F", 
                     temp.celsius, temp.fahrenheit);
        }
        Err(e) => {
            eprintln!("Error reading temperature: {}", e);
        }
    }
}
```

## Examples

Run the included example:

```bash
cargo run --example simple
```

## API Documentation

### `CpuTemperature`

The main struct representing a temperature reading.

#### Fields

- `celsius: f64` - Temperature in degrees Celsius
- `fahrenheit: f64` - Temperature in degrees Fahrenheit

#### Methods

- `CpuTemperature::get() -> Result<CpuTemperature, String>` - Gets the current CPU temperature

## How It Works

This library uses Windows Management Instrumentation (WMI) to query thermal zone sensors through PowerShell. It:

1. Executes a PowerShell command to query `MSAcpi_ThermalZoneTemperature`
2. Parses the output to extract temperature values
3. Converts from the raw format (0.1 Kelvin units) to Celsius and Fahrenheit
4. Returns the first available temperature reading

## Error Handling

The library provides detailed error messages for common issues:

- PowerShell execution failures
- WMI query errors  
- Temperature sensor unavailability
- Parsing errors
- Permission issues

## Troubleshooting

If you encounter errors:

1. **"Failed to execute PowerShell"** - Ensure PowerShell is installed and in PATH
2. **"WMI query failed"** - Try running as administrator
3. **"No temperature data received"** - Your system may not have accessible thermal sensors
4. **"No valid temperature readings found"** - The thermal zone sensors may not be accessible

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

This library is designed to be a simple, reliable solution for CPU temperature monitoring on Windows systems.
