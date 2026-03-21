<template>
  <div class="itools-simulator">
    <div class="terminal-header">
      <div class="terminal-title">iTools CLI Simulator</div>
      <div class="terminal-controls">
        <div class="control-dot red"></div>
        <div class="control-dot yellow"></div>
        <div class="control-dot green"></div>
      </div>
    </div>
    <div class="terminal-body">
      <div class="terminal-output" ref="outputRef">
        <div v-for="(line, index) in output" :key="index" class="output-line">
          <span v-if="line.type === 'prompt'" class="prompt">$ </span>
          <span v-if="line.type === 'command'" class="command">{{ line.content }}</span>
          <span v-if="line.type === 'output'" class="output">{{ line.content }}</span>
          <span v-if="line.type === 'error'" class="error">{{ line.content }}</span>
        </div>
        <div class="input-line">
          <span class="prompt">$ </span>
          <input
            v-model="input"
            @keydown.enter="executeCommand"
            @keydown.up="navigateHistory(-1)"
            @keydown.down="navigateHistory(1)"
            class="command-input"
            ref="inputRef"
            placeholder="Type a command..."
          />
          <span class="cursor"></span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'

// Types for terminal output
interface OutputLine {
  type: 'prompt' | 'command' | 'output' | 'error'
  content: string
}

// State
const output = ref<OutputLine[]>([])
const input = ref('')
const history = ref<string[]>([])
const historyIndex = ref(-1)
const outputRef = ref<HTMLElement>()
const inputRef = ref<HTMLInputElement>()

// Command types
const commandTypes: ('init' | 'build' | 'dev' | 'check')[] = ['init', 'build', 'dev', 'check']

// Mock i18n messages
const messages = {
  en: {
    'errors.expected_string_argument': 'Expected string argument',
    'errors.expected_command': 'Expected command',
    'errors.unknown_command': 'Unknown command: {command}',
    'messages.initializing_project': 'Initializing project: {name}',
    'messages.building_project': 'Building project',
    'messages.starting_dev_server': 'Starting development server',
    'messages.checking_project': 'Checking project'
  },
  'zh-CN': {
    'errors.expected_string_argument': '期望字符串参数',
    'errors.expected_command': '期望命令',
    'errors.unknown_command': '未知命令: {command}',
    'messages.initializing_project': '正在初始化项目: {name}',
    'messages.building_project': '正在构建项目',
    'messages.starting_dev_server': '正在启动开发服务器',
    'messages.checking_project': '正在检查项目'
  }
}

// Get current locale
const getLocale = (): string => {
  return navigator.language === 'zh-CN' ? 'zh-CN' : 'en'
}

// Translate function
const t = (key: string, args?: Record<string, string>): string => {
  const locale = getLocale()
  let message = messages[locale as keyof typeof messages][key as keyof typeof messages.en] || 
               messages.en[key as keyof typeof messages.en] || key
  
  if (args) {
    for (const [name, value] of Object.entries(args)) {
      message = message.replace(`{${name}}`, value)
    }
  }
  
  return message
}

// Execute command
const executeCommand = () => {
  const command = input.value.trim()
  if (!command) return
  
  // Add to history
  history.value.push(command)
  historyIndex.value = history.value.length
  
  // Add command to output
  output.value.push({ type: 'command', content: command })
  
  // Parse and execute command
  const args = command.split(' ')
  const cmd = args[0]
  
  try {
    if (!cmd) {
      throw new Error(t('errors.expected_command'))
    }
    
    if (!commandTypes.includes(cmd as any)) {
      throw new Error(t('errors.unknown_command', { command: cmd }))
    }
    
    switch (cmd) {
      case 'init':
        if (args.length < 2) {
          throw new Error(t('errors.expected_string_argument'))
        }
        output.value.push({ 
          type: 'output', 
          content: t('messages.initializing_project', { name: args[1] }) 
        })
        break
      case 'build':
        output.value.push({ type: 'output', content: t('messages.building_project') })
        break
      case 'dev':
        output.value.push({ type: 'output', content: t('messages.starting_dev_server') })
        break
      case 'check':
        output.value.push({ type: 'output', content: t('messages.checking_project') })
        break
    }
  } catch (error) {
    output.value.push({ type: 'error', content: error instanceof Error ? error.message : String(error) })
  }
  
  // Clear input
  input.value = ''
  
  // Scroll to bottom
  nextTick(() => {
    outputRef.value?.scrollTo(0, outputRef.value.scrollHeight)
    inputRef.value?.focus()
  })
}

// Navigate history
const navigateHistory = (direction: number) => {
  const newIndex = historyIndex.value + direction
  if (newIndex >= 0 && newIndex < history.value.length) {
    historyIndex.value = newIndex
    input.value = history.value[newIndex]
  } else if (newIndex === history.value.length) {
    historyIndex.value = newIndex
    input.value = ''
  }
}

// Initialize
onMounted(() => {
  // Welcome message
  output.value.push({ 
    type: 'output', 
    content: 'iTools CLI Simulator' 
  })
  output.value.push({ 
    type: 'output', 
    content: 'Type commands like: init, build, dev, check' 
  })
  output.value.push({ 
    type: 'output', 
    content: '' 
  })
  
  // Focus input
  nextTick(() => {
    inputRef.value?.focus()
  })
})
</script>

<style scoped>
.itools-simulator {
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 255, 163, 0.2), 0 0 0 1px rgba(0, 255, 163, 0.1);
  background-color: #0a0a0a;
  color: #00ffa3;
  font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  overflow: hidden;
  max-width: 100%;
  margin: 0 auto;
  border: 1px solid rgba(0, 255, 163, 0.3);
  position: relative;
  transition: all 0.3s ease;
}

.itools-simulator:hover {
  box-shadow: 0 12px 40px rgba(0, 255, 163, 0.3), 0 0 0 1px rgba(0, 255, 163, 0.2);
}

@media (min-width: 640px) {
  .itools-simulator {
    font-size: 14px;
  }
}

.terminal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: linear-gradient(135deg, #0f0f0f 0%, #1a1a1a 100%);
  border-bottom: 1px solid rgba(0, 255, 163, 0.2);
  backdrop-filter: blur(10px);
}

.terminal-title {
  font-size: 12px;
  color: rgba(0, 255, 163, 0.8);
  font-weight: 500;
  letter-spacing: 0.5px;
  text-shadow: 0 0 10px rgba(0, 255, 163, 0.5);
}

.terminal-controls {
  display: flex;
  gap: 8px;
}

.control-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 1px solid rgba(255, 255, 255, 0.1);
  position: relative;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.3);
}

.control-dot:hover {
  transform: scale(1.1);
  box-shadow: 0 0 10px rgba(255, 255, 255, 0.2);
}

.control-dot::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%) scale(0.6);
  width: 60%;
  height: 60%;
  border-radius: 50%;
  background: currentColor;
  opacity: 0.8;
}

.control-dot.red {
  background-color: rgba(255, 85, 85, 0.2);
  color: #ff5555;
  box-shadow: 0 0 8px rgba(255, 85, 85, 0.4);
}

.control-dot.yellow {
  background-color: rgba(255, 180, 0, 0.2);
  color: #ffb400;
  box-shadow: 0 0 8px rgba(255, 180, 0, 0.4);
}

.control-dot.green {
  background-color: rgba(0, 255, 163, 0.2);
  color: #00ffa3;
  box-shadow: 0 0 8px rgba(0, 255, 163, 0.4);
}

.terminal-body {
  padding: 16px;
  background: #0a0a0a;
  position: relative;
  overflow: hidden;
}

.terminal-body::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    linear-gradient(rgba(0, 255, 163, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(0, 255, 163, 0.03) 1px, transparent 1px);
  background-size: 20px 20px;
  pointer-events: none;
  z-index: 0;
}

.terminal-body::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(circle at 50% 50%, rgba(0, 255, 163, 0.05) 0%, transparent 70%);
  pointer-events: none;
  z-index: 0;
}

.terminal-output {
  max-height: 400px;
  overflow-y: auto;
  padding-bottom: 12px;
  position: relative;
  z-index: 1;
}

@media (min-width: 640px) {
  .terminal-output {
    max-height: 450px;
  }
}

.output-line {
  margin-bottom: 6px;
  white-space: pre-wrap;
  position: relative;
  z-index: 1;
}

.prompt {
  color: #00ffa3;
  font-weight: 600;
  text-shadow: 0 0 5px rgba(0, 255, 163, 0.5);
}

.command {
  color: #00ffa3;
  font-weight: 600;
  text-shadow: 0 0 5px rgba(0, 255, 163, 0.3);
}

.output {
  color: rgba(0, 255, 163, 0.9);
}

.error {
  color: #ff5555;
  text-shadow: 0 0 5px rgba(255, 85, 85, 0.5);
}

.input-line {
  display: flex;
  align-items: center;
  margin-top: 12px;
  position: relative;
  z-index: 1;
}

.command-input {
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: #00ffa3;
  font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  font-size: 13px;
  margin-left: 6px;
  caret-color: #00ffa3;
}

@media (min-width: 640px) {
  .command-input {
    font-size: 14px;
  }
}

/* Cursor animation */
.cursor {
  display: inline-block;
  width: 8px;
  height: 1.2em;
  background-color: #00ffa3;
  margin-left: 2px;
  animation: blink 1s infinite;
  vertical-align: text-bottom;
}

@keyframes blink {
  0%, 50% {
    opacity: 1;
  }
  51%, 100% {
    opacity: 0;
  }
}

/* Custom scrollbar */
.terminal-output::-webkit-scrollbar {
  width: 8px;
}

.terminal-output::-webkit-scrollbar-track {
  background: rgba(10, 10, 10, 0.8);
  border-left: 1px solid rgba(0, 255, 163, 0.1);
}

.terminal-output::-webkit-scrollbar-thumb {
  background: rgba(0, 255, 163, 0.5);
  border-radius: 4px;
  transition: background 0.2s ease;
}

.terminal-output::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 255, 163, 0.8);
  box-shadow: 0 0 10px rgba(0, 255, 163, 0.5);
}

/* Animation for new output lines */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(5px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.output-line {
  animation: fadeIn 0.3s ease-out;
}

/* Glow effect for terminal */
.itools-simulator::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  border-radius: 12px;
  box-shadow: 0 0 30px rgba(0, 255, 163, 0.1);
  pointer-events: none;
  z-index: -1;
}
</style>