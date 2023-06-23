<script setup lang="ts">
import {NForm,NFormItem,NInput,NButton,NSelect,SelectOption,useNotification}from 'naive-ui'
import { LoadOptions, AppRun } from '../../wailsjs/go/main/App';
import  {useRouter} from 'vue-router';
import {onMounted, ref}from 'vue'

const $router = useRouter()
const $notification = useNotification()

let target = ref("")
let options =ref<SelectOption[]>([])
let optionValue = ref("")
let selectLoading = ref(false)
let submitLoading = ref(false)
onMounted(()=>{
  loadOptions()
})

const loadOptions = ()=>{
  selectLoading.value = true
  // 读取文件配置缓冲区
  let optionTemp:SelectOption[] = []
  LoadOptions().then((result)=>{
    result.forEach((value)=>{
      let reg = value.split('.json')[0]
      optionTemp.push({
        value:reg,
        label:value
      })
      options.value = optionTemp
      optionValue.value = options.value[0].value as string
    })
    selectLoading.value =false
  })
}

const submit =()=>{
  let command:string = optionValue.value
  // 如果选中配置文件
  if (command == "创建新配置" || command == ""){
    $router.push("/config")
    $notification.info({
      content:"请创建配置",
      duration: 2000,
      closable: false,
      keepAliveOnHover: true
    })
  }else{
    submitLoading.value =true
    AppRun(target.value,optionValue.value).then((result)=>{
      if(result.status){
        $notification.success({
          content:"电影《"+result.name+"》上传成功",
          duration: 2000,
          keepAliveOnHover: true
        })
      }else{
        $notification.error({
          content:result.name
        })
      }
      submitLoading.value = false
    })
  }
}


</script>

<template>
  <div class="home-wrapper">
    <n-form class="form-wrapper">
      <n-form-item label="配置文件" size="large">
        <n-select
            :options="options"
            :loading="selectLoading"
            v-model:value="optionValue"
            @click="loadOptions">
        </n-select>
      </n-form-item>
      <n-form-item label="豆瓣电影ID" size="large">
        <n-input 
          v-model:value="target" 
          @keydown.enter="submit"
          placeholder="请输入豆瓣电影ID或URL链接" />
      </n-form-item>
      <n-form-item :show-label="false" :show-feedback="false">
        <n-button 
          type="success" 
          @click="submit"
          class="button"
          size="large"
          :loading="submitLoading"
          :disabled="submitLoading">
          提交
        </n-button>
      </n-form-item>
    </n-form>

  </div>
</template>

<style scoped lang="scss">
.home-wrapper{
  .form-wrapper{
    display: flex;
    flex-grow: 1;
    color: black;
    flex-direction: column;
    margin: 0 8%;
    text-align: left;
    .button{
      width: 100%;
      margin: 0 auto;
    }
  }
  
}

</style>