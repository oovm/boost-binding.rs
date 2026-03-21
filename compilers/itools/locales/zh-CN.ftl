# 命令
commands.init = 初始化新项目
commands.build = 构建项目
commands.dev = 启动开发服务器
commands.check = 检查项目

# 错误
errors.expected_argument = 缺少参数
errors.expected_string_argument = 缺少字符串参数
errors.expected_integer_argument = 缺少整数参数
errors.expected_unsigned_integer_argument = 缺少无符号整数参数
errors.expected_boolean_argument = 缺少布尔参数
errors.expected_command = 缺少命令
errors.unknown_command = 未知命令: {command}
errors.invalid_integer = 无效的整数
errors.invalid_unsigned_integer = 无效的无符号整数
errors.invalid_boolean = 无效的布尔值
errors.config_not_found = 配置文件未找到: {path}
errors.config_parse_error = 配置解析错误: {error}

# 帮助
help.usage = 用法: itools <命令> [参数]
help.commands = 命令:
help.init_usage = init <名称> - 初始化一个新项目
help.build_usage = build - 构建项目
help.dev_usage = dev - 启动开发服务器
help.check_usage = check - 检查项目
help.language_switch = 使用 LANG 环境变量切换语言 (例如: LANG=en 或 LANG=zh-CN)

# 消息
messages.cli_initialized = iTools CLI 已初始化
messages.initializing_project = 正在初始化项目: {name}
messages.building_project = 正在构建项目
messages.starting_dev_server = 正在启动开发服务器
messages.checking_project = 正在检查项目
messages.loading_config = 正在加载配置
messages.loading_config_fragment = 正在加载配置片段: {path}

# 复数形式示例
messages.item_count = { $count ->
    [one] 有 { $count } 个项目
    *[other] 有 { $count } 个项目
}

# 性别示例
messages.greeting = { $gender ->
    [male] 你好，{ $name }先生！
    [female] 你好，{ $name }女士！
    *[other] 你好，{ $name }！
}
