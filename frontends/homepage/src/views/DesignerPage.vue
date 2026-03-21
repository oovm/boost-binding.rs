<template>
  <section class="pt-32 pb-32">
    <div class="max-w-7xl mx-auto px-6 lg:px-8">
      <div class="text-center max-w-4xl mx-auto mb-16">
        <div class="text-primary-600 text-sm font-semibold mb-4 flex items-center gap-2 justify-center">
          <span class="w-1 h-6 bg-primary-600 rounded-full"></span>
          🛠️ 代码生成器
        </div>
        <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold leading-tight mb-6">
          <span class="bg-gradient-to-r from-primary-600 via-primary-500 to-primary-400 bg-clip-text text-transparent">
            iTools 设计器
          </span>
        </h1>
        <p class="text-xl text-slate-600 leading-relaxed">
          可视化配置您的 iTools 项目，自动生成高质量的 Rust 代码
        </p>
      </div>

      <div class="grid lg:grid-cols-2 gap-8">
        <div class="space-y-6">
          <div class="bg-white rounded-2xl p-6 shadow-xl border border-slate-200">
            <h2 class="text-2xl font-bold text-slate-800 mb-6 flex items-center gap-2">
              <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
              </svg>
              项目配置
            </h2>

            <div class="space-y-4">
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">项目名称</label>
                <input 
                  v-model="config.projectName" 
                  type="text" 
                  class="w-full px-4 py-3 rounded-xl border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                  placeholder="my-awesome-tool"
                />
              </div>

              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">命令名称</label>
                <input 
                  v-model="config.commandName" 
                  type="text" 
                  class="w-full px-4 py-3 rounded-xl border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                  placeholder="mycli"
                />
              </div>

              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">描述</label>
                <textarea 
                  v-model="config.description" 
                  class="w-full px-4 py-3 rounded-xl border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                  rows="3"
                  placeholder="这是一个使用 iTools 构建的命令行工具"
                ></textarea>
              </div>

              <div>
                <label class="flex items-center gap-2 cursor-pointer">
                  <input 
                    v-model="config.allowPlugins" 
                    type="checkbox" 
                    class="w-5 h-5 rounded-lg border border-slate-300 text-primary-600 focus:ring-2 focus:ring-primary-500"
                  />
                  <span class="text-sm font-semibold text-slate-700">允许插件支持</span>
                </label>
                <p class="text-xs text-slate-500 mt-1">当启用时，未知命令会尝试执行对应的插件（如：mycli-unknown）</p>
              </div>
            </div>
          </div>

          <div class="bg-white rounded-2xl p-6 shadow-xl border border-slate-200">
            <div class="flex items-center justify-between mb-6">
              <h2 class="text-2xl font-bold text-slate-800 flex items-center gap-2">
                <svg class="w-6 h-6 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path>
                </svg>
                子命令
              </h2>
              <button 
                @click="addCommand"
                class="flex items-center gap-2 px-4 py-2 rounded-xl bg-primary-600 text-white font-semibold hover:bg-primary-700 transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                </svg>
                添加命令
              </button>
            </div>

            <div class="space-y-4">
              <div v-for="(cmd, index) in config.commands" :key="index" class="bg-slate-50 rounded-xl p-4 border border-slate-200">
                <div class="flex items-center justify-between mb-4">
                  <span class="font-semibold text-slate-700">命令 {{ index + 1 }}</span>
                  <button 
                    v-if="config.commands.length > 1"
                    @click="removeCommand(index)"
                    class="text-red-500 hover:text-red-600 transition-colors"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>

                <div class="space-y-3">
                  <div>
                    <label class="block text-sm font-semibold text-slate-600 mb-1">名称</label>
                    <input 
                      v-model="cmd.name" 
                      type="text" 
                      class="w-full px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                      placeholder="greet"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-semibold text-slate-600 mb-1">描述</label>
                    <input 
                      v-model="cmd.description" 
                      type="text" 
                      class="w-full px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                      placeholder="问候命令"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-semibold text-slate-600 mb-1">类型</label>
                    <select 
                      v-model="cmd.type" 
                      class="w-full px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                    >
                      <option value="action">动作命令</option>
                      <option value="subcommands">带子命令</option>
                    </select>
                  </div>
                  <div v-if="cmd.type === 'action'" class="space-y-2">
                    <div class="flex items-center justify-between">
                      <span class="text-sm font-semibold text-slate-600">参数</span>
                      <button 
                        @click="addArgument(index)"
                        class="text-primary-600 hover:text-primary-700 text-sm font-semibold transition-colors"
                      >
                        + 添加参数
                      </button>
                    </div>
                    <div v-for="(arg, argIndex) in cmd.arguments" :key="argIndex" class="flex gap-2 items-center">
                      <input 
                        v-model="arg.name" 
                        type="text" 
                        class="flex-1 px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                        placeholder="参数名"
                      />
                      <select 
                        v-model="arg.type" 
                        class="px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                      >
                        <option value="String">String</option>
                        <option value="i32">i32</option>
                        <option value="u32">u32</option>
                        <option value="bool">bool</option>
                      </select>
                      <button 
                        @click="removeArgument(index, argIndex)"
                        class="text-red-500 hover:text-red-600 transition-colors"
                      >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                      </button>
                    </div>
                  </div>
                  
                  <div v-if="cmd.type === 'subcommands'" class="space-y-4">
                    <div class="flex items-center justify-between">
                      <span class="text-sm font-semibold text-slate-600">子命令</span>
                      <button 
                        @click="addSubcommand(index)"
                        class="text-primary-600 hover:text-primary-700 text-sm font-semibold transition-colors"
                      >
                        + 添加子命令
                      </button>
                    </div>
                    <div v-for="(subcmd, subcmdIndex) in cmd.subcommands" :key="subcmdIndex" class="bg-white rounded-lg p-3 border border-slate-200">
                      <div class="flex items-center justify-between mb-3">
                        <span class="text-sm font-medium text-slate-700">子命令 {{ subcmdIndex + 1 }}</span>
                        <button 
                          @click="removeSubcommand(index, subcmdIndex)"
                          class="text-red-500 hover:text-red-600 transition-colors"
                        >
                          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                          </svg>
                        </button>
                      </div>
                      <div class="space-y-2">
                        <input 
                          v-model="subcmd.name" 
                          type="text" 
                          class="w-full px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                          placeholder="子命令名称"
                        />
                        <input 
                          v-model="subcmd.description" 
                          type="text" 
                          class="w-full px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                          placeholder="子命令描述"
                        />
                        <select 
                          v-model="subcmd.type" 
                          class="w-full px-3 py-2 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
                        >
                          <option value="action">动作命令</option>
                          <option value="subcommands">带子命令</option>
                        </select>
                        <div v-if="subcmd.type === 'action'" class="space-y-2">
                          <div class="flex items-center justify-between">
                            <span class="text-xs font-semibold text-slate-600">参数</span>
                            <button 
                              @click="addArgument(index)"
                              class="text-primary-600 hover:text-primary-700 text-xs font-semibold transition-colors"
                            >
                              + 添加参数
                            </button>
                          </div>
                          <div v-for="(arg, argIndex) in subcmd.arguments" :key="argIndex" class="flex gap-2 items-center">
                            <input 
                              v-model="arg.name" 
                              type="text" 
                              class="flex-1 px-2 py-1 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-xs"
                              placeholder="参数名"
                            />
                            <select 
                              v-model="arg.type" 
                              class="px-2 py-1 rounded-lg border border-slate-300 focus:ring-2 focus:ring-primary-500 focus:border-transparent text-xs"
                            >
                              <option value="String">String</option>
                              <option value="i32">i32</option>
                              <option value="u32">u32</option>
                              <option value="bool">bool</option>
                            </select>
                            <button 
                              @click="removeArgument(index, argIndex)"
                              class="text-red-500 hover:text-red-600 transition-colors"
                            >
                              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                              </svg>
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div class="bg-slate-900 rounded-2xl p-6 shadow-xl">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-xl font-bold text-white flex items-center gap-2">
                <svg class="w-6 h-6 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path>
                </svg>
                生成的代码
              </h2>
              <div class="flex gap-2">
                <button 
                  @click="copyCode"
                  class="flex items-center gap-2 px-4 py-2 rounded-lg bg-slate-700 text-white text-sm font-semibold hover:bg-slate-600 transition-colors"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                  </svg>
                  {{ copied ? '已复制!' : '复制代码' }}
                </button>
                <button 
                  @click="downloadCode"
                  class="flex items-center gap-2 px-4 py-2 rounded-lg bg-emerald-600 text-white text-sm font-semibold hover:bg-emerald-700 transition-colors"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                  </svg>
                  下载
                </button>
              </div>
            </div>
            <div class="bg-slate-800 rounded-xl p-4 overflow-x-auto">
              <div v-html="highlightedCode" class="text-sm"></div>
            </div>
          </div>

          <div class="bg-blue-50 rounded-2xl p-6 border border-blue-200">
            <h3 class="text-lg font-bold text-blue-800 mb-3 flex items-center gap-2">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              使用提示
            </h3>
            <ul class="text-blue-700 space-y-2 text-sm">
              <li>• 配置好项目信息后，代码会自动更新</li>
              <li>• 点击「复制代码」按钮快速复制到剪贴板</li>
              <li>• 点击「下载」按钮保存为 main.rs 文件</li>
              <li>• 生成的代码遵循 iTools 最佳实践</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import * as shiki from "shiki";

interface Argument {
	name: string;
	type: string;
}

interface Command {
	name: string;
	description: string;
	type: "action" | "subcommands";
	arguments: Argument[];
	subcommands: Command[];
}

interface Config {
	projectName: string;
	commandName: string;
	description: string;
	allowPlugins: boolean;
	commands: Command[];
}

const config = ref<Config>({
	projectName: "my-awesome-tool",
	commandName: "mycli",
	description: "这是一个使用 iTools 构建的命令行工具",
	allowPlugins: false,
	commands: [
		{
			name: "greet",
			description: "问候命令",
			type: "action",
			arguments: [
				{ name: "name", type: "String" },
				{ name: "age", type: "u32" }
			],
			subcommands: []
		},
		{
			name: "calculate",
			description: "计算命令",
			type: "action",
			arguments: [
				{ name: "a", type: "i32" },
				{ name: "b", type: "i32" }
			],
			subcommands: []
		}
	]
});

const copied = ref(false);
const highlightedCode = ref("");

const generatedCode = computed(() => {
	// 生成子命令枚举的递归函数
	function generateSubcommandEnum(command: Command, prefix: string): string {
		const enumName = `${prefix}${toPascalCase(command.name)}Command`;
		let code = `/// ${command.description} 命令枚举
#[Command]
#[derive(Debug)]
pub enum ${enumName} {
`;

		command.subcommands.forEach(subcmd => {
			code += `    /// ${subcmd.description}\n`;
			if (subcmd.type === "action" && subcmd.arguments.length > 0) {
				code += `    ${toPascalCase(subcmd.name)} {\n`;
				subcmd.arguments.forEach(arg => {
					code += `        /// ${arg.name} 参数\n`;
					code += `        ${arg.name}: ${arg.type},\n`;
				});
				code += `    },\n`;
			} else if (subcmd.type === "subcommands" && subcmd.subcommands.length > 0) {
				// 生成子命令的枚举
				code += `    ${toPascalCase(subcmd.name)}(${generateSubcommandEnum(subcmd, enumName)}),\n`;
			} else {
				code += `    ${toPascalCase(subcmd.name)},\n`;
			}
		});

		code += `}

`;
		return code;
	}

	// 生成执行子命令的递归函数
	function generateSubcommandMatch(command: Command, prefix: string): string {
		let code = ``;

		command.subcommands.forEach(subcmd => {
			if (subcmd.type === "action" && subcmd.arguments.length > 0) {
				const argNames = subcmd.arguments.map(a => a.name).join(", ");
				code += `        ${prefix}::${toPascalCase(subcmd.name)} { ${argNames} } => {\n`;
				code += `            println!("Executing ${command.name} ${subcmd.name} command");\n`;
				subcmd.arguments.forEach(arg => {
					code += `            println!("${arg.name}: {:?}", ${arg.name});\n`;
				});
				code += `        },\n`;
			} else if (subcmd.type === "subcommands" && subcmd.subcommands.length > 0) {
				const subEnumName = `${prefix}${toPascalCase(subcmd.name)}Command`;
				code += `        ${prefix}::${toPascalCase(subcmd.name)}(subcmd) => {\n`;
				code += `            match subcmd {\n`;
				code += generateSubcommandMatch(subcmd, subEnumName);
				code += `            }\n`;
				code += `        },\n`;
			} else {
				code += `        ${prefix}::${toPascalCase(subcmd.name)} => {\n`;
				code += `            println!("Executing ${command.name} ${subcmd.name} command");\n`;
				code += `        },\n`;
			}
		});

		return code;
	}

	let code = `//! ${config.value.description}

use itools::{Parse, init_cli};
use itools_macros::Command;
use std::process;

/// ${config.value.commandName} 命令行工具
#[Command]
#[derive(Debug)]
pub struct ${toPascalCase(config.value.commandName)} {
    /// 命令
    pub command: ${toPascalCase(config.value.commandName)}Command,
}

/// ${config.value.commandName} 命令枚举
#[Command]
#[derive(Debug)]
pub enum ${toPascalCase(config.value.commandName)}Command {
`;

	config.value.commands.forEach(cmd => {
		code += `    /// ${cmd.description}\n`;
		if (cmd.type === "action" && cmd.arguments.length > 0) {
			code += `    ${toPascalCase(cmd.name)} {\n`;
			cmd.arguments.forEach(arg => {
				code += `        /// ${arg.name} 参数\n`;
				code += `        ${arg.name}: ${arg.type},\n`;
			});
			code += `    },\n`;
		} else if (cmd.type === "subcommands" && cmd.subcommands.length > 0) {
			// 生成子命令的枚举
			code += `    ${toPascalCase(cmd.name)}(${toPascalCase(config.value.commandName)}${toPascalCase(cmd.name)}Command),\n`;
		} else {
			code += `    ${toPascalCase(cmd.name)},\n`;
		}
	});

	code += `}

`;

	// 生成子命令的枚举
	config.value.commands.forEach(cmd => {
		if (cmd.type === "subcommands" && cmd.subcommands.length > 0) {
			code += generateSubcommandEnum(cmd, `${toPascalCase(config.value.commandName)}`);
		}
	});

	code += `fn main() {
    // 初始化 CLI 环境
    init_cli();

    // 获取命令行参数
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // 解析命令行参数
`;

	if (config.value.allowPlugins) {
		code += `    let result = ${toPascalCase(config.value.commandName)}::parse(&mut args);
    match result {
        Ok(cli) => {
            // 执行命令
            match cli.command {
`;
	} else {
		code += `    let cli = ${toPascalCase(config.value.commandName)}::parse(&mut args);

    // 执行命令
    match cli.command {
`;
	}

	config.value.commands.forEach(cmd => {
		if (cmd.type === "action" && cmd.arguments.length > 0) {
			const argNames = cmd.arguments.map(a => a.name).join(", ");
			code += `        ${toPascalCase(config.value.commandName)}Command::${toPascalCase(cmd.name)} { ${argNames} } => {\n`;
			code += `            println!("Executing ${cmd.name} command");\n`;
			cmd.arguments.forEach(arg => {
				code += `            println!("${arg.name}: {:?}", ${arg.name});\n`;
			});
			code += `        },\n`;
		} else if (cmd.type === "subcommands" && cmd.subcommands.length > 0) {
			const subEnumName = `${toPascalCase(config.value.commandName)}${toPascalCase(cmd.name)}Command`;
			code += `        ${toPascalCase(config.value.commandName)}Command::${toPascalCase(cmd.name)}(subcmd) => {\n`;
			code += `            match subcmd {\n`;
			code += generateSubcommandMatch(cmd, subEnumName);
			code += `            }\n`;
			code += `        },\n`;
		} else {
			code += `        ${toPascalCase(config.value.commandName)}Command::${toPascalCase(cmd.name)} => {\n`;
			code += `            println!("Executing ${cmd.name} command");\n`;
			code += `        },\n`;
		}
	});

	if (config.value.allowPlugins) {
		code += `    }        },
        Err(err) => {
            // 尝试执行插件
            if let Some(first_arg) = args.get(0) {
                let plugin_name = format!("{}-{}", "${config.value.commandName}", first_arg);
                println!("Unknown command, trying to execute plugin: {}", plugin_name);
                
                let status = process::Command::new(plugin_name)
                    .args(&args[1..])
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("Error executing plugin: {}", e);
                        process::exit(1);
                    });
                
                process::exit(status.code().unwrap_or(1));
            } else {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    }
`;
	} else {
		code += `    }
}
`;
	}

	return code;
});

function toPascalCase(str: string): string {
	return str
		.split(/[-_]/)
		.map(word => word.charAt(0).toUpperCase() + word.slice(1))
		.join("");
}

function addCommand() {
	config.value.commands.push({
		name: "newcmd",
		description: "新命令",
		type: "action",
		arguments: [],
		subcommands: []
	});
}

function removeCommand(index: number) {
	config.value.commands.splice(index, 1);
}

function addArgument(cmdIndex: number) {
	config.value.commands[cmdIndex].arguments.push({
		name: "param",
		type: "String"
	});
}

function addSubcommand(cmdIndex: number) {
	config.value.commands[cmdIndex].subcommands.push({
		name: "subcmd",
		description: "子命令",
		type: "action",
		arguments: [],
		subcommands: []
	});
}

function removeSubcommand(cmdIndex: number, subcmdIndex: number) {
	config.value.commands[cmdIndex].subcommands.splice(subcmdIndex, 1);
}

function removeArgument(cmdIndex: number, argIndex: number) {
	config.value.commands[cmdIndex].arguments.splice(argIndex, 1);
}

async function copyCode() {
	try {
		await navigator.clipboard.writeText(generatedCode.value);
		copied.value = true;
		setTimeout(() => {
			copied.value = false;
		}, 2000);
	} catch (err) {
		console.error("Failed to copy code:", err);
	}
}

function downloadCode() {
	const blob = new Blob([generatedCode.value], { type: "text/rust" });
	const url = URL.createObjectURL(blob);
	const a = document.createElement("a");
	a.href = url;
	a.download = "main.rs";
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);
	URL.revokeObjectURL(url);
}

async function updateHighlighting() {
	try {
		const highlighted = await shiki.codeToHtml(generatedCode.value, {
			lang: "rust",
			theme: "github-dark"
		});
		highlightedCode.value = highlighted;
	} catch (err) {
		console.error("Failed to highlight code:", err);
		highlightedCode.value = `<pre><code>${generatedCode.value}</code></pre>`;
	}
}

watch(generatedCode, () => {
	updateHighlighting();
}, { immediate: true });

onMounted(() => {
	updateHighlighting();
});
</script>
