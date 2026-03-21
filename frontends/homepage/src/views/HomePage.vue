<template>
  <section class="pt-24 pb-24 sm:pt-32 sm:pb-32 bg-one-dark-bg-dark relative overflow-hidden">
    <!-- One Dark Pro Background Effects -->
    <div class="absolute inset-0 z-0">
      <div class="absolute inset-0 bg-tech-bg opacity-30"></div>
      <div class="absolute inset-0 bg-grid-pattern bg-grid-pattern opacity-10"></div>
      <div class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-one-dark-accent to-transparent"></div>
      <div class="absolute bottom-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-one-dark-accent to-transparent"></div>
      <div class="absolute top-0 bottom-0 left-0 w-px bg-gradient-to-b from-transparent via-one-dark-accent to-transparent"></div>
      <div class="absolute top-0 bottom-0 right-0 w-px bg-gradient-to-b from-transparent via-one-dark-accent to-transparent"></div>
    </div>
    
    <!-- Particle Background -->
    <ParticleBackground />

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
      <!-- Hero Section with Animation -->
      <div class="text-center max-w-4xl mx-auto mb-16 sm:mb-24">
        <transition name="fade-up" appear>
          <h1 class="text-xl sm:text-2xl md:text-4xl lg:text-5xl font-bold leading-tight mb-4 sm:mb-6">
            <span class="bg-gradient-to-r from-one-dark-accent via-one-dark-cyan to-one-dark-accent bg-clip-text text-transparent">iTools</span>
            <br>
            <span class="text-one-dark-fg-light text-sm sm:text-base md:text-lg">高性能 · 可扩展 · 开发者工具框架</span>
          </h1>
        </transition>
        
        <transition name="fade-up" appear>
          <p class="text-sm sm:text-base md:text-lg text-one-dark-fg-dark mb-8 sm:mb-12 max-w-2xl mx-auto leading-relaxed">
            iTools 为开发者提供高性能、可扩展的交互式工具框架，让您的团队更高效地构建和管理开发工具
          </p>
        </transition>
        
        <transition name="fade-up" appear>
          <div class="flex flex-col sm:flex-row gap-4 sm:gap-6 justify-center items-center">
            <button 
              @click="router.push('/download')"
              @mousedown="isDownloadButtonPressed = true"
              @mouseup="isDownloadButtonPressed = false"
              @mouseleave="isDownloadButtonPressed = false"
              @touchstart="isDownloadButtonPressed = true"
              @touchend="isDownloadButtonPressed = false"
              class="group relative overflow-hidden px-6 sm:px-8 py-3 sm:py-4 rounded-md sm:rounded-lg text-sm sm:text-base font-medium bg-one-dark-accent text-one-dark-bg-darker shadow-md shadow-one-dark-accent/20 hover:shadow-one-dark-accent/40 transition-all duration-300 hover:-translate-y-0.5 hover:scale-105 border border-one-dark-accent/50 active:scale-95 w-full sm:w-auto"
              :class="{ 'scale-95': isDownloadButtonPressed }"
            >
              <span class="relative z-10 flex items-center justify-center gap-2 sm:gap-3">
                立即下载
                <svg class="w-4 h-4 sm:w-5 sm:h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path>
                </svg>
              </span>
              <div class="absolute inset-0 bg-gradient-to-r from-one-dark-accent via-one-dark-cyan to-one-dark-accent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            </button>
            <button 
              @click="router.push('/document')"
              @mousedown="isDocumentButtonPressed = true"
              @mouseup="isDocumentButtonPressed = false"
              @mouseleave="isDocumentButtonPressed = false"
              @touchstart="isDocumentButtonPressed = true"
              @touchend="isDocumentButtonPressed = false"
              class="group px-6 sm:px-8 py-3 sm:py-4 rounded-md sm:rounded-lg text-sm sm:text-base font-medium bg-one-dark-bg border border-one-dark-border text-one-dark-fg hover:bg-one-dark-bg-light hover:border-one-dark-accent shadow-sm hover:shadow-md transition-all duration-300 hover:-translate-y-0.5 hover:scale-105 active:scale-95 w-full sm:w-auto"
              :class="{ 'scale-95': isDocumentButtonPressed }"
            >
              <span class="flex items-center justify-center gap-2 sm:gap-3">
                <svg class="w-4 h-4 sm:w-5 sm:h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                </svg>
                查看文档
              </span>
            </button>
          </div>
        </transition>
      </div>

      <!-- Features Grid -->
      <div class="grid md:grid-cols-2 gap-6 sm:gap-8">
        <!-- Feature 1 -->
        <transition name="fade-up" appear>
          <div class="bg-one-dark-bg border border-one-dark-border rounded-lg p-6 sm:p-8 hover:border-one-dark-accent/50 transition-all duration-300 hover:shadow-md hover:shadow-one-dark-accent/10">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 rounded-md bg-one-dark-accent/10 border border-one-dark-accent/30 flex items-center justify-center">
                <div class="text-2xl sm:text-3xl text-one-dark-accent">⚡</div>
              </div>
              <h3 class="text-xl sm:text-2xl font-semibold text-one-dark-fg-light">高性能</h3>
            </div>
            <h4 class="text-lg font-medium text-one-dark-fg mb-4">基于 Rust 编译时架构</h4>
            <p class="text-sm sm:text-base text-one-dark-fg-dark leading-relaxed mb-6">采用 Rust 全链路编译器，提供极致的工具处理性能，即使处理复杂任务也能保持流畅响应。</p>
            <button class="flex items-center gap-2 text-one-dark-accent font-medium hover:text-one-dark-cyan transition-colors border-b border-one-dark-accent/30 hover:border-one-dark-cyan/50 pb-1 group">
              了解更多
              <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
              </svg>
            </button>
          </div>
        </transition>

        <!-- Feature 2 -->
        <transition name="fade-up" appear>
          <div class="bg-one-dark-bg border border-one-dark-border rounded-lg p-6 sm:p-8 hover:border-one-dark-purple/50 transition-all duration-300 hover:shadow-md hover:shadow-one-dark-purple/10">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 rounded-md bg-one-dark-purple/10 border border-one-dark-purple/30 flex items-center justify-center">
                <div class="text-2xl sm:text-3xl text-one-dark-purple">🔌</div>
              </div>
              <h3 class="text-xl sm:text-2xl font-semibold text-one-dark-fg-light">可扩展</h3>
            </div>
            <h4 class="text-lg font-medium text-one-dark-fg mb-4">灵活的插件系统</h4>
            <p class="text-sm sm:text-base text-one-dark-fg-dark leading-relaxed mb-6">支持丰富的插件系统，轻松集成各种高级功能，如 KaTeX 数学公式、Mermaid 图表等。</p>
            <button class="flex items-center gap-2 text-one-dark-purple font-medium hover:text-one-dark-cyan transition-colors border-b border-one-dark-purple/30 hover:border-one-dark-cyan/50 pb-1 group">
              了解更多
              <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
              </svg>
            </button>
          </div>
        </transition>

        <!-- Feature 3 -->
        <transition name="fade-up" appear>
          <div class="bg-one-dark-bg border border-one-dark-border rounded-lg p-6 sm:p-8 hover:border-one-dark-green/50 transition-all duration-300 hover:shadow-md hover:shadow-one-dark-green/10">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 rounded-md bg-one-dark-green/10 border border-one-dark-green/30 flex items-center justify-center">
                <div class="text-2xl sm:text-3xl text-one-dark-green">🌐</div>
              </div>
              <h3 class="text-xl sm:text-2xl font-semibold text-one-dark-fg-light">多语言支持</h3>
            </div>
            <h4 class="text-lg font-medium text-one-dark-fg mb-4">统一 URL 路径多语言切换</h4>
            <p class="text-sm sm:text-base text-one-dark-fg-dark leading-relaxed mb-6">在统一 URL 路径下实现多语言切换，无需维护不同路由，提供无缝的国际化体验。</p>
            <button class="flex items-center gap-2 text-one-dark-green font-medium hover:text-one-dark-cyan transition-colors border-b border-one-dark-green/30 hover:border-one-dark-cyan/50 pb-1 group">
              了解更多
              <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
              </svg>
            </button>
          </div>
        </transition>

        <!-- Feature 4 -->
        <transition name="fade-up" appear>
          <div class="bg-one-dark-bg border border-one-dark-border rounded-lg p-6 sm:p-8 hover:border-one-dark-yellow/50 transition-all duration-300 hover:shadow-md hover:shadow-one-dark-yellow/10">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 rounded-md bg-one-dark-yellow/10 border border-one-dark-yellow/30 flex items-center justify-center">
                <div class="text-2xl sm:text-3xl text-one-dark-yellow">🎨</div>
              </div>
              <h3 class="text-xl sm:text-2xl font-semibold text-one-dark-fg-light">主题定制</h3>
            </div>
            <h4 class="text-lg font-medium text-one-dark-fg mb-4">完整的主题系统</h4>
            <p class="text-sm sm:text-base text-one-dark-fg-dark leading-relaxed mb-6">支持自定义组件和样式，提供完整的主题系统，让您的工具站点拥有独特的视觉风格。</p>
            <button class="flex items-center gap-2 text-one-dark-yellow font-medium hover:text-one-dark-cyan transition-colors border-b border-one-dark-yellow/30 hover:border-one-dark-cyan/50 pb-1 group">
              了解更多
              <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
              </svg>
            </button>
          </div>
        </transition>
      </div>

      <!-- iTools CLI Simulator -->
      <transition name="fade-up" appear>
        <div class="mt-20 sm:mt-28">
          <div class="text-center mb-8 sm:mb-12">
            <div class="text-one-dark-accent text-xs sm:text-sm font-semibold mb-3 sm:mb-4 flex items-center gap-2 justify-center">
              <span class="w-1 h-6 bg-one-dark-accent rounded-full"></span>
              🖥️ 命令行模拟器
            </div>
            <h3 class="text-xl sm:text-2xl md:text-3xl font-bold text-one-dark-fg-light mb-3 sm:mb-4">体验 iTools 命令行</h3>
            <p class="text-sm sm:text-base text-one-dark-fg-dark leading-relaxed mb-6 sm:mb-8 max-w-2xl mx-auto">在浏览器中模拟 iTools 命令行操作，体验各种命令的执行效果。</p>
          </div>
          <div class="bg-one-dark-bg border border-one-dark-border rounded-lg shadow-md overflow-hidden">
            <IToolsSimulator />
          </div>
        </div>
      </transition>
    </div>
  </section>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { IToolsSimulator, ParticleBackground } from "../components";
import { ref } from "vue";

const router = useRouter();

// Button press states for click feedback
const isDownloadButtonPressed = ref(false);
const isDocumentButtonPressed = ref(false);
</script>

<style scoped>
/* Fade up animation */
.fade-up-enter-active,
.fade-up-leave-active {
  transition: all 0.6s ease-out;
}

.fade-up-enter-from,
.fade-up-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

/* Staggered animation delays */
.fade-up-enter-active {
  animation: fadeUp 0.8s ease-out forwards;
}

/* Smooth hover effects */
button {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes fadeUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Intersection Observer for scroll-based animations */
@media (prefers-reduced-motion: no-preference) {
  .fade-up {
    animation-duration: 0.8s;
    animation-fill-mode: both;
  }
}
</style>