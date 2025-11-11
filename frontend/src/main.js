import { createApp } from 'vue'
import router from './router'
import App from './App.vue'
import i18n from './i18n'

var API_BASE
if (import.meta.env.MODE === 'development') {
    API_BASE = "http://localhost:9000/api/v1"
} else {
    API_BASE = "/api/v1"
}

const app = createApp(App)
app.provide('API_BASE', API_BASE)
app.use(i18n);
app.use(router)
app.mount('#app')