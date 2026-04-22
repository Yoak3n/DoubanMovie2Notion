<script setup lang="ts">
import { NForm, NFormItem, NInput, NButton, NSelect, NDrawer, NList, NListItem, NImage, NThing, NTag, NSpace, NDrawerContent, NEmpty, SelectOption, NSwitch, useNotification } from 'naive-ui'
// import { LoadOptions, AppRun, QueryMovie } from '../../wailsjs/go/main/App';
import { LoadOptions, Crawl, PickOption, QueryMovie, AddToPath, RemoveFromPath, IsPathAdded, GetInstallDir } from '../api'
import { useRouter } from 'vue-router';
import { onMounted, ref } from 'vue'
import type { Result } from "../types";
const $router = useRouter()
const $notification = useNotification()

let target = ref("")
let options = ref<SelectOption[]>([])
let optionValue = ref("")
let selectLoading = ref(false)
let submitLoading = ref(false)
let showQuery = ref(false)
let addToPath = ref(false)
let installDir = ref("")
const showPathSetting = !import.meta.env.DEV

onMounted(() => {
  loadOptions()
  if (showPathSetting) {
    initPathStatus()
  }
})

const initPathStatus = async () => {
  try {
    installDir.value = await GetInstallDir()
    addToPath.value = await IsPathAdded(installDir.value)
  } catch (e) {
    console.error("Failed to get install dir or path status:", e)
  }
}

const togglePath = async (value: boolean) => {
  try {
    if (value) {
      await AddToPath(installDir.value)
      $notification.success({
        content: "已添加到PATH",
        duration: 2000,
        closable: false
      })
    } else {
      await RemoveFromPath(installDir.value)
      $notification.success({
        content: "已从PATH移除",
        duration: 2000,
        closable: false
      })
    }
  } catch (e) {
    $notification.error({
      content: "操作失败: " + e,
      duration: 3000,
      closable: false
    })
    addToPath.value = !value
  }
}

const loadOptions = () => {
  selectLoading.value = true
  // 读取文件配置缓冲
  let optionTemp: SelectOption[] = []
  LoadOptions().then((result) => {
    result[0].forEach((value) => {
      let reg = value.split('.json')[0]
      optionTemp.push({
        value: reg,
        label: value
      })
      options.value = optionTemp
    })
    optionValue.value = result[1]
    selectLoading.value = false
  })
  
}

const queryMovie = () => {
  idQuery()
  showQuery.value = true

}
const chooseMovie = (id: string) => {
  showQuery.value = false
  target.value = id
}

const submit = () => {
  let command: string = optionValue.value
  // 如果选中配置文件
  if (command == "创建新配置" || command == "") {
    $router.push("/config")
    $notification.info({
      content: "请创建配置",
      duration: 2000,
      closable: false,
      keepAliveOnHover: true
    })
  } else {
    submitLoading.value = true
    Crawl(target.value).then((result) => {
      if (result != "") {
        $notification.success({
          content: "电影《" + result + "》上传成功",
          duration: 2000,
          keepAliveOnHover: true
        })
      }
      submitLoading.value = false
    }).catch((e) => {
      $notification.error({
        content: e
      })
      submitLoading.value = false
    })
  }
}


let queryResult = ref<Result[]>([])


const idQuery = (): void => {
  QueryMovie(target.value).then((results) => {
    console.log(results);
    queryResult.value = results
  })
}


</script>

<template>
  <div class="home-wrapper">
    <div v-if="showPathSetting" class="path-setting">
      <n-switch v-model:value="addToPath" @update:value="togglePath" />
      <span class="path-setting-label">使用cli</span>
    </div>
    <!-- 表单部分 -->
    <n-form class="form-wrapper">
      <n-form-item label="配置文件" size="large">
        <n-select :options="options" :loading="selectLoading" v-model:value="optionValue" @click="loadOptions" @update:value="(val:string) => PickOption(val)">
        </n-select>
      </n-form-item>
      <n-form-item label="豆瓣电影ID" size="large">
        <n-input v-model:value="target" @keydown.enter="submit" placeholder="请输入豆瓣电影名或ID或URL链接" autofocus>
          <template #suffix>
            <n-button @click="queryMovie" :focusable="false" :bordered="false" size="small">搜索</n-button>
          </template>
        </n-input>
      </n-form-item>
      <n-form-item :show-label="false" :show-feedback="false">
        <n-button type="success" @click="submit" class="button" size="large" :loading="submitLoading">
          提交
        </n-button>
      </n-form-item>
    </n-form>
    <!-- 抽屉组件部分 -->
    <n-drawer v-model:show="showQuery" placement="top" :show-mask="false" height="50%" resizable>
      <n-drawer-content closable>
        <div class="query-wrapper" id="image-scroll-container">
          <n-list v-for="item in queryResult" key="item.id" hoverable clickable @click="chooseMovie(item.id)">
            <div>
              <n-list-item>
                <n-thing :title="item.title" :title-extra="item.sub_title">
                  <template #avatar>
                    <n-image lazy :alt="item.sub_title" height="80" width="80" :src="item.img"
                      :intersection-observer-options="{
                        root: '#image-scroll-container'
                      }">
                      <template #placeholder>
                        <div style="
                          width: 80px;
                          height: 80px;
                          display: flex;
                          align-items: center;
                          justify-content: center;
                          background-color: #0001;
                        ">
                          Loading...
                        </div>
                      </template>
                    </n-image>
                  </template>
                  <template #description>
                    <n-space size="small" style="margin-top: 4px">
                      <n-tag :bordered="false" size="small">
                        年份：{{ item.year }}
                      </n-tag>
                      <n-tag :bordered="false" type="success" size="small">
                        id:{{ item.id }}
                      </n-tag>
                    </n-space>

                  </template>

                </n-thing>

              </n-list-item>
            </div>

          </n-list>
          <n-empty v-if="queryResult == null || queryResult.length == 0" description="你什么也没找到">

          </n-empty>
        </div>
      </n-drawer-content>
    </n-drawer>

  </div>
</template>

<style scoped lang="scss">
.home-wrapper {
  .path-setting {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    min-height: 40px;
    margin: 0 8% 12px;
  }

  .path-setting-label {
    line-height: 1;
  }

  .form-wrapper {
    display: flex;
    flex-grow: 1;
    color: black;
    flex-direction: column;
    margin: 0 8%;
    text-align: left;

    .button {
      width: 100%;
      margin: 0 auto;
    }
  }

  .query-wrapper {
    overflow: auto;
  }
}
</style>
