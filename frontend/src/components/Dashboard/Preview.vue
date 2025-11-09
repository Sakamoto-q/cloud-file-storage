<template>
    <div v-if="previewFile" class="modal-overlay" @click.self="$emit('close')">
        <div class="modal">
            <div class="modal-header">
                <div class="header-left">
                    <div class="file-icon">
                        <Image v-if="fileType === 'image'" :size="22" />
                        <Video v-else-if="fileType === 'video'" :size="22" />
                        <Music v-else-if="fileType === 'audio'" :size="22" />
                        <File v-else :size="22" />
                    </div>
                    <div class="file-info-header">
                        <h2>{{ previewFile.filename }}</h2>
                        <p class="file-meta">
                            ID: {{ previewFile.id }} • {{ formatDatetime(previewFile.created_at) }}
                        </p>
                    </div>
                </div>
                <button class="close-btn" @click="$emit('close')">
                    <X :size="22" />
                </button>
            </div>
            <div class="modal-preview">
                <div v-if="fileType === 'image'" class="preview-content image-preview">
                    <img :src="fileUrl" :alt="previewFile.filename" />
                </div>

                <div v-else-if="fileType === 'video'" class="preview-content video-preview">
                    <video controls crossorigin="anonymous">
                        <source :src="fileUrl" :type="mediaType" />
                        {{ $t('unsupported_video') }}
                    </video>
                </div>

                <div v-else-if="fileType === 'audio'" class="preview-content audio-preview">
                    <div class="audio-icon">
                        <Music :size="48" />
                    </div>
                    <audio controls crossorigin="anonymous">
                        <source :src="fileUrl" :type="mediaType" />
                        {{ $t('unsupported_audio') }}
                    </audio>
                </div>

                <div v-else class="preview-content file-preview">
                    <div class="file-icon-large">
                        <File :size="56" />
                    </div>
                    <p class="no-preview-title">{{ $t('no_preview') }}</p>
                    <p class="no-preview-desc">{{ $t('download_file_message') }}</p>
                </div>
            </div>
            <div class="modal-footer">
                <a 
                    v-if="previewFile.downloadUrl"
                    :href="previewFile.downloadUrl" 
                    download
                    class="download-btn"
                >
                    <Download :size="20" />
                    {{ $t('download') }}
                </a>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { Image, Video, Music, File, X, Download } from 'lucide-vue-next'

const props = defineProps({
    previewFile: Object
})

const emit = defineEmits(['close'])

const fileType = computed(() => {
    if (!props.previewFile) return 'file'
    return getFileTypeFromExtension(props.previewFile.filename)
})

const mediaType = computed(() => {
    if (!props.previewFile) return ''
    return getMimeType(props.previewFile.filename)
})

const fileUrl = computed(() => {
    if (!props.previewFile) return ''
    return props.previewFile.downloadUrl || ''
})

const getFileTypeFromExtension = (filename) => {
    if (!filename) return 'file'
    
    const ext = filename.split('.').pop().toLowerCase()
    
    const typeMap = {
        'jpg': 'image', 'jpeg': 'image', 'png': 'image', 'gif': 'image', 
        'webp': 'image', 'svg': 'image', 'bmp': 'image', 'ico': 'image',
        'mp4': 'video', 'webm': 'video', 'mov': 'video', 'avi': 'video',
        'mkv': 'video', 'flv': 'video', 'wmv': 'video', 'm3u8': 'video',
        'mp3': 'audio', 'wav': 'audio', 'aac': 'audio', 'm4a': 'audio',
        'flac': 'audio', 'ogg': 'audio', 'wma': 'audio'
    }
    
    return typeMap[ext] || 'file'
}

const getMimeType = (filename) => {
    const mimeMap = {
        'jpg': 'image/jpeg', 'jpeg': 'image/jpeg', 'png': 'image/png', 'gif': 'image/gif',
        'webp': 'image/webp', 'svg': 'image/svg+xml', 'bmp': 'image/bmp', 'ico': 'image/x-icon',
        'mp4': 'video/mp4', 'webm': 'video/webm', 'mov': 'video/quicktime', 'avi': 'video/x-msvideo',
        'mkv': 'video/x-matroska', 'flv': 'video/x-flv', 'wmv': 'video/x-ms-wmv', 'm3u8': 'application/x-mpegURL',
        'mp3': 'audio/mpeg', 'wav': 'audio/wav', 'aac': 'audio/aac', 'm4a': 'audio/mp4',
        'flac': 'audio/flac', 'ogg': 'audio/ogg', 'wma': 'audio/x-ms-wma'
    }
    const ext = filename.split('.').pop().toLowerCase()
    return mimeMap[ext] || 'application/octet-stream'
}

const formatDatetime = (dateString) => {
    return new Date(dateString).toLocaleString('ko-KR')
}
</script>

<style lang="scss" scoped>
@use "@/styles/Dashboard/Preview.scss" as *;
</style>