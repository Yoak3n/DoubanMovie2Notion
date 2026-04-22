<script lang="ts" setup>
import {NPageHeader,NButton,NIcon,NSpace,NNotificationProvider} from 'naive-ui'
import { LogInOutline as LogInIcon } from '@vicons/ionicons5'
import {useRouter} from 'vue-router'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

const $router = useRouter()
const compactHeader = !import.meta.env.DEV

const handleBack  = ()=>{
  $router.push("/")
}

const handleQuit = () =>getCurrentWebviewWindow().close()

</script>

<template>
  <div class="app-wrapper">
    <n-page-header :class="['welcome', { 'welcome-compact': compactHeader }]" @back="handleBack" data-tauri-drag-region>
      <template #title>
      <a href="https://github.com/Yoak3n/DoubanMovie2Notion"
        style="text-decoration: none; color: inherit"
      >欢迎使用豆瓣电影2notion！</a>
    </template>
    <template #extra>
      <n-space>
        <n-button
        strong
        :focusable="false"
        quaternary
        :bordered="false" 
        round
        @click="handleQuit">
          <template #icon>
            <n-icon><log-in-icon></log-in-icon></n-icon>
          </template>
          退出
        </n-button>
      </n-space>
    </template>
    </n-page-header>
    <n-notification-provider>
      <router-view></router-view>
    </n-notification-provider>
     
    
  </div>

</template>

<style lang="scss">
.app-wrapper{
  .welcome{
    font-size: large;
    padding: 2rem 0;
  }

  .welcome.welcome-compact{
    padding: 1.25rem 0 0.5rem;
  }
  margin: 0 2rem  ;
}
</style>
