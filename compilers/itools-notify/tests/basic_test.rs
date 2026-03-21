//! 基本测试用例

use std::{fs::File, io::Write, path::PathBuf, thread, time::Duration};

use itools_notify::{FileSystemEvent, PathFilter, new_watcher};

#[test]
fn test_basic_monitoring() {
    // 创建临时目录
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // 创建监控器
    let mut watcher = new_watcher().unwrap();

    // 开始监控临时目录
    watcher.watch(temp_path, true).unwrap();

    // 创建测试文件
    let test_file = temp_path.join("test.txt");
    let mut file = File::create(&test_file).unwrap();
    writeln!(file, "Hello, world!").unwrap();

    // 等待事件
    thread::sleep(Duration::from_millis(100));

    // 接收事件
    let event = watcher.next_event().unwrap();
    assert!(event.is_some());

    // 验证事件类型
    if let Some(FileSystemEvent::Create(path)) = event {
        assert_eq!(path, test_file);
    }
    else {
        panic!("Expected Create event, got {:?}", event);
    }
}

#[test]
fn test_event_filtering() {
    // 创建临时目录
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // 创建监控器
    let mut watcher = new_watcher().unwrap();

    // 添加过滤器，排除 .txt 文件
    let filter = PathFilter::new(vec![], vec![".txt".to_string()]);
    watcher.add_filter(Box::new(filter));

    // 开始监控临时目录
    watcher.watch(temp_path, true).unwrap();

    // 创建测试文件
    let test_file = temp_path.join("test.txt");
    let mut file = File::create(&test_file).unwrap();
    writeln!(file, "Hello, world!").unwrap();

    // 等待事件
    thread::sleep(Duration::from_millis(100));

    // 接收事件，应该被过滤
    let event = watcher.next_event().unwrap();
    assert!(event.is_none());
}
