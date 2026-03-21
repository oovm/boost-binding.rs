use futures::stream::StreamExt;
use itools_walker::{WalkOptions, Walker};
use std::num::NonZeroU32;

#[tokio::test]
async fn test_stream_walk() {
    let walker = Walker::new().unwrap();

    // Test streaming walk of the current directory
    let mut stream = walker.walk(".").await.unwrap();

    let mut count = 0;
    // Collect some entries to verify the stream works
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(walk_entry) => {
                count += 1;
                // At least one entry should be found (the current directory itself)
                if count == 1 {
                    assert!(walk_entry.is_dir());
                    assert_eq!(walk_entry.path.file_name().unwrap_or_default(), std::ffi::OsStr::new("."));
                }
            }
            Err(e) => panic!("Error during stream walk: {:?}", e),
        }
        // Stop after collecting 10 entries to avoid too much output
        if count >= 10 {
            break;
        }
    }

    assert!(count > 0); // Should have processed at least one entry
}

#[tokio::test]
async fn test_stream_walk_with_options() {
    let options = WalkOptions {
        recursive: false, // Only walk the current directory
        follow_symlinks: false,
        channel_size: NonZeroU32::new(64).unwrap(),
        concurrency_limit: NonZeroU32::new(10).unwrap(),
    };

    let walker = Walker::with_options(options).unwrap();
    let mut stream = walker.walk(".").await.unwrap();

    let mut count = 0;
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(_) => count += 1,
            Err(e) => panic!("Error during stream walk: {:?}", e),
        }
    }

    // Should have at least one entry (the current directory itself)
    assert!(count >= 1);
}

#[tokio::test]
async fn test_stream_walk_concurrency() {
    let options = WalkOptions {
        recursive: true,
        follow_symlinks: false,
        channel_size: NonZeroU32::new(64).unwrap(),
        concurrency_limit: NonZeroU32::new(5).unwrap(), // Set a lower concurrency limit for testing
    };

    let walker = Walker::with_options(options).unwrap();
    let mut stream = walker.walk(".").await.unwrap();

    let mut count = 0;
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(_) => count += 1,
            Err(e) => panic!("Error during stream walk: {:?}", e),
        }
        // Stop after collecting 20 entries to avoid too much output
        if count >= 20 {
            break;
        }
    }

    assert!(count > 0); // Should have processed at least one entry
}
