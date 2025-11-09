<template>
    <div class="dashboard-header">
        <div class="header-right">
            <div class="user-info">
                <div class="user-avatar">
                    <img 
                        v-if="userData.icon_url" 
                        :src="userData.icon_url" 
                        :alt="userData.email"
                    >
                    <span v-else class="avatar-placeholder">
                        {{ userData.email[0].toUpperCase() }}
                    </span>
                </div>
                <div class="user-details">
                    <p class="user-email">{{ userData.email }}</p>
                    <p class="user-id">ID: {{ userData.id }}</p>
                </div>
            </div>

            <button class="action-btn upload" @click="$emit('show-upload')" :aria-label="$t('upload_file')">
                <Upload :size="20" />
                <span>{{ $t('upload') }}</span>
            </button>

            <button class="action-btn logout" @click="handleLogout" :aria-label="$t('logout')">
                <LogOut :size="20" />
                <span>{{ $t('logout') }}</span>
            </button>
        </div>
    </div>
</template>

<script setup>
import { Upload, LogOut } from 'lucide-vue-next'

const props = defineProps({
    userData: Object
})

const emit = defineEmits(['show-upload', 'logout'])
const handleLogout = () => emit('logout')
</script>

<style scoped lang="scss">
@use "@/styles/Dashboard/Header.scss" as *;
</style>
