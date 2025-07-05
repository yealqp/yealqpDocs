import { defineConfig } from "vitepress";

let GTAG_ID = "G-TBCY5W7YVR";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Yea帮助文档",
  description: "Yea帮助文档",
  head: [
    ["link", { rel: "icon", href: "/favicon.ico" }],
    [
      "script",
      {
        async: "",
        src: `https://www.googletagmanager.com/gtag/js?id=${GTAG_ID}`,
      },
    ],
    [
      "script",
      {},
      `window.dataLayer = window.dataLayer || [];
      function gtag(){dataLayer.push(arguments);}
      gtag('js', new Date());
      gtag('config', '${GTAG_ID}');`,
    ],
  ],

  lastUpdated: true,
  sitemap: {
    hostname: "https://docs.yealqp.fun",
  },

  lang: "zh",
  appearance: true,

  markdown: {
    theme: { light: "catppuccin-latte", dark: "one-dark-pro" },
    image: { lazyLoading: true },
  },

  cleanUrls: true,

  locales: {
    root: {
      label: "中文",
      lang: "zh",
    },
  },

  themeConfig: {
    logo: "/favicon.ico",
    nav: [
      { text: "BYS皮肤站", link: "https://skin.yealqp.fun/" },
    ],

    docFooter: {
      prev: "上一页",
      next: "下一页",
    },
    langMenuLabel: "多语言",
    returnToTopLabel: "回到顶部",
    sidebarMenuLabel: "目录",
    darkModeSwitchLabel: "主题",
    lightModeSwitchTitle: "切换到浅色模式",
    darkModeSwitchTitle: "切换到深色模式",
    externalLinkIcon: true,
    lastUpdated: {
      text: "上次更新于",
    },
    outline: {
      label: "在此页面上的内容",
    },
    editLink: {
      pattern: "https://github.com/yealqp/yealqpDocs",
      text: "帮助我们完善这个页面",
    },
    sidebar: {
      "/": [
        {
          text: "介绍",
          link: "/",
          items: [
            {
              items: [
                { text: "赞助我们", link: "/pay" },
                { text: "服务器状态", link: "/status" },
              ],
            },
            {
              text: "BYS相关",
              items: [
                { text: "注册指南", link: "/faq/reg" },
                {
                  text: "政策条款",
                  link: "/policies/",
                  items: [
                    { text: "用户服务条款", link: "/policies/tos" },
                    { text: "隐私声明", link: "/policies/privacy" },
                  ],
                },
                {
                  text: "疑难杂症",
                  items: [
                    { text: "遇到问题了咋办", link: "/problems" },
                    {
                      text: "常见问题解答",
                      link: "/faq/",
                      items: [
                        { text: "站点使用相关", link: "/faq/site" },
                        { text: "游戏内加载相关", link: "/faq/in-game" },
                      ],
                    },
                  ],
                },
              ],
            },
            {
              text: "传奇人物榜",
              link: "/heros",
              items: [
                { text: "《装比哥》", link: "/heros/yuxudan" },
                { text: "《嘴硬哥》", link: "/heros/hanliang" },
                { text: "《说话姐》", link: "/heros/liusheng" },
                { text: "《应当哥》", link: "/heros/yasai" },
              ],
            },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/yealqp/yealqpDocs" },
    ],
  },
  vite: {
    optimizeDeps: {
      exclude: ["@nolebase/vitepress-plugin-enhanced-readabilities/client"],
    },
    ssr: {
      noExternal: [
        // If there are other packages that need to be processed by Vite, you can add them here.
        "@nolebase/vitepress-plugin-enhanced-readabilities",
        "@nolebase/vitepress-plugin-highlight-targeted-heading",
      ],
    },
    server: {
    host: '0.0.0.0',
    allowedHosts: [''],
  },
  },
});
