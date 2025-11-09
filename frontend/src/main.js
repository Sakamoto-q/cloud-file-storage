import { createApp } from 'vue'
import router from './router'
import App from './App.vue'
import i18n from './i18n'

if (import.meta.env.MODE === 'development') {
    window.api = "http://localhost:9000/api/v1"
} else {
    window.api = "/api/v1"
}

const app = createApp(App)
app.use(i18n);
app.use(router)
app.mount('#app')