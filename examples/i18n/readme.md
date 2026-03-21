# 国际化示例

这是一个展示 ctmd 国际化功能的示例应用。

## 功能

- 展示如何使用 `lang!` 宏进行国际化
- 支持根据语言环境显示不同的消息

## 构建和运行

```bash
# 构建
cargo build

# 运行
cargo run -- greet John

# 运行帮助命令
cargo run -- help
```

## 示例输出

英文环境：
```
Hello, John!
Welcome to the internationalization example!
```

中文环境：
```
你好，John！
欢迎使用国际化示例！
```
