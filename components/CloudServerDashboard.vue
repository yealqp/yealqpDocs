<script setup lang="ts">
import { withDefaults, defineProps, computed, ref, onMounted, watch } from 'vue'
const props = withDefaults(
  defineProps<{
    IDCname: string,
    config: string,
    time: string,
    intro?: string,
    size?: string,
    link?: string,
    api?: string
  }>(),
  { size: '1' }
)

const cardClass = computed(() => {
  if (props.size === '3') return 'cloud-card-1-3'
  if (props.size === '2') return 'cloud-card-1-2'
  return 'cloud-card-1-1'
})

function openLink(e: Event) {
  e.stopPropagation()
  if (props.link) window.open(props.link, '_blank')
}

const apiData = ref<string>('')
const cpuPercent = ref<string>('')
const memPercent = ref<string>('')

function parseCpuMem(data: any) {
  // 只取第一个（后端只返回一个）
  if (!Array.isArray(data) || !data[0] || !data[0].success) {
    cpuPercent.value = ''
    memPercent.value = ''
    return
  }
  // 解析cpu
  const cpuRaw = data[0].cpu || ''
  // 只取第二个%Cpu(s):...行
  const cpuLines = cpuRaw.split('\n').filter(line => line.includes('Cpu(s)'))
  let cpuLine = ''
  if (cpuLines.length >= 2) {
    cpuLine = cpuLines[1]
  } else if (cpuLines.length === 1) {
    cpuLine = cpuLines[0]
  }
  // 例如: %Cpu(s):  1.4 us,  0.5 sy,  0.0 ni, 97.6 id, ...
  const cpuMatch = cpuLine.match(/([\d.]+)\s*id/)
  if (cpuMatch) {
    const idle = parseFloat(cpuMatch[1])
    cpuPercent.value = (100 - idle).toFixed(1) + '%'
  } else {
    cpuPercent.value = ''
  }
  // 解析mem
  const memRaw = data[0].mem || ''
  const memMatch = memRaw.match(/Mem:\s+(\d+)\s+(\d+)/)
  if (memMatch) {
    const total = parseFloat(memMatch[1])
    const used = parseFloat(memMatch[2])
    if (total > 0) {
      memPercent.value = (used / total * 100).toFixed(1) + '%'
    } else {
      memPercent.value = ''
    }
  } else {
    memPercent.value = ''
  }
}

async function fetchApiData() {
  if (!props.api) return
  try {
    const apihost = "http://localhost:5000"
    const url = `${apihost}/api/ssh_status?api=${encodeURIComponent(props.api)}`
    const res = await fetch(url)
    const data = await res.json()
    if (data && data.success && data.data) {
      apiData.value = JSON.stringify(data.data)
      parseCpuMem(data.data)
    } else {
      apiData.value = '获取数据失败'
      cpuPercent.value = ''
      memPercent.value = ''
    }
  } catch (e) {
    apiData.value = '接口异常'
    cpuPercent.value = ''
    memPercent.value = ''
  }
}

onMounted(fetchApiData)
watch(() => props.api, fetchApiData)
</script>

<template>
  <div :class="['ncard', 'cloud-card', cardClass]">
    <div class="ncardBody">
      <div class="card-title text vendor-row">
        <span>{{ props.IDCname }}</span>
        <Badge v-if="props.link" type="tip" style="margin-left:10px;cursor:pointer;" @click="openLink">控制台</Badge>
      </div>
      <div class="card-text text">
        <div class="server-config">配置：{{ props.config }}</div>
        <div>到期：{{ props.time }}</div>
        <div>{{ props.intro }}</div>
        <div v-if="props.api" style="margin-top:8px;font-size:0.95em;color:var(--vp-c-text-1);word-break:break-all;">
          <span v-if="!cpuPercent && !memPercent && !apiData">正在获取数据...</span>
          <span v-else-if="cpuPercent || memPercent">CPU: {{ cpuPercent }} &nbsp; MEM: {{ memPercent }}</span>
          <span v-else>{{ apiData }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ncard {
  border: 1px solid var(--vp-c-border);
  /* box-shadow: 2px 2px 8px 1px rgba(0, 0, 0, .15); */
  margin-bottom: 2em;
  border-radius: 8px;
  transition: all 200ms ease;
  background: transparent;
  box-sizing: border-box;
  color: var(--vp-c-text-1);
  text-decoration: none;
  display: block;
}
.ncard:hover {
  border: 1px solid var(--vp-c-brand);
  /* box-shadow: 2px 2px 1px 0 rgba(0, 0, 0, .1); */
}
.ncardBody {
  padding: 1.8em;
}
.card-title {
  font-size: 1.5em;
  margin-bottom: .8em;
  color: var(--vp-c-text-1);
}
.vendor-row {
  display: flex;
  align-items: center;
  gap: 0.5em;
}
.card-text {
  font-size: 0.9em;
  color: var(--vp-c-text-2);
}
.server-config {
  font-weight: bold;
  font-size: 1.1em;
  margin-bottom: 0.5em;
  color: var(--vp-c-text-1);
}
.server-expire {
  font-size: 1em;
  color: var(--vp-c-text-2);
}
.cloud-card-1-1 {
  width: 100%;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}
.cloud-card-1-2 {
  width: 48%;
  min-width: 260px;
  max-width: 420px;
  display: inline-block;
  margin-left: 1%;
  margin-right: 1%;
}
.cloud-card-1-3 {
  width: 31.5%;
  min-width: 220px;
  max-width: 340px;
  display: inline-block;
  margin-left: 1%;
  margin-right: 1%;
}
@media (max-width: 900px) {
  .cloud-card-1-2, .cloud-card-1-3 {
    width: 98%;
    max-width: 600px;
    min-width: 200px;
    margin-left: auto;
    margin-right: auto;
    display: block;
  }
}
.cloud-link {
  cursor: pointer;
}
</style> 