import {RouteRecordRaw} from "vue-router"

const routes : RouteRecordRaw[] = [
    {path:'/',component:()=>import("../views/Home.vue"),name:"Home"},
    {path:'/config',component:()=>import("../views/Config.vue"),name:"Config"}
]

export default routes


