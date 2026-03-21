# 进度条

进度条是 iTools 框架中用于显示任务执行进度的重要组件，它可以直观地向用户展示操作的完成情况，提升用户体验。作为一个 Rust 框架，iTools 提供了类型安全、高性能的进度条实现。

## 基本概念

进度条是一种可视化组件，用于表示一个操作的完成程度，通常以水平条的形式显示，填充部分表示已完成的比例。在 Rust 中，进度条被实现为类型安全的结构体，提供了丰富的配置选项和方法。

### 核心属性

- **当前值**：当前进度值
- **最小值**：进度条的起始值（通常为 0）
- **最大值**：进度条的结束值（通常为 100）
- **百分比**：当前进度占总进度的百分比
- **状态**：进度条的状态（正常、成功、失败等）

## 使用方法

### 基本用法

在 iTools 框架中，您可以通过以下方式创建和使用进度条：

```rust
use itools::ui::progress::ProgressBar;

// 创建进度条
let mut progress_bar = ProgressBar::new(0, 100);
progress_bar.set_label(Some("任务进度"));

// 更新进度
progress_bar.set_value(50);

// 完成进度
progress_bar.set_value(100);
```

### 配置选项

| 选项 | 类型 | 默认值 | 描述 |
| :--- | :--- | :--- | :--- |
| min | u64 | 0 | 进度条的最小值 |
| max | u64 | 100 | 进度条的最大值 |
| value | u64 | 0 | 当前进度值 |
| label | Option<String> | None | 进度条标签 |
| show_percentage | bool | true | 是否显示百分比 |
| status | ProgressStatus | ProgressStatus::Normal | 进度条状态（Normal、Success、Error、Warning） |
| color | Color | Color::Blue | 进度条颜色 |
| height | u32 | 8 | 进度条高度（像素） |
| animated | bool | true | 是否启用动画效果 |

## 高级特性

### 分段进度条

分段进度条允许您将进度划分为多个阶段，每个阶段可以有不同的颜色和标签：

```rust
use itools::ui::progress::{SegmentedProgressBar, Segment};

let segments = vec![
    Segment::new("准备", 25, Color::Blue),
    Segment::new("处理", 25, Color::Green),
    Segment::new("验证", 25, Color::Yellow),
    Segment::new("完成", 25, Color::Red),
];

let mut segmented_progress = SegmentedProgressBar::new(segments);

// 更新特定阶段的进度
segmented_progress.update_segment(1, 50); // 更新第二个阶段到50%
```

### 环形进度条

环形进度条以圆形方式显示进度，适用于空间有限的场景：

```rust
use itools::ui::progress::CircularProgressBar;

let mut circular_progress = CircularProgressBar::new(
    75,      // 当前值
    100,     // 最大值
    80,      // 大小（像素）
    8,       // 厚度（像素）
    Color::Blue,
    Some("75%".to_string())
);

// 更新进度
circular_progress.set_value(90);
```

## 事件处理

进度条支持多种事件，您可以通过回调函数来处理这些事件：

```rust
use itools::ui::progress::ProgressBar;

let mut progress_bar = ProgressBar::new(0, 100);

// 设置进度变化回调
progress_bar.on_change(|value| {
    println!("进度更新为: {}%", value);
});

// 设置完成回调
progress_bar.on_complete(|| {
    println!("任务完成！");
});

// 设置错误回调
progress_bar.on_error(|error| {
    eprintln!("发生错误: {:?}", error);
});
```

## 使用场景

### 1. 文件上传/下载

```rust
use itools::ui::progress::ProgressBar;
use std::fs::File;
use std::io::{self, Read, Write};

fn upload_file(file_path: &str, destination: &str) -> io::Result<()>
    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    
    let mut progress_bar = ProgressBar::new(0, file_size);
    progress_bar.set_label(Some("文件上传"));
    
    let mut reader = file;
    let mut writer = File::create(destination)?;
    let mut buffer = [0; 1024 * 1024]; // 1MB 缓冲区
    let mut bytes_read = 0;
    
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        writer.write_all(&buffer[..n])?;
        bytes_read += n as u64;
        progress_bar.set_value(bytes_read);
    }
    
    Ok(())
}
```

### 2. 数据处理

```rust
use itools::ui::progress::ProgressBar;

fn process_items(items: &[String]) {
    let total_items = items.len() as u64;
    let mut progress_bar = ProgressBar::new(0, total_items);
    progress_bar.set_label(Some("数据处理"));
    
    for (index, item) in items.iter().enumerate() {
        // 处理项目
        process_item(item);
        
        // 更新进度
        progress_bar.set_value((index + 1) as u64);
    }
}

fn process_item(item: &str) {
    // 处理逻辑
    println!("处理项目: {}", item);
}
```

### 3. 安装/更新

```rust
use itools::ui::progress::ProgressBar;

fn install_package(package_name: &str) {
    let mut progress_bar = ProgressBar::new(0, 100);
    progress_bar.set_label(Some("安装中"));
    progress_bar.set_show_percentage(true);
    
    // 模拟安装过程
    for i in 0..=100 {
        // 执行安装步骤
        simulate_install_step(i);
        
        // 更新进度
        progress_bar.set_value(i);
        
        // 模拟耗时
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn simulate_install_step(percentage: u64) {
    // 模拟安装步骤
    if percentage % 20 == 0 {
        println!("安装进度: {}%", percentage);
    }
}
```

## 最佳实践

1. **合理设置进度范围**：根据实际任务的复杂度设置合适的最小值和最大值
2. **提供有意义的标签**：使用清晰的标签让用户了解正在进行的操作
3. **实时更新进度**：确保进度条能够及时反映任务的实际进展
4. **处理错误情况**：当任务失败时，使用错误状态的进度条向用户反馈
5. **考虑用户体验**：对于长时间运行的任务，提供取消选项和详细的进度信息
6. **利用 Rust 的类型安全**：充分利用 Rust 的类型系统，确保进度条的使用是类型安全的
7. **异步支持**：对于异步任务，使用异步回调来更新进度条

## 示例代码

### 基本进度条

```rust
use itools::ui::progress::ProgressBar;

fn main() {
    let mut progress_bar = ProgressBar::new(0, 100);
    progress_bar.set_label(Some("任务进度"));
    progress_bar.set_show_percentage(true);
    
    // 模拟任务执行
    for i in 0..=100 {
        progress_bar.set_value(i);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
```

### 带状态的进度条

```rust
use itools::ui::progress::{ProgressBar, ProgressStatus};

fn main() {
    let mut progress_bar = ProgressBar::new(0, 100);
    progress_bar.set_label(Some("安装进度"));
    
    // 模拟安装过程
    for i in 0..=100 {
        progress_bar.set_value(i);
        
        // 根据进度设置不同状态
        if i < 30 {
            progress_bar.set_status(ProgressStatus::Normal);
        } else if i < 70 {
            progress_bar.set_status(ProgressStatus::Warning);
        } else {
            progress_bar.set_status(ProgressStatus::Success);
        }
        
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
```

### 环形进度条

```rust
use itools::ui::progress::CircularProgressBar;

fn main() {
    let mut circular_progress = CircularProgressBar::new(
        0, 100, 80, 6, Color::Green, None
    );
    
    // 模拟任务执行
    for i in 0..=100 {
        circular_progress.set_value(i);
        circular_progress.set_label(Some(format!("{}%", i)));
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
```

## 总结

进度条是 iTools 框架中一个重要的用户界面组件，它可以帮助用户了解任务的执行进度，提升用户体验。作为一个 Rust 框架，iTools 提供了类型安全、高性能的进度条实现，支持多种进度条类型和配置选项。

通过合理配置和使用进度条，您可以为用户提供清晰、直观的反馈，使长时间运行的操作变得更加透明和可控。同时，Rust 的类型安全特性确保了进度条的使用更加可靠和安全。