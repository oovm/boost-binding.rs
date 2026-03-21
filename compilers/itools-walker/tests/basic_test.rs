use itools_walker::{WalkOptions, Walker};

#[tokio::test]
async fn test_basic_walk() {
    let walker = Walker::new();

    // Test walking the current directory
    let entries = walker.walk(".").await.unwrap();

    // At least one entry should be found (the current directory itself)
    assert!(!entries.is_empty());

    // Check that the first entry is the current directory
    let first_entry = &entries[0];
    assert!(first_entry.is_dir);
    assert_eq!(first_entry.path.file_name().unwrap_or_default(), std::ffi::OsStr::new("."));
}

#[tokio::test]
async fn test_walk_with_options() {
    let options = WalkOptions {
        recursive: false, // Only walk the current directory
        follow_symlinks: false,
    };

    let walker = Walker::new().with_options(options);
    let entries = walker.walk(".").await.unwrap();

    // Should only have one entry (the current directory itself)
    assert_eq!(entries.len(), 1);
    assert!(entries[0].is_dir);
}

#[tokio::test]
async fn test_walk_with_callback() {
    let walker = Walker::new();

    let mut count = 0;
    let result = walker
        .walk_with(".", |entry| {
            count += 1;
            Ok(())
        })
        .await;

    assert!(result.is_ok());
    assert!(count > 0); // Should have processed at least one entry
}
