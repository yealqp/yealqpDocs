<script setup>
import { faFileArrowDown } from '@fortawesome/free-solid-svg-icons'
</script>

# 这里有自定义组件的示例
> [!CAUTION] 本文档自定义组件的源码均来自Littleskin开源官方文档
> [<BSSection>点击前往</BSSection>](https://github.com/LittleSkinChina/manual-ng)
## 提示类卡片
> [!NOTE]
> 强调用户在快速浏览文档时也不应忽略的重要信息。

> [!TIP]
> 有助于用户更顺利达成目标的建议性信息。

> [!IMPORTANT]
> 对用户达成目标至关重要的信息。

> [!WARNING]
> 因为可能存在风险，所以需要用户立即关注的关键内容。

> [!CAUTION]
> 行为可能带来的负面影响。

## 装饰按钮

<BSSection>装饰按钮</BSSection>

[<BSSection>超链接示例</BSSection>](/example)

<BSButton>蓝色的装饰按钮</BSButton>

[<BSButton>蓝色的超链接示例</BSButton>](/example)
- 圆角
<Badge type="info" text="信息" />
<Badge type="tip" text="普通" />
<Badge type="warning" text="关键" />
<Badge type="danger" text="致命" />
- 引用图标
<BSButton ><FA :icon="faFileArrowDown" /> 下载此文件 </BSButton>

## 插表

| 符号名称 | 符号 |
| -------- | ---- |
| 下划线   | `_`  |
| 连字符   | `-`  |

## 下拉框
::: details 标题
内容
:::

## 大文本框

 <div align="center" style="padding: 2em; margin: 2em 0; border: 1px solid var(--vp-c-text-1); border-radius: 8px">
 <p align="left">Troubleshooting any problem without the error log is like driving with your eyes closed.</p>
 <p align="left" style="font-size: 1.15em">在没有错误日志的情况下诊断任何问题无异于闭眼开车。</p>
 <p align="right" style="font-size: 1.15em">-- Apache官方文档Getting Started 篇章</p>
 </div>

## Ncard 组件

<NCard title="这是Ncard组件" >
内容 可以嵌套 <BSSection><FA :icon="faUsers" /> 卡片</BSSection>
</NCard>

## 代码块

```markdown {2-3}
在代码块类型后使用{行号}实现行高亮
多行：例如 {5-8}、{3-10}、{10-17}
多个单行：例如 {4,7,9}
多行与单行：例如 {4,7-13,16,23-27,40}
```