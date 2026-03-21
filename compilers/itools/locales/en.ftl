# 命令
commands.init = Initialize a new project
commands.build = Build the project
commands.dev = Start development server
commands.check = Check the project

# 错误
errors.expected_argument = Expected argument
errors.expected_string_argument = Expected string argument
errors.expected_integer_argument = Expected integer argument
errors.expected_unsigned_integer_argument = Expected unsigned integer argument
errors.expected_boolean_argument = Expected boolean argument
errors.expected_command = Expected command
errors.unknown_command = Unknown command: {command}
errors.invalid_integer = Invalid integer
errors.invalid_unsigned_integer = Invalid unsigned integer
errors.invalid_boolean = Invalid boolean
errors.config_not_found = Configuration file not found: {path}
errors.config_parse_error = Configuration parse error: {error}

# 帮助
help.usage = Usage: itools <command> [args]
help.commands = Commands:
help.init_usage = init <name> - Initialize a new project with the given name
help.build_usage = build - Build the project
help.dev_usage = dev - Start development server
help.check_usage = check - Check the project
help.language_switch = Use LANG environment variable to switch language (e.g., LANG=en or LANG=zh-CN)

# 消息
messages.cli_initialized = iTools CLI initialized
messages.initializing_project = Initializing project: {name}
messages.building_project = Building project
messages.starting_dev_server = Starting development server
messages.checking_project = Checking project
messages.loading_config = Loading configuration
messages.loading_config_fragment = Loading configuration fragment: {path}

# 复数形式示例
messages.item_count = { $count ->
    [one] There is { $count } item
    *[other] There are { $count } items
}

# 性别示例
messages.greeting = { $gender ->
    [male] Hello, Mr. { $name }!
    [female] Hello, Ms. { $name }!
    *[other] Hello, { $name }!
}
