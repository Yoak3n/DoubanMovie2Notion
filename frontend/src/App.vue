<script lang="ts" setup>
import {NPageHeader,NButton,NIcon,NSpace,NNotificationProvider} from 'naive-ui'
import { LogInOutline as LogInIcon } from '@vicons/ionicons5'
import {useRouter} from 'vue-router'
import { Quit } from '../wailsjs/runtime/runtime';

const $router = useRouter()

const handleBack  = ()=>{
  $router.push("/")
}

const handleQuit = () =>{
  Quit()
}

</script>

<template>
  <div class="app-wrapper">
    <n-page-header class="welcome" @back="handleBack" style="--wails-draggable:drag">
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
    padding: 2rem 0 ;
  }
  margin: 0 2rem  ;
}
</style>
