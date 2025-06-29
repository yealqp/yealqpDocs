<script setup>
import { ref, onMounted } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faServer, faUsers, faCircle } from '@fortawesome/free-solid-svg-icons'

const serverData = ref(null)
const loading = ref(true)
const error = ref(null)

// 服务器信息
const serverInfo = ref({
  online: false,
  players: { online: 0, max: 0 },
  version: '',
  motd: '',
  software: ''
})

const fetchServerStatus = async () => {
  try {
    loading.value = true
    error.value = null

    const response = await fetch('https://api.mcsrvstat.us/3/main.yealqp.xyz')
    const data = await response.json()

    serverData.value = data
    serverInfo.value = {
      online: data.online || false,
      players: data.players || { online: 0, max: 0 },
      version: data.version || '未知',
      motd: data.motd?.clean?.[0] || '未知',
      hostname: 'main.yealqp.xyz',
      software: data.software || '未知'
    }
  } catch (err) {
    error.value = '获取服务器状态失败'
    console.error('获取服务器状态失败:', err)
  } finally {
    loading.value = false
  }
}

// 格式化 ping 时间
const formatPing = (ping) => {
  if (ping === null || ping === undefined) return '未知'
  return `${ping}ms`
}

// 获取状态颜色
const getStatusColor = (online) => {
  return online ? '#10b981' : '#ef4444'
}

// 获取状态文本
const getStatusText = (online) => {
  return online ? '在线' : '离线'
}

onMounted(() => {
  fetchServerStatus()
  // 每30秒刷新一次状态
  setInterval(fetchServerStatus, 30000)
})
</script>

<template>
  <div class="ncard">
    <div class="ncardBody">
      <div class="card-title text">
        <FontAwesomeIcon :icon="faServer" />&nbsp;&nbsp;MC服务器状态
        <div>
          <Badge type="tip" @click="fetchServerStatus">
            刷新
          </Badge>
        </div>
      </div>

      <div v-if="loading" class="card-text text">
        <p>正在获取服务器状态...</p>
      </div>

      <div v-else-if="error" class="card-text text">
        <p class="error-text">{{ error }}</p>
        <button class="refresh-button" @click="fetchServerStatus">
          重新获取
        </button>
      </div>

      <div v-else class="card-text text">
        <div class="status-row">
          <span class="status-label">状态:</span>
          <span class="status-value">
            <FontAwesomeIcon :icon="faCircle" :style="{ color: getStatusColor(serverInfo.online) }" />
            {{ getStatusText(serverInfo.online) }}
          </span>
        </div>

        <div class="status-row">
          <span class="status-label">地址:</span>
          <span class="status-value">{{ serverInfo.hostname }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">版本:</span>
          <span class="status-value">{{ serverInfo.version }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">核心:</span>
          <span class="status-value">{{ serverInfo.software }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">
            <FontAwesomeIcon :icon="faUsers" />&nbsp;&nbsp;玩家:
          </span>
          <span class="status-value">
            {{ serverInfo.players.online }}/{{ serverInfo.players.max }}
          </span>
        </div>

        <div class="status-row">
          <span class="status-label">MOTD:</span>
          <span class="status-value motd-text">{{ serverInfo.motd }}</span>
        </div>

      </div>
    </div>
  </div>
</template>

<style scoped>
.ncard {
  border: 1px solid var(--vp-c-border);
  margin: 2em 0;
  border-radius: 8px;
}

.ncardBody {
  padding: 1.8em;
}

.card-title {
  font-size: 1.2em;
  margin-bottom: .8em;
  display: flex;
  align-items: center;
}

.card-text {
  font-size: 0.9em;
}

.status-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5em;
  padding: 0.3em 0;
  border-bottom: 1px solid var(--vp-c-border);
}

.status-row:last-of-type {
  border-bottom: none;
}

.status-label {
  font-weight: 500;
  color: var(--vp-c-text-2);
  display: flex;
  align-items: center;
}

.status-value {
  color: var(--vp-c-text-1);
  font-weight: 400;
}

.motd-text {
  max-width: 200px;
  word-wrap: break-word;
  text-align: right;
}

.error-text {
  color: #ef4444;
  margin-bottom: 1em;
}

.action-buttons {
  margin-top: 1.5em;
  display: flex;
  gap: 1em;
  flex-wrap: wrap;
}

.refresh-button,
.connect-button {
  cursor: pointer;
  background-color: var(--vp-c-bg);
  box-shadow: none;
  color: var(--vp-c-text-1);
  border: 1px solid var(--vp-c-text-1);
  border-radius: 0.25rem;
  text-align: center;
  padding: .6em 2em;
  display: inline-block;
  font-size: 1em;
  line-height: 1.5;
  vertical-align: middle;
  transition: all 200ms ease;
  text-decoration: none;
}

.refresh-button:hover,
.connect-button:hover {
  border: 1px solid var(--vp-c-brand);
  color: var(--vp-c-brand);
}

.connect-button {
  background-color: var(--vp-c-brand);
  color: white;
  border-color: var(--vp-c-brand);
}

.connect-button:hover {
  background-color: var(--vp-c-brand-dark);
  border-color: var(--vp-c-brand-dark);
  color: white;
}

@media (max-width: 768px) {
  .status-row {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.3em;
  }

  .status-value {
    align-self: flex-end;
  }

  .action-buttons {
    flex-direction: column;
  }
}
</style>