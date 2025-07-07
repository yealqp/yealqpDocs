<template>
  <div class="ncard cos-bucket-info">
    <div class="ncardBody">
      <div class="cos-title">COS存储桶信息</div>
      <div class="cos-row"><span class="cos-label">桶名：</span>{{ bucket }}</div>
      <div class="cos-row"><span class="cos-label">地区：</span>{{ region }}</div>
      <div class="cos-row"><span class="cos-label">存储容量：</span>
        <span v-if="loading">正在获取...</span>
        <span v-else-if="error">{{ error }}</span>
        <span v-else>{{ sizeGib }}GiB/50GiB</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const bucket = ref('')
const region = ref('')
const sizeGib = ref('')
const loading = ref(true)
const error = ref('')

async function fetchInfo() {
  loading.value = true
  error.value = ''
  try {
    // 获取后端配置（可选，假设后端返回桶名和地区，或直接写死）
    // 这里只请求容量
    const resp = await fetch('http://localhost:5000/api/cos_size')
    const data = await resp.json()
    if (data && data.success) {
      sizeGib.value = data.size_gib
    } else {
      error.value = data.error || '获取失败'
    }
  } catch (e) {
    error.value = '接口异常'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  // 这里桶名和地区直接写死，实际可从后端返回
  bucket.value = 'yea-1326788603'
  region.value = 'ap-beijing'
  fetchInfo()
})
</script>

<style scoped>
.ncard {
  border: 1px solid var(--vp-c-border);
  border-radius: 8px;
  margin-bottom: 2em;
  background: transparent;
  color: var(--vp-c-text-1);
  box-sizing: border-box;
}
.ncardBody {
  padding: 1.8em;
}
.cos-title {
  font-size: 1.3em;
  font-weight: bold;
  margin-bottom: 1em;
}
.cos-row {
  margin-bottom: 0.7em;
  font-size: 1.05em;
}
.cos-label {
  color: var(--vp-c-text-2);
  margin-right: 0.5em;
}
</style> 