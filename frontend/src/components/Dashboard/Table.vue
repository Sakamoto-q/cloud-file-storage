<template>
    <div class="files-container" @click="closeMenu">
        <div v-if="loading" class="state-message">
            <div class="spinner"></div>
            <p>{{ $t('loading_files') }}</p>
        </div>
        <div v-else-if="error" class="state-message error">
            <p>{{ error }}</p>
        </div>
        <div v-else-if="files.length === 0" class="state-message empty">
            <p>{{ $t('no_files') }}</p>
        </div>
        <div v-else class="files-list">
            <div
                v-for="file in files"
                :key="file.id"
                class="file-item"
                @click.stop="openMenu($event, file)"
                @contextmenu.prevent.stop="openMenu($event, file)"
                tabindex="0"
                @keyup.enter="openMenu($event, file)"
            >
                <div class="file-info">
                    <div class="filename">
                        <File :size="18" />
                        <span :title="file.filename">{{ file.filename }}</span>
                    </div>
                    <div class="meta">
                        <span class="date">{{ formatDate(file.created_at) }}</span>
                        <span class="access">
                            <Users :size="16" />
                            {{ file.accessible_user_ids.length }}
                        </span>
                    </div>
                </div>
            </div>
        </div>
        <teleport to="body">
            <transition name="menu">
                <div
                    v-if="menuVisible"
                    class="action-menu"
                    :style="{ top: menuPosition.y + 'px', left: menuPosition.x + 'px' }"
                    @click.stop
                    role="menu"
                >
                    <button class="menu-btn preview" @click.stop="$emit('preview', currentFile); closeMenu()" role="menuitem">
                        <Eye :size="16" /> {{ $t('preview') }}
                    </button>
                    <button class="menu-btn share" @click.stop="$emit('share', currentFile); closeMenu()" role="menuitem">
                        <Share2 :size="16" /> {{ $t('share') }}
                    </button>
                    <button class="menu-btn delete" @click.stop="$emit('delete', currentFile.id); closeMenu()" role="menuitem">
                        <Trash2 :size="16" /> {{ $t('delete') }}
                    </button>
                </div>
            </transition>
        </teleport>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { File, Users, Eye, Share2, Trash2 } from 'lucide-vue-next'

const props = defineProps({
    files: Array,
    loading: Boolean,
    error: String
})

const emit = defineEmits(['preview', 'share', 'delete'])

const menuVisible = ref(false)
const menuPosition = ref({ x: 0, y: 0 })
const currentFile = ref(null)

const openMenu = (e, file) => {
    e.preventDefault()
    currentFile.value = file
    const clickX = e.clientX
    const clickY = e.clientY
    const menuW = 160
    const menuH = 132
    const offset = 8
    let x = clickX + offset
    let y = clickY + offset

    if (x + menuW > window.innerWidth) x = clickX - menuW - offset
    if (y + menuH > window.innerHeight) y = clickY - menuH - offset
    if (x < 0) x = 0
    if (y < 0) y = 0

    menuPosition.value = { x, y }
    menuVisible.value = true
}

const closeMenu = () => {
    menuVisible.value = false
}

const formatDate = (d) => {
    return new Date(d).toLocaleDateString('ko-KR', {
        month: 'short',
        day: 'numeric',
        year: 'numeric'
    })
}

const handleClickOutside = (e) => {
    if (menuVisible.value && !e.target.closest('.action-menu') && !e.target.closest('.file-item')) {
        closeMenu()
    }
}

onMounted(() => document.addEventListener('click', handleClickOutside))
onUnmounted(() => document.removeEventListener('click', handleClickOutside))
</script>

<style scoped lang="scss">
@use "@/styles/Dashboard/Table.scss" as *;
</style>