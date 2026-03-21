<template>
  <div class="particle-background absolute inset-0 z-0 overflow-hidden pointer-events-none">
    <canvas ref="canvas" class="w-full h-full"></canvas>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const canvas = ref<HTMLCanvasElement | null>(null);
let animationId: number | null = null;
let particles: Particle[] = [];
let isVisible = true;

interface Particle {
  x: number;
  y: number;
  size: number;
  speedX: number;
  speedY: number;
  color: string;
  opacity: number;
}

// 节流函数
function throttle<T extends (...args: any[]) => any>(func: T, delay: number): (...args: Parameters<T>) => void {
  let lastCall = 0;
  return (...args: Parameters<T>) => {
    const now = Date.now();
    if (now - lastCall >= delay) {
      lastCall = now;
      func(...args);
    }
  };
}

function createParticle(canvasWidth: number, canvasHeight: number): Particle {
  // One Dark Pro 主题颜色
  const colors = ['#61afef', '#c678dd', '#98c379', '#56b6c2'];
  return {
    x: Math.random() * canvasWidth,
    y: Math.random() * canvasHeight,
    size: Math.random() * 3 + 1,
    speedX: (Math.random() - 0.5) * 0.5,
    speedY: (Math.random() - 0.5) * 0.5,
    color: colors[Math.floor(Math.random() * colors.length)],
    opacity: Math.random() * 0.5 + 0.2
  };
}

function initParticles(canvasWidth: number, canvasHeight: number) {
  particles = [];
  // 根据设备性能和屏幕尺寸调整粒子数量
  const isMobile = window.innerWidth < 768;
  const baseCount = isMobile ? 30 : 60;
  const particleCount = Math.min(baseCount, Math.floor((canvasWidth * canvasHeight) / 15000));
  for (let i = 0; i < particleCount; i++) {
    particles.push(createParticle(canvasWidth, canvasHeight));
  }
}

function drawParticles() {
  if (!canvas.value || !isVisible) return;
  
  const ctx = canvas.value.getContext('2d');
  if (!ctx) return;
  
  const canvasWidth = canvas.value.width;
  const canvasHeight = canvas.value.height;
  
  // Clear canvas
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);
  
  // Draw particles
  for (let i = 0; i < particles.length; i++) {
    const particle = particles[i];
    
    // Draw particle
    ctx.beginPath();
    ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
    ctx.fillStyle = `${particle.color}${Math.floor(particle.opacity * 255).toString(16).padStart(2, '0')}`;
    ctx.fill();
    
    // Update position
    particle.x += particle.speedX;
    particle.y += particle.speedY;
    
    // Wrap around edges
    if (particle.x < 0) particle.x = canvasWidth;
    if (particle.x > canvasWidth) particle.x = 0;
    if (particle.y < 0) particle.y = canvasHeight;
    if (particle.y > canvasHeight) particle.y = 0;
  }
  
  // Draw connections between particles (optimized)
  const connectionDistance = 80;
  for (let i = 0; i < particles.length; i++) {
    const particleA = particles[i];
    for (let j = i + 1; j < particles.length; j++) {
      const particleB = particles[j];
      
      const dx = particleA.x - particleB.x;
      const dy = particleA.y - particleB.y;
      const distance = Math.sqrt(dx * dx + dy * dy);
      
      if (distance < connectionDistance) {
        ctx.beginPath();
        ctx.strokeStyle = `${particleA.color}${Math.floor((0.2 - distance / 400) * 255).toString(16).padStart(2, '0')}`;
        ctx.lineWidth = 0.3;
        ctx.moveTo(particleA.x, particleA.y);
        ctx.lineTo(particleB.x, particleB.y);
        ctx.stroke();
      }
    }
  }
  
  animationId = requestAnimationFrame(drawParticles);
}

const throttledHandleResize = throttle(() => {
  if (!canvas.value) return;
  
  canvas.value.width = window.innerWidth;
  canvas.value.height = window.innerHeight;
  initParticles(canvas.value.width, canvas.value.height);
}, 200);

function handleVisibilityChange() {
  isVisible = !document.hidden;
  if (isVisible && !animationId) {
    drawParticles();
  }
}

onMounted(() => {
  if (!canvas.value) return;
  
  canvas.value.width = window.innerWidth;
  canvas.value.height = window.innerHeight;
  
  initParticles(canvas.value.width, canvas.value.height);
  drawParticles();
  
  window.addEventListener('resize', throttledHandleResize);
  document.addEventListener('visibilitychange', handleVisibilityChange);
});

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId);
  }
  window.removeEventListener('resize', throttledHandleResize);
  document.removeEventListener('visibilitychange', handleVisibilityChange);
});
</script>

<style scoped>
.particle-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
  overflow: hidden;
  pointer-events: none;
}
</style>