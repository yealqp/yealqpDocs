<script setup>
import { ref, onMounted } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faServer, faDatabase, faCode, faPlug, faCircle } from '@fortawesome/free-solid-svg-icons'

const skinData = ref(null)
const loading = ref(true)
const error = ref(null)

// 站点信息
const siteInfo = ref({
  version: '',
  environment: '',
  debug: false,
  commit: '',
  laravelVersion: '',
  phpVersion: '',
  webServer: '',
  os: '',
  dbType: '',
  dbHost: '',
  dbPort: '',
  dbUser: '',
  dbName: '',
  plugins: []
})

const fetchSkinStatus = async () => {
  try {
    loading.value = true
    error.value = null

    // 调用本地后端 API
    const response = await fetch('http://localhost:5000/api/status')

    if (!response.ok) {
      throw new Error('获取状态失败')
    }

    const result = await response.json()

    if (result.success) {
      skinData.value = result.data

      // 解析数据到组件状态
      siteInfo.value = {
        version: result.data.version || '',
        environment: result.data.environment || '',
        debug: result.data.debug || false,
        commit: result.data.commit || '',
        laravelVersion: result.data.laravel_version || '',
        phpVersion: result.data.php_version || '',
        webServer: result.data.web_server || '',
        os: result.data.os || '',
        dbType: result.data.db_type || '',
        dbHost: result.data.db_host || '',
        dbPort: result.data.db_port || '',
        dbUser: result.data.db_user || '',
        dbName: result.data.db_name || '',
        plugins: result.data.plugins || []
      }
    } else {
      throw new Error(result.message || '获取数据失败')
    }
  } catch (err) {
    error.value = '获取站点状态失败'
    console.error('获取站点状态失败:', err)
  } finally {
    loading.value = false
  }
}

// 获取状态颜色
const getStatusColor = (status) => {
  return status ? '#10b981' : '#ef4444'
}

// 获取状态文本
const getStatusText = (status) => {
  return status ? '正常' : '异常'
}

onMounted(() => {
  fetchSkinStatus()
  // 每60秒刷新一次状态
  setInterval(fetchSkinStatus, 60000)
})
</script>

<template>
  <div class="ncard">
    <div class="ncardBody">
      <div class="card-title text">
        <FontAwesomeIcon :icon="faServer" />&nbsp;&nbsp;Blessing Skin 运行状态
        <div>
          <Badge type="tip" @click="fetchSkinStatus">
            刷新
          </Badge>
        </div>
      </div>

      <div v-if="loading" class="card-text text">
        <p>正在获取站点状态...</p>
      </div>

      <div v-else-if="error" class="card-text text">
        <p class="error-text">{{ error }}</p>
        <button class="refresh-button" @click="fetchSkinStatus">
          重新获取
        </button>
      </div>

      <div v-else class="card-text text">
        <!-- Blessing Skin 信息 -->
        <div class="section-title">
          <FontAwesomeIcon :icon="faCode" />&nbsp;&nbsp;Blessing Skin
        </div>

        <div class="status-row">
          <span class="status-label">版本:</span>
          <span class="status-value">{{ siteInfo.version }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">环境:</span>
          <span class="status-value">{{ siteInfo.environment }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Laravel 版本:</span>
          <span class="status-value">{{ siteInfo.laravelVersion }}</span>
        </div>

        <!-- 服务器信息 -->
        <div class="section-title">
          <FontAwesomeIcon :icon="faServer" />&nbsp;&nbsp;服务器
        </div>

        <div class="status-row">
          <span class="status-label">PHP 版本:</span>
          <span class="status-value">{{ siteInfo.phpVersion }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">Web 服务器:</span>
          <span class="status-value">{{ siteInfo.webServer }}</span>
        </div>

        <div class="status-row">
          <span class="status-label">操作系统:</span>
          <span class="status-value">{{ siteInfo.os }}</span>
        </div>

        <!-- 数据库信息 -->
        <div class="section-title">
          <FontAwesomeIcon :icon="faDatabase" />&nbsp;&nbsp;数据库
        </div>

        <div class="status-row">
          <span class="status-label">类型:</span>
          <span class="status-value">{{ siteInfo.dbType }}</span>
        </div>

        <!-- 插件信息 -->
        <div class="section-title">
          <FontAwesomeIcon :icon="faPlug" />&nbsp;&nbsp;已开启的插件 ({{ siteInfo.plugins.length }})
        </div>

        <div v-for="plugin in siteInfo.plugins" :key="plugin.name" class="status-row">
          <span class="status-label">{{ plugin.name }}:</span>
          <span class="status-value">{{ plugin.version }}</span>
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

.section-title {
  font-weight: 600;
  color: var(--vp-c-text-1);
  margin: 1.5em 0 0.8em 0;
  padding-bottom: 0.3em;
  border-bottom: 2px solid var(--vp-c-brand);
  display: flex;
  align-items: center;
}

.section-title:first-child {
  margin-top: 0;
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
  display: flex;
  align-items: center;
  gap: 0.3em;
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