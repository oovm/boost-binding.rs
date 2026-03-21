# Shared Dylib Example

This example demonstrates how multiple applications can share a single dynamic library (dylib) to minimize application size and reduce code duplication.

## Directory Structure

```
shared-dylib/
├── Cargo.toml          # Workspace configuration
├── libapp/             # Shared dynamic library
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── app1/               # Basic binary application
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── app-run/            # Plugin application
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── README.md           # This documentation
```

## What's Included

### libapp (Shared Dynamic Library)
- A shared library containing common functionality that can be used by multiple applications
- Includes:
  - `greet()` function: Generates a greeting message
  - `square()` function: Calculates the square of a number
  - `AppConfig` struct: Represents an application configuration

### app1 (Basic Binary Application)
- A small basic application that uses the `greet()` function from the shared library
- Demonstrates basic usage of the shared library

### app-run (Plugin Application)
- A plugin application that uses multiple functions from the shared library
- Demonstrates more complex usage of the shared library as a plugin

## Benefits of Shared Libraries

1. **Code Reuse**: Common functionality is defined once and used by multiple applications
2. **Smaller Binaries**: Each application only contains its specific code, not the shared library code
3. **Easier Maintenance**: Changes to the shared library are automatically available to all applications
4. **Consistent Behavior**: All applications use the same implementation of shared functionality

## How to Build and Run

1. **Build all applications**:
   ```bash
   cargo build
   ```

2. **Run app1 (Basic Application)**:
   ```bash
   cargo run -p app1
   ```

3. **Run app-run (Plugin Application)**:
   ```bash
   cargo run -p app-run
   ```

## Expected Output

### app1 Output:
```
=== App1: Basic Application ===
Hello, World! Welcome to libapp shared library.
App1 completed successfully!
```

### app-run Output:
```
=== App-Run: Plugin Application ===
Hello, Plugin User! Welcome to libapp shared library.
Square of 7 is: 49
Plugin configuration: app-run v1.0.0
App-run plugin executed successfully!
```

## How It Works

1. The `libapp` library is defined as a separate crate with public functions and structs
2. Both `app1` and `app-run` declare a dependency on `libapp` in their Cargo.toml files
3. When built, each application only includes its own code, not the shared library code
4. At runtime, both applications use the same shared library functionality

## Key Files to Look At

- `libapp/src/lib.rs`: Contains the shared library implementation
- `app1/src/main.rs`: Shows how to use the `greet()` function
- `app-run/src/main.rs`: Shows how to use multiple functions and structs from the shared library

This example demonstrates the power of shared libraries in Rust, allowing multiple applications to share code while keeping each application small and focused.
