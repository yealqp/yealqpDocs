// @ts-ignore: 2307
import { h } from 'vue'
import DefaultTheme from 'vitepress/theme'
import type { Theme as ThemeConfig } from 'vitepress'

import type { Options } from '@nolebase/vitepress-plugin-enhanced-readabilities'
import { InjectionKey } from '@nolebase/vitepress-plugin-enhanced-readabilities'


import {
    NolebaseEnhancedReadabilitiesMenu,
    NolebaseEnhancedReadabilitiesScreenMenu,
} from '@nolebase/vitepress-plugin-enhanced-readabilities'
import '@nolebase/vitepress-plugin-enhanced-readabilities/client/style.css'

import '@nolebase/vitepress-plugin-enhanced-mark/client/style.css'

import {
    NolebaseHighlightTargetedHeading,
} from '@nolebase/vitepress-plugin-highlight-targeted-heading'
import '@nolebase/vitepress-plugin-highlight-targeted-heading/client/style.css'

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import BSButton from '../../../components/BSButton.vue'
import BSSection from '../../../components/BSSection.vue'
import NCard from '../../../components/NCard.vue'
import Helpme from "../../../components/Helpme.vue"
import ICPFooter from "../../../components/ICPFooter.vue"


export const Theme: ThemeConfig = {
    extends: DefaultTheme,
    Layout: () => {
        return h(DefaultTheme.Layout, null, {
            // A enhanced readabilities menu for wider screens
            'nav-bar-content-after': () => h(NolebaseEnhancedReadabilitiesMenu),
            // A enhanced readabilities menu for narrower screens (usually smaller than iPad Mini)
            'nav-screen-content-after': () => h(NolebaseEnhancedReadabilitiesScreenMenu),
            // vitepress-plugin-highlight-targeted-heading
            'layout-top': () => [
                h(NolebaseHighlightTargetedHeading),
            ],
            // ICP备案信息页脚
            'doc-after': () => h(ICPFooter),
        })
    },
    enhanceApp({ app, router }) {
        app.provide(InjectionKey, {
            layoutSwitch: {
                defaultMode: 1
            },
            spotlight: {
                defaultToggle: true,
            }
        } as Options)
            .component('FA', FontAwesomeIcon)
            .component('BSButton', BSButton)
            .component('BSSection', BSSection)
            .component('NCard', NCard)
            .component("Helpme", Helpme)
            .component('ICPFooter', ICPFooter)
        
        // 优化路由钩子，减少卡顿，并添加顶部加载进度条
        if (router && typeof window !== 'undefined') {
            let isNeZhaPage = false;
            
            router.onBeforeRouteChange = (to) => {
                const wasNeZhaPage = isNeZhaPage;
                isNeZhaPage = to.endsWith('/nezha');

                // 立即更新页面状态，避免延迟
                if (isNeZhaPage) {
                    document.documentElement.classList.add('nezha-page', 'nezha-loading');
                } else {
                    document.documentElement.classList.remove('nezha-page', 'nezha-loading');
                    if (wasNeZhaPage) {
                        document.documentElement.classList.add('leaving-nezha-page');
                        // 使用requestAnimationFrame优化性能
                        requestAnimationFrame(() => {
                            document.documentElement.classList.remove('leaving-nezha-page');
                        });
                    }
                }
            }
            
            // 简化页面加载完成后的处理
            router.onAfterRouteChange = (to) => {
                // 使用更快的滚动方式
                if (typeof window !== 'undefined') {
                    window.scrollTo(0, 0);
                }
            }
        }
    },
    setup() {
        // 确保在客户端环境下执行
        if (typeof window !== 'undefined') {
            // 检查当前URL是否是哪吒页面
            if (window.location.pathname.endsWith('/nezha')) {
                document.documentElement.classList.add('nezha-page');
            }
            
            // 添加简化的页面样式
            const style = document.createElement('style');
            style.textContent = `
                /* 哪吒页面加载中状态 - 简化版本 */
                .nezha-loading .VPDoc::before {
                    content: '';
                    position: fixed;
                    top: var(--vp-nav-height);
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: var(--vp-c-bg);
                    z-index: 100;
                }
                
                .nezha-loading .VPDoc::after {
                    content: '加载中...';
                    position: fixed;
                    top: calc(50% + var(--vp-nav-height)/2);
                    left: 50%;
                    transform: translate(-50%, -50%);
                    color: var(--vp-c-brand);
                    font-size: 18px;
                    font-weight: bold;
                    z-index: 101;
                }
            `;
            document.head.appendChild(style);
        }
    }
}

export default Theme