---
id: intro
title: 介绍
sidebar_position: 1
---

<div class="home-page-content">

# 欢迎来到 Yea 帮助文档

这里是 Yea 帮助文档的主页。您可以在这里找到所有关于我们服务器以及Byd Yealqp Skin（本文简称"BYS"）相关的信息和指南。

推荐使用暗色模式查看本站。
## 快速导航

- [BYS相关](faq/reg) - 如何注册皮肤站并开始游戏
- [传奇人物](heros/yuxudan) - 查看传奇乐子人

<Helpme>如果您在使用过程中遇到任何问题，请随时联系我们的管理员或在社区中提问。</Helpme>

## 免责声明
> [!CAUTION] 本文档部分自定义组件以及部分BYS相关文档修改自Littleskin官方开源文档
> [<BSSection>点击前往</BSSection>](https://github.com/LittleSkinChina/manual-ng)

</div>

<script>
// 为根页面添加特殊类名以支持平滑过渡
if (typeof document !== 'undefined') {
  document.documentElement.classList.add('home-page-enter');
  setTimeout(() => {
    document.documentElement.classList.remove('home-page-enter');
  }, 1000);
}
</script>

<style>
.home-page-content {
  opacity: 0;
  animation: fadeIn 0.5s ease forwards;
  animation-delay: 0.1s;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
</style>
