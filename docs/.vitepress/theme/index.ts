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
import Status from "../../../components/ServerStatus.vue"
import SkinDashboard from "../../../components/BlessingSkinDashboard.vue"
import CloudServer from "../../../components/CloudServerDashboard.vue"
import CosInfo from "../../../components/CosBucketInfo.vue"

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
    enhanceApp({ app }) {
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
            .component('Status', Status)
            .component('SkinDashboard', SkinDashboard)
            .component('CloudServer', CloudServer)
            .component('CosInfo', CosInfo)
    }
}
export default Theme