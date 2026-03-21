# iTools Walker

Asynchronous directory traversal library with concurrent processing and glob pattern support.

## Features

- **Asynchronous**: Built on Tokio for non-blocking I/O operations
- **Concurrent**: Parallel directory traversal with configurable concurrency limits
- **Glob Pattern Support**: Includes and excludes files using glob patterns (like .gitignore)
- **Flexible API**: Easy-to-use interface with both callback-based and stream-based methods
- **Symbolic Link Support**: Optional symbolic link following
- **Recursive Traversal**: Configurable recursive directory traversal

## Usage Examples

### Basic Async Traversal with Stream

```rust
use itools_walker::Walker;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a walker with default options
    let walker = Walker::new()?;
    
    // Walk the current directory asynchronously (returns a stream)
    let mut stream = walker.walk(".").await?;
    
    // Process the results stream
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(walk_entry) => println!("{:?} - dir: {}, file: {}, symlink: {}", 
                walk_entry.path, walk_entry.is_dir(), walk_entry.is_file(), walk_entry.is_symlink()),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    
    Ok(())
}
```

### Concurrent Traversal with Concurrency Limit

```rust
use itools_walker::{Walker, WalkOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create options with concurrency limit
    let options = WalkOptions {
        concurrency_limit: Some(5), // Limit to 5 concurrent operations
        ..Default::default()
    };
    
    let walker = Walker::with_options(options)?;
    
    // Walk with callback (more memory efficient for large directories)
    walker.walk_with(".", |entry| {
        println!("Processing: {:?}", entry.path);
        Ok(())
    }).await?;
    
    Ok(())
}
```

### Glob Pattern Support

```rust
use itools_walker::{Walker, WalkOptions};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create options with glob patterns
    let options = WalkOptions {
        include_patterns: vec!["**/*.rs", "**/*.toml"], // Include Rust and TOML files
        exclude_patterns: vec!["target/**", ".git/**"], // Exclude target and .git directories
        ..Default::default()
    };
    
    let walker = Walker::with_options(options)?;
    
    let mut stream = walker.walk(".").await?;
    
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(walk_entry) if walk_entry.is_file() => {
                println!("Found file: {:?}", walk_entry.path);
            }
            Err(e) => println!("Error: {:?}", e),
            _ => {}
        }
    }
    
    Ok(())
}
```

### Walk with Convenient Functions

```rust
use itools_walker::{walk, walk_with_options, WalkOptions};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the convenient function with default options
    let mut stream = walk(".").await?;
    let mut count = 0;
    
    while let Some(entry) = stream.next().await {
        if entry.is_ok() {
            count += 1;
        }
    }
    println!("Found {} entries", count);
    
    // Create options with glob patterns
    let options = WalkOptions {
        include_patterns: vec!["**/*.rs"],
        exclude_patterns: vec!["target/**"],
        concurrency_limit: Some(10),
        ..Default::default()
    };
    
    // Use the convenient function with custom options
    let mut rust_stream = walk_with_options(".", options).await?;
    let mut rust_count = 0;
    
    while let Some(entry) = rust_stream.next().await {
        if let Ok(walk_entry) = entry {
            if walk_entry.is_file() {
                rust_count += 1;
            }
        }
    }
    
    println!("Found {} Rust files", rust_count);
    
    Ok(())
}
```

## Performance Benefits

- **Non-blocking**: Uses Tokio's async I/O for efficient directory traversal
- **Parallel Processing**: Concurrent traversal speeds up processing of large directory structures
- **Memory Efficient**: Stream-based API allows processing entries as they are found
- **Flexible**: Configurable concurrency limits to balance performance and system resources

## Use Cases

- **File System Analysis**: Quickly scan large directory structures
- **Build Tools**: Efficiently find and process source files
- **Static Site Generators**: Traverse content directories with pattern matching
- **Dependency Scanners**: Identify files matching specific patterns
- **Backup Utilities**: Select files for backup based on glob patterns

## Why Choose itools-walker?

- **Modern Async Design**: Built with Tokio for optimal performance
- **Concurrent Processing**: Parallel traversal for faster results
- **Glob Pattern Support**: Powerful filtering capabilities
- **Flexible API**: Choose between stream-based or callback-based approaches
- **Reliable**: Handles errors gracefully and provides clear error messages
- **Well Documented**: Comprehensive documentation and examples

## License

AGPL-3.0
