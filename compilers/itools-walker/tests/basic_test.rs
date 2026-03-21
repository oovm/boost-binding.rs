use itools_walker::{WalkOptions, Walker};
use futures::stream::StreamExt;
use std::num::NonZeroU32;

#[tokio::test]
async fn test_basic_walk() {
    let walker = Walker::new().unwrap();

    // Test walking the current directory
    let mut stream = walker.walk(".").await.unwrap();

    // At least one entry should be found (the current directory itself)
    let mut found = false;
    while let Some(entry) = stream.next().await {
        if let Ok(walk_entry) = entry {
            found = true;
            // Check that the entry is a directory
            assert!(walk_entry.is_dir());
            break;
        }
    }
    assert!(found);
}

#[tokio::test]
async fn test_walk_with_options() {
    let options = WalkOptions {
        recursive: false, // Only walk the current directory
        follow_symlinks: false,
        channel_size: NonZeroU32::new(64).unwrap(),
        concurrency_limit: NonZeroU32::new(10).unwrap(),
    };

    let walker = Walker::with_options(options).unwrap();
    let mut stream = walker.walk(".").await.unwrap();

    // Should only have one entry (the current directory itself)
    let mut count = 0;
    while let Some(entry) = stream.next().await {
        if entry.is_ok() {
            count += 1;
        }
    }
    assert_eq!(count, 1);
}

#[tokio::test]
async fn test_walk_with_callback() {
    let walker = Walker::new().unwrap();

    let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count_clone = count.clone();
    let result = walker
        .walk_with(".", move |_entry| {
            count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        })
        .await;

    assert!(result.is_ok());
    assert!(count.load(std::sync::atomic::Ordering::SeqCst) > 0); // Should have processed at least one entry
}
