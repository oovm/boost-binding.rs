#![warn(missing_docs)]

/// asred - A shared dynamic library for ctmd applications
///
/// This library provides common functionality that can be shared across multiple ctmd applications,
/// reducing code duplication and keeping application binaries small.
/// It also wraps and extends functionality from the ctmd-core library.
use ctmd_core::{Config as CoreConfig, factorial as core_factorial, greet as core_greet};

/// A function to greet users (wraps ctmd-core's greet function)
///
/// @param name The name to greet
/// @return A formatted greeting string
pub fn greet(name: &str) -> String {
    core_greet(name)
}

/// A function to calculate the factorial of a number (wraps ctmd-core's factorial function)
///
/// @param n The number to calculate the factorial for
/// @return The factorial of the given number
pub fn factorial(n: u64) -> u64 {
    core_factorial(n)
}

/// A function to calculate the square of a number (additional functionality)
///
/// @param num The number to calculate the square for
/// @return The square of the given number
pub fn square(num: i32) -> i32 {
    num * num
}

/// A struct representing an application configuration
///
/// @field name The name of the application
/// @field version The version of the application
pub struct AppConfig {
    /// The name of the application
    pub name: String,
    /// The version of the application
    pub version: String,
}

impl AppConfig {
    /// Creates a new AppConfig instance
    ///
    /// @param name The name of the application
    /// @param version The version of the application
    /// @return A new AppConfig instance
    pub fn new(name: &str, version: &str) -> Self {
        Self { name: name.to_string(), version: version.to_string() }
    }

    /// Gets the configuration as a string
    ///
    /// @return A string representation of the configuration
    pub fn to_string(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }

    /// Converts to ctmd-core's Config struct
    ///
    /// @return A CoreConfig instance
    pub fn to_core_config(&self) -> CoreConfig {
        CoreConfig::new(&self.name, &self.version)
    }
}
