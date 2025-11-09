<template>
    <div id="turnstile-widget"></div>
</template>

<script setup>
import { onMounted } from 'vue'

const props = defineProps({
    onSuccess: {
        type: Function,
        required: true
    },
    sitekey: {
        type: String,
        required: true
    }
})

function script() {
    const script = document.createElement('script')
    script.src = 'https://challenges.cloudflare.com/turnstile/v0/api.js?onload=onLoad'
    script.async = true
    script.defer = true
    document.head.appendChild(script)
}

window.onLoad = async function () {
    if (!window.turnstile) {
        console.error('Turnstile script not loaded')
        return
    }

    const widgetId = window.turnstile.render('#turnstile-widget', {
        sitekey: props.sitekey,
        callback: props.onSuccess,
        execution: 'execute',
        action: 'submit'
    })

    window.turnstile.execute(widgetId)
}

onMounted(script)
</script>