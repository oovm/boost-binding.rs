/**
 * shared-build - A plugin application that uses the shared library
 * 
 * This application demonstrates how to use the shared library
 * as a plugin to extend functionality, including features from ctmd-core.
 */

use asred::{greet, square, factorial, AppConfig};

fn main() {
    println!("=== Shared-Build: Plugin Application ===");
    
    // Test greet function from the shared library (wraps ctmd-core)
    let name = "Plugin User";
    let message = greet(name);
    println!("{}", message);
    
    // Test square function from the shared library (additional functionality)
    let number = 7;
    let square_result = square(number);
    println!("Square of {} is: {}", number, square_result);
    
    // Test factorial function from the shared library (from ctmd-core)
    let factorial_number = 7;
    let factorial_result = factorial(factorial_number);
    println!("Factorial of {} is: {}", factorial_number, factorial_result);
    
    // Test AppConfig struct and conversion to core config
    let config = AppConfig::new("shared-build", "1.0.0");
    println!("Plugin configuration: {}", config.to_string());
    
    // Convert to core config and use core functionality
    let core_config = config.to_core_config();
    println!("Core configuration: {}", core_config.to_string());
    
    println!("Shared-build plugin executed successfully!");
}
