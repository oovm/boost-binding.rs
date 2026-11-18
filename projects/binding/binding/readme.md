# `binding`

外部绑定层门面：按编译目标导出当前平台可用的宿主底座与绑定样板宏。

| 入口 | 可用平台 | 内容 |
|------|----------|------|
| `binding::platform_module!` | 全部 | 平台门控模块声明与公开项 re-export |
| `binding::dbus` | 全部（`conn` 仅 Linux） | D-Bus 编解码与会话总线连接 |
| `binding::com` | Windows | COM apartment 守卫、`ComError`、`CoTaskMem` 回收 |

平台差异只体现在 `com` 是否存在。需要更细的控制时，可直接依赖 `binding-com` / `binding-dbus`。
