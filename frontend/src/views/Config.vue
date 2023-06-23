<template>
    <div class="config-wrapper">
        <n-modal v-model:show="showModal" :mask-closable="false" class="modal">
            <n-card
            :bordered="false"
            style="height: 80%;"
            content-style="text-align:left"
            role="dialog">
            <!-- <template> -->
            <n-form class="form">
                <n-form-item :show-label="false" :show-feedback="false">
                    <n-input v-model:value="configName" placeholder="请输入配置文件名" size="large" @keydown.enter="enterConfigName"/>
                </n-form-item>
            </n-form>
            <!-- </template> -->
                
            <template #action>
                <n-button @click="enterConfigName" type="success" class="button">确认</n-button>
            </template>
            </n-card> 
        </n-modal>
        <n-form 
        class="form"
        :rules="rules" 
        :model="configuration"
        :show-require-mark="false">
            <n-form-item label="DatabaseID" path="databaseid">
                <n-input v-model:value="configuration.databaseid"/>
            </n-form-item>
            <n-form-item label="Token" path="token">
                <n-input v-model:value="configuration.token" @keydown.enter="setConfiguration"/>
            </n-form-item>
            <n-form-item :show-label="false">
                <n-button class="button" @click="setConfiguration" :loading="createLoading" :disabled="createLoading">创建</n-button>
            </n-form-item>
        </n-form>
    </div>
</template>

<script setup lang="ts">
import { NModal,NCard,NForm,NFormItem,NInput,NButton,useNotification } from 'naive-ui';
import {onMounted,ref} from 'vue'
import type{ Configuration} from '../types/index'
import { useRouter } from 'vue-router';
import { WriteConfig } from '../../wailsjs/go/main/App';




const $notification = useNotification()
let showModal = ref(false)
let createLoading = ref(false)
let configName = ref("")
const $router = useRouter()


onMounted(()=>{
    showModal.value = true
})

const enterConfigName = ()=>{
    if (configName.value != ""){
        showModal.value = false
    }
}

const  configuration =ref<Configuration>({databaseid:"",token:""})

const rules = {
    databaseid:{
        required: true,
        message: '请输入数据库id',
        trigger: ['blur','input'],
    },
    token:{
        required: true,
        message: '请输入机器人令牌',
        trigger: ['blur','input'],
    }
}
const setConfiguration = ()=>{
    createLoading.value = true
    WriteConfig(configName.value,configuration.value.databaseid,configuration.value.token).then((msg)=>{
        if(msg!=""){
            $notification.error({
                content:"出现错误请重试："+msg,
                duration:1000,
                closable:false
            })
        }else{
            $notification.success({
                content:"创建配置成功！",
                duration:1000,
                closable:false
            })
        }
        createLoading.value = false
        $router.push('/')
    })
}


</script>

<style scoped>
    .config-wrapper{
        .form{
            text-align: left;
            margin: 0 50px;
            .button{
                width: 100%;
            }
        }
    }


</style>