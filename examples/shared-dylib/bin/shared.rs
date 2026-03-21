/// shared - Main application using the shared library
///
/// This application demonstrates how to use the shared library
/// which wraps functionality from the ctmd-core library.
use asred::{AppConfig, factorial, square};

fn main() {
    println!("=== Shared: Main Application ===");

    // Test square function (additional functionality)
    let number = 5;
    let square_result = square(number);
    println!("Square of {} is: {}", number, square_result);

    // Test factorial function (from ctmd-core)
    let factorial_number = 5;
    let factorial_result = factorial(factorial_number);
    println!("Factorial of {} is: {}", factorial_number, factorial_result);

    // Test AppConfig struct and conversion to core config
    let config = AppConfig::new("shared", "1.0.0");
    println!("App configuration: {}", config.to_string());

    // Convert to core config and use core functionality
    let core_config = config.to_core_config();
    println!("Core configuration: {}", core_config.to_string());

    println!("Shared application completed successfully!");
}
