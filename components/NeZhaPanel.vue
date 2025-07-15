<!-- 哪吒监控面板嵌入组件 -->
<template>
  <div class="nezha-container" :class="{ 'fade-in': isLoaded, 'fade-out': isLeaving }">
    <div class="loading-overlay" v-if="!isLoaded">
      <div class="loading-content">
        <div class="spinner"></div>
        <div class="loading-text">正在加载监控面板...</div>
        <div class="loading-progress">
          <div class="progress-bar" :style="{ width: `${loadingProgress}%` }"></div>
        </div>
      </div>
    </div>
    <iframe 
      ref="iframeRef"
      :src="url" 
      frameborder="0" 
      @load="handleIframeLoaded"
      allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
      allowfullscreen
      style="visibility: hidden"
      :style="{ visibility: isLoaded ? 'visible' : 'hidden' }">
    </iframe>
  </div>
</template>
  
<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vitepress'

const props = defineProps({
  url: {
    type: String,
    required: true
  }
})

const isLoaded = ref(false)
const isLeaving = ref(false)
const loadingProgress = ref(0)
const router = useRouter()
const iframeRef = ref(null)

// 模拟加载进度
function simulateLoading() {
  const interval = setInterval(() => {
    if (loadingProgress.value < 90) {
      loadingProgress.value += Math.random() * 10
      if (loadingProgress.value > 90) {
        loadingProgress.value = 90
      }
    }
    
    if (isLoaded.value) {
      loadingProgress.value = 100
      clearInterval(interval)
    }
  }, 300)
  
  return interval
}

function handleIframeLoaded() {
  // 确保iframe内容已完全渲染
  try {
    // 检查iframe是否真的加载完成
    const iframeDoc = iframeRef.value.contentDocument || iframeRef.value.contentWindow.document
    
    // 设置最终进度为100%
    loadingProgress.value = 100
    
    // 延迟一点显示，确保iframe内容已完全渲染
    setTimeout(() => {
      isLoaded.value = true
      // 移除全局加载状态
      document.documentElement.classList.remove('nezha-loading')
    }, 500)
  } catch (e) {
    // 如果出现跨域问题，则假设已加载完成
    console.warn('无法访问iframe内容，可能是跨域限制', e)
    loadingProgress.value = 100
    setTimeout(() => {
      isLoaded.value = true
      // 移除全局加载状态
      document.documentElement.classList.remove('nezha-loading')
    }, 500)
  }
}

// 监听路由变化，添加退出动画
function handleRouteChange(to) {
  if (!to.endsWith('/nezha')) {
    isLeaving.value = true
    // 给退出动画一些时间
    return new Promise(resolve => {
      setTimeout(resolve, 300)
    })
  }
}

onMounted(() => {
  // 开始模拟加载进度
  const loadingInterval = simulateLoading()
  
  // 添加路由变化监听
  if (router) {
    const originalBeforeChange = router.onBeforeRouteChange
    router.onBeforeRouteChange = async (to) => {
      await handleRouteChange(to)
      if (originalBeforeChange) {
        return originalBeforeChange(to)
      }
    }
  }
  
  // 如果iframe加载时间过长，设置超时处理
  const timeoutId = setTimeout(() => {
    if (!isLoaded.value) {
      console.warn('iframe加载超时，强制显示')
      loadingProgress.value = 100
      isLoaded.value = true
      // 移除全局加载状态
      document.documentElement.classList.remove('nezha-loading')
      clearInterval(loadingInterval)
    }
  }, 10000) // 10秒超时
  
  // 清理函数
  onBeforeUnmount(() => {
    clearInterval(loadingInterval)
    clearTimeout(timeoutId)
    isLeaving.value = true
    // 确保移除全局加载状态
    document.documentElement.classList.remove('nezha-loading')
  })
})
</script>

<style scoped>
.nezha-container {
  position: fixed;
  top: var(--vp-nav-height);
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 10;
  background-color: var(--vp-c-bg);
  overflow: hidden;
  opacity: 0;
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.nezha-container.fade-in {
  opacity: 1;
}

.nezha-container.fade-out {
  opacity: 0;
  transform: translateY(20px);
}

.nezha-container iframe {
  width: 100%;
  height: 100%;
  border: none;
  overflow: hidden;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--vp-c-bg);
  z-index: 20;
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading-content {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.spinner {
  width: 60px;
  height: 60px;
  border: 6px solid rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  border-top-color: var(--vp-c-brand);
  animation: spin 1s ease-in-out infinite;
}

.loading-text {
  margin-top: 20px;
  font-size: 18px;
  color: var(--vp-c-text-1);
  font-weight: 500;
}

.loading-progress {
  width: 200px;
  height: 6px;
  background-color: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
  margin-top: 20px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background-color: var(--vp-c-brand);
  border-radius: 3px;
  transition: width 0.3s ease;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

:root.dark .spinner {
  border-color: rgba(255, 255, 255, 0.1);
  border-top-color: var(--vp-c-brand);
}

:root.dark .loading-progress {
  background-color: rgba(255, 255, 255, 0.1);
}
</style> 