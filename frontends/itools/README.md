# iTools TypeScript 前端实现

## 概述

这是 iTools 项目的 TypeScript 前端实现，提供与后端 Rust 版本对应的 TypeScript 接口和功能。

## 特性

- 命令解析功能，支持 init、build、dev、check 命令
- 配置管理功能，支持加载和解析配置文件
- 国际化功能，支持多语言
- 支持浏览器和 Node.js 环境
- 与后端 Rust 版本保持 API 一致性

## 安装

```bash
# 从项目根目录安装依赖
pnpm install

# 构建项目
pnpm build
```

## 使用

### 命令行使用

```bash
# 初始化项目
pnpm exec itools init my-project

# 构建项目
pnpm exec itools build

# 启动开发服务器
pnpm exec itools dev

# 检查项目
pnpm exec itools check
```

### 作为库使用

```typescript
import { Cli, runCli, ConfigManager, i18n } from '@itools/frontend'

// 使用命令行解析
const cli = Cli.parse(['init', 'my-project'])
await cli.execute()

// 使用配置管理
const configManager = ConfigManager.fromObject({
  title: 'My Project',
  description: 'A project using iTools',
  lang: 'zh-CN'
})
const config = configManager.getConfig()

// 使用国际化
const message = i18n.t('messages.cli_initialized')
const messageWithArgs = i18n.tWithArgs('messages.initializing_project', { name: 'my-project' })
```

## 目录结构

```
frontends/itools/
├── src/
│   ├── __tests__/       # 测试文件
│   ├── cli.ts           # 命令行解析
│   ├── config.ts        # 配置管理
│   ├── i18n.ts          # 国际化
│   ├── types.ts         # 类型定义
│   └── index.ts         # 入口文件
├── dist/                # 构建输出
├── package.json         # 项目配置
├── tsconfig.json        # TypeScript 配置
├── tsconfig.node.json   # Node.js TypeScript 配置
├── vite.config.ts       # Vite 配置
└── README.md            # 文档
```

## 测试

```bash
# 运行测试
pnpm test

# 运行类型检查
pnpm typecheck
```

## 构建

```bash
# 构建项目
pnpm build
```

## 环境支持

- Node.js 18+
- 现代浏览器

## 贡献

欢迎贡献代码和提出问题！
