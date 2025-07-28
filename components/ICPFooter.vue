<template>
  <div class="icp-footer">
    <div class="icp-content">
      <p class="icp-text">
        <a href="https://beian.miit.gov.cn/" target="_blank" rel="noopener noreferrer">
          冀ICP备2025123259号-1
        </a>
      </p>
    </div>
  </div>
</template>

<script setup>
// ICP备案信息组件
</script>

<style scoped>
.icp-footer {
  background-color: var(--vp-c-bg-soft);
  border-top: 1px solid var(--vp-c-divider);
  padding: 16px 0;
  margin-top: 32px;
  /* 防止布局跳动的关键样式 */
  min-height: 56px;
  opacity: 0;
  transform: translateY(10px);
  animation: fadeInUp 0.3s ease-out 0.1s forwards;
  /* 使用 will-change 优化性能 */
  will-change: opacity, transform;
}

.icp-content {
  max-width: 1152px;
  margin: 0 auto;
  padding: 0 24px;
  text-align: center;
  /* 确保内容稳定 */
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 24px;
}

.icp-text {
  margin: 0;
  font-size: 14px;
  color: var(--vp-c-text-2);
  line-height: 1.5;
  /* 防止文字跳动 */
  white-space: nowrap;
}

.icp-text a {
  color: var(--vp-c-text-2);
  text-decoration: none;
  transition: color 0.2s ease;
}

.icp-text a:hover {
  color: var(--vp-c-brand);
  text-decoration: underline;
}

/* 淡入动画 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .icp-footer {
    min-height: 48px;
  }
  
  .icp-content {
    padding: 0 16px;
  }
  
  .icp-text {
    font-size: 12px;
    white-space: normal;
    text-align: center;
  }
}

/* 减少重绘和回流 */
.icp-footer * {
  box-sizing: border-box;
}
</style>