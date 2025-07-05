<script setup>
import { ref, onMounted } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faUsers, faGamepad, faFile, faHdd, faChartLine } from '@fortawesome/free-solid-svg-icons'

const dashboardData = ref(null)
const loading = ref(true)
const error = ref(null)

// 仪表盘信息
const dashboardInfo = ref({
  users_count: 0,
  players_count: 0,
  textures_count: 0,
  storage_size: '',
  site_name: '',
  version: '',
  locale: '',
  base_url: ''
})

const fetchDashboardData = async () => {
  try {
    loading.value = true
    error.value = null
    
    // 调用本地后端 API
    const response = await fetch('http://localhost:5000/api/dashboard')
    
    if (!response.ok) {
      throw new Error('获取仪表盘数据失败')
    }
    
    const result = await response.json()
    
    if (result.success) {
      dashboardData.value = result.data
      
      // 解析数据到组件状态
      dashboardInfo.value = {
        users_count: result.data.users_count || 0,
        players_count: result.data.players_count || 0,
        textures_count: result.data.textures_count || 0,
        storage_size: result.data.storage_size || '',
        site_name: result.data.site_name || '',
        version: result.data.version || '',
        locale: result.data.locale || '',
        base_url: result.data.base_url || ''
      }
    } else {
      throw new Error(result.message || '获取数据失败')
    }
  } catch (err) {
    error.value = '获取仪表盘数据失败'
    console.error('获取仪表盘数据失败:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchDashboardData()
  // 每60秒刷新一次数据
  setInterval(fetchDashboardData, 60000)
})
</script>

<template>
  <div class="ncard">
    <div class="ncardBody">
      <div class="card-title text">
        <FontAwesomeIcon :icon="faChartLine" />&nbsp;&nbsp;Byd Yealqp Skin 仪表盘
        <div>
          <Badge type="tip" @click="fetchDashboardData">
            刷新
          </Badge>
        </div>
      </div>
      
      <div v-if="loading" class="card-text text">
        <p>正在获取仪表盘数据...</p>
      </div>
      
      <div v-else-if="error" class="card-text text">
        <p class="error-text">{{ error }}</p>
        <button class="refresh-button" @click="fetchDashboardData">
          重新获取
        </button>
      </div>
      
      <div v-else class="card-text text">
        <!-- 站点信息 -->
        <div class="site-info">
          <h3>{{ "Byd Yealqp Skin" }}</h3>
          <p class="site-details">
            版本: {{ dashboardInfo.version }} | 
            语言: {{ dashboardInfo.locale }}
          </p>
        </div>
        
        <!-- 统计卡片 -->
        <div class="stats-grid">
          <div class="stat-card users">
            <div class="stat-icon">
              <FontAwesomeIcon :icon="faUsers" />
            </div>
            <div class="stat-content">
              <div class="stat-number">{{ dashboardInfo.users_count }}</div>
              <div class="stat-label">注册用户</div>
            </div>
          </div>
          
          <div class="stat-card players">
            <div class="stat-icon">
              <FontAwesomeIcon :icon="faGamepad" />
            </div>
            <div class="stat-content">
              <div class="stat-number">{{ dashboardInfo.players_count }}</div>
              <div class="stat-label">角色总数</div>
            </div>
          </div>
          
          <div class="stat-card textures">
            <div class="stat-icon">
              <FontAwesomeIcon :icon="faFile" />
            </div>
            <div class="stat-content">
              <div class="stat-number">{{ dashboardInfo.textures_count }}</div>
              <div class="stat-label">上传材质</div>
            </div>
          </div>
          
          <div class="stat-card storage">
            <div class="stat-icon">
              <FontAwesomeIcon :icon="faHdd" />
            </div>
            <div class="stat-content">
              <div class="stat-number">{{ dashboardInfo.storage_size }}</div>
              <div class="stat-label">占用空间</div>
            </div>
          </div>
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

.site-info {
  text-align: center;
  margin-bottom: 2em;
  padding-bottom: 1em;
  border-bottom: 2px solid var(--vp-c-brand);
}

.site-info h3 {
  margin: 0 0 0.5em 0;
  color: var(--vp-c-text-1);
  font-size: 1.5em;
}

.site-details {
  margin: 0;
  color: var(--vp-c-text-2);
  font-size: 0.9em;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1em;
  margin-bottom: 2em;
}

.stat-card {
  display: flex;
  align-items: center;
  padding: 1.5em;
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-border);
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stat-icon {
  font-size: 2em;
  margin-right: 1em;
  width: 60px;
  text-align: center;
}

.stat-content {
  flex: 1;
}

.stat-number {
  font-size: 1.8em;
  font-weight: bold;
  color: var(--vp-c-text-1);
  line-height: 1;
  margin-bottom: 0.2em;
}

.stat-label {
  font-size: 0.9em;
  color: var(--vp-c-text-2);
}

/* 统计卡片颜色主题 */
.stat-card.users .stat-icon {
  color: #17a2b8;
}

.stat-card.players .stat-icon {
  color: #28a745;
}

.stat-card.textures .stat-icon {
  color: #6f42c1;
}

.stat-card.storage .stat-icon {
  color: #ffc107;
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
  justify-content: center;
}

.refresh-button, .connect-button {
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

.refresh-button:hover, .connect-button:hover {
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
  .stats-grid {
    grid-template-columns: 1fr;
  }
  
  .stat-card {
    padding: 1em;
  }
  
  .stat-icon {
    font-size: 1.5em;
    width: 40px;
  }
  
  .stat-number {
    font-size: 1.5em;
  }
  
  .action-buttons {
    flex-direction: column;
  }
}
</style> 