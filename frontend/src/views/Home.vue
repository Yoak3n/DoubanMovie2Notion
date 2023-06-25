<script setup lang="ts">
import { NForm, NFormItem, NInput, NButton, NSelect, NDrawer, NList, NListItem, NImage, NThing, NTag, NSpace, NDrawerContent, NEmpty, SelectOption, useNotification } from 'naive-ui'
import { LoadOptions, AppRun, QueryMovie } from '../../wailsjs/go/main/App';
import { useRouter } from 'vue-router';
import { onMounted, ref, watch } from 'vue'
import type { Result } from "../types";
const $router = useRouter()
const $notification = useNotification()

let target = ref("")
let options = ref<SelectOption[]>([])
let optionValue = ref("")
let selectLoading = ref(false)
let submitLoading = ref(false)
let showQuery = ref(false)

onMounted(() => {
  loadOptions()
})

const loadOptions = () => {
  selectLoading.value = true
  // 读取文件配置缓冲
  let optionTemp: SelectOption[] = []
  LoadOptions().then((result) => {
    result.forEach((value) => {
      let reg = value.split('.json')[0]
      optionTemp.push({
        value: reg,
        label: value
      })
      options.value = optionTemp
      optionValue.value = options.value[0].value as string
    })
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
    AppRun(target.value, optionValue.value).then((result) => {
      if (result.status) {
        $notification.success({
          content: "电影《" + result.name + "》上传成功",
          duration: 2000,
          keepAliveOnHover: true
        })
      } else {
        $notification.error({
          content: result.name
        })
      }
      submitLoading.value = false
    })
  }
}


let queryResult = ref<Result[]>([])


const idQuery = (): void => {
  console.log(1111);
  QueryMovie(target.value).then((results) => {
    console.log(results);
    queryResult.value = results
  })
}


</script>

<template>
  <div class="home-wrapper">
    <!-- 表单部分 -->
    <n-form class="form-wrapper">
      <n-form-item label="配置文件" size="large">
        <n-select :options="options" :loading="selectLoading" v-model:value="optionValue" @click="loadOptions">
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
        <n-button type="success" @click="submit" class="button" size="large" :loading="submitLoading"
          :disabled="submitLoading">
          提交
        </n-button>
      </n-form-item>
    </n-form>
    <!-- 抽屉组件部分 -->
    <n-drawer v-model:show="showQuery" placement="top" :show-mask="false" height="50%" resizable>
      <n-drawer-content closable>
        <div class="query-wrapper" id="image-scroll-container" >
          <n-list v-for="item in queryResult" key="item.id" hoverable clickable @click="chooseMovie(item.id)" >
            <div>
              <n-list-item>
                <n-thing :title="item.title" :title-extra="item.sub_title">
                  <template #avatar>
                  <n-image lazy :alt="item.sub_title" height="80" width="80" :src="item.img" :intersection-observer-options="{
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
  .query-wrapper{
    overflow: auto;
  }

}
</style>