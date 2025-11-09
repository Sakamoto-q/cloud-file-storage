<template>
    <div v-if="visible" class="modal-overlay" @click.self="close">
        <div class="modal">
            <div class="modal-header">
                <div class="header-left">
                    <div class="file-icon">
                        <Upload :size="32" />
                    </div>
                    <div class="file-info-header">
                        <h2>{{ $t('file_upload') }}</h2>
                        <div class="file-meta" v-if="isUploading">{{ $t('upload_progress', { progress: uploadProgress }) }}</div>
                    </div>
                </div>
                <button class="close-btn" @click="close">
                    <X :size="22" />
                </button>
            </div>
            <div class="modal-preview">
                <div class="preview-content">
                    <div v-if="isUploading" class="upload-status-large">
                        <div class="upload-progress-large">
                            <div class="progress-bar-large" :style="{ width: uploadProgress + '%' }"></div>
                        </div>
                        <span class="progress-text-large">{{ uploadProgress }}%</span>
                    </div>
                    <button 
                        v-else
                        class="btn btn-primary"
                        @click="openFileInput"
                    >
                        <ArrowUp :size="16" />
                        <span>{{ $t('select_file') }}</span>
                    </button>
                    <input 
                        ref="fileInput"
                        type="file" 
                        style="display: none"
                        @change="handleFileSelect"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue'
import { Upload, X, ArrowUp } from 'lucide-vue-next'

const emit = defineEmits(['file-select', 'close'])
const props = defineProps({
    isUploading: Boolean,
    uploadProgress: Number,
    visible: Boolean
})

const fileInput = ref(null)

function openFileInput() {
    fileInput.value?.click()
}

async function handleFileSelect(event) {
    const result = await emitAsync('file-select', event)
    if (result !== false) close()
}

function emitAsync(name, payload) {
    return new Promise((resolve) => {
        const res = emit(name, payload)
        if (Array.isArray(res) && res.length > 0) resolve(res[0])
        else resolve()
    })
}

function close() {
    if (!props.isUploading) emit('close')
}

defineExpose({
    openFileInput
})
</script>

<style lang="scss" scoped>
@use "@/styles/Dashboard/Upload.scss" as *;
</style>