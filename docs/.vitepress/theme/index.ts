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
import MinecraftStatus from "../../../components/MinecraftStatus.vue"
import SkinDashboard from "../../../components/SkinDashboard.vue"
import CosInfo from "../../../components/CosBucketInfo.vue"
import NeZhaPanel from "../../../components/NeZhaPanel.vue"

// 导入自定义CSS
import './custom.css'


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
            .component('MinecraftStatus', MinecraftStatus)
            .component('SkinDashboard', SkinDashboard)
            .component('CosInfo', CosInfo)
            .component('NeZhaPanel', NeZhaPanel)
        
        // 添加路由钩子，处理哪吒页面的特殊布局
        if (router) {
            // 存储上一个路由状态，用于判断是从哪吒页面切换回来
            let wasOnNeZhaPage = false;
            
            router.onBeforeRouteChange = (to) => {
                // 在页面切换前添加过渡类
                document.documentElement.classList.add('page-transitioning');
                
                // 检查是否是从哪吒页面切换到其他页面
                const isLeavingNeZhaPage = document.documentElement.classList.contains('nezha-page') && !to.endsWith('/nezha');
                
                if (isLeavingNeZhaPage) {
                    // 从哪吒页面切换出去时，添加特殊的过渡类
                    document.documentElement.classList.add('leaving-nezha-page');
                    wasOnNeZhaPage = true;
                }
                
                // 如果正在进入哪吒页面，立即添加加载中状态
                if (to.endsWith('/nezha')) {
                    document.documentElement.classList.add('nezha-loading');
                }
                
                // 延迟执行以允许过渡效果完成
                setTimeout(() => {
                    if (to.endsWith('/nezha')) {
                        // 为哪吒页面添加特殊类名
                        document.documentElement.classList.add('nezha-page');
                    } else {
                        document.documentElement.classList.remove('nezha-page');
                        document.documentElement.classList.remove('nezha-loading');
                    }
                    
                    // 移除过渡类，触发新状态的过渡
                    setTimeout(() => {
                        document.documentElement.classList.remove('page-transitioning');
                        
                        // 如果是从哪吒页面切换回来，延迟一点移除特殊类，确保过渡效果完成
                        if (wasOnNeZhaPage && !to.endsWith('/nezha')) {
                            setTimeout(() => {
                                document.documentElement.classList.remove('leaving-nezha-page');
                                wasOnNeZhaPage = false;
                            }, 300);
                        }
                    }, 50);
                }, 50);
            }
            
            // 在页面加载完成后执行
            router.onAfterRouteChange = (to) => {
                // 确保页面平滑滚动到顶部
                window.scrollTo({
                    top: 0,
                    behavior: 'smooth'
                });
                
                // 如果是切换到根路径，确保侧边栏和内容区域平滑显示
                if (to === '/' || to === '') {
                    document.documentElement.classList.add('entering-home-page');
                    setTimeout(() => {
                        document.documentElement.classList.remove('entering-home-page');
                    }, 500);
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
                document.documentElement.classList.add('nezha-loading');
            }
            
            // 添加页面过渡样式
            const style = document.createElement('style');
            style.textContent = `
                .page-transitioning .VPContent,
                .page-transitioning .VPSidebar,
                .page-transitioning .VPDoc {
                    opacity: 0.6;
                    transition: opacity 0.3s ease;
                }
                
                .leaving-nezha-page .VPSidebar,
                .leaving-nezha-page .VPDocAside {
                    opacity: 0;
                    transform: translateX(-20px);
                    transition: opacity 0.5s ease, transform 0.5s ease;
                }
                
                .leaving-nezha-page .VPContent {
                    opacity: 0.8;
                    transition: opacity 0.5s ease, padding-left 0.5s ease;
                }
                
                .entering-home-page .VPSidebar,
                .entering-home-page .VPDocAside {
                    animation: fadeSlideIn 0.5s ease forwards;
                }
                
                .entering-home-page .VPContent {
                    animation: contentSlideIn 0.5s ease forwards;
                }
                
                /* 哪吒页面加载中状态 */
                .nezha-loading .VPDoc::before {
                    content: '';
                    position: fixed;
                    top: var(--vp-nav-height);
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: var(--vp-c-bg);
                    z-index: 100;
                    display: flex;
                    align-items: center;
                    justify-content: center;
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
                
                @keyframes fadeSlideIn {
                    from {
                        opacity: 0;
                        transform: translateX(-20px);
                    }
                    to {
                        opacity: 1;
                        transform: translateX(0);
                    }
                }
                
                @keyframes contentSlideIn {
                    from {
                        opacity: 0.8;
                        padding-left: 0;
                    }
                    to {
                        opacity: 1;
                        padding-left: var(--vp-sidebar-width);
                    }
                }
            `;
            document.head.appendChild(style);
        }
    }
}

export default Theme