<template>
  <footer class="relative z-10 border-t border-one-dark-border bg-one-dark-bg-dark">
    <!-- One Dark Pro Background Effects -->
    <div class="absolute inset-0 z-0">
      <div class="absolute inset-0 bg-tech-bg opacity-20"></div>
      <div class="absolute inset-0 bg-grid-pattern bg-grid-pattern opacity-5"></div>
    </div>
    
    <div class="max-w-7xl mx-auto px-6 lg:px-8 py-12 relative z-10">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-8 mb-8">
        <div class="md:col-span-1">
          <div class="flex items-center gap-3 mb-4">
            <div class="w-8 h-8 bg-gradient-to-br from-one-dark-accent via-one-dark-cyan to-one-dark-purple rounded-md flex items-center justify-center shadow-md shadow-one-dark-accent/20">
              <span class="text-one-dark-bg-darker font-bold text-lg">i</span>
            </div>
            <span class="text-xl font-bold text-one-dark-fg-light">iTools</span>
          </div>
          <p class="text-one-dark-fg-dark text-sm leading-relaxed">
            专业游戏配置导表工具，让游戏开发更高效、更准确。
          </p>
        </div>
        
        <div>
          <h4 class="text-one-dark-fg-light font-semibold mb-4">产品</h4>
          <ul class="space-y-2">
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">功能特性</a></li>
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">下载</a></li>
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">更新日志</a></li>
          </ul>
        </div>
        
        <div>
          <h4 class="text-one-dark-fg-light font-semibold mb-4">资源</h4>
          <ul class="space-y-2">
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">文档</a></li>
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">教程</a></li>
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">API</a></li>
          </ul>
        </div>
        
        <div>
          <h4 class="text-one-dark-fg-light font-semibold mb-4">社区</h4>
          <ul class="space-y-2">
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">GitHub</a></li>
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">Discord</a></li>
            <li><a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">Twitter</a></li>
          </ul>
        </div>
      </div>
      
      <div class="pt-6 border-t border-one-dark-border flex flex-col md:flex-row justify-between items-center gap-4">
        <p class="text-one-dark-fg-dark text-sm">
          © 2026 iTools. All rights reserved.
        </p>
        <div class="flex items-center gap-6">
          <a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">隐私政策</a>
          <a href="#" class="text-one-dark-fg hover:text-one-dark-accent transition-colors text-sm">服务条款</a>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
// Footer component
</script>