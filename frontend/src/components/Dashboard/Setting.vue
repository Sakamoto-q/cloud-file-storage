<template>
    <div class="sessions-container">
        <div v-if="!userData || !userData.sessions" class="state-message">
            <div class="spinner"></div>
            <p>{{ t('loading_sessions') }}</p>
        </div>
        <div v-else-if="userData.sessions.length === 0" class="state-message empty">
            <p>{{ t('no_active_sessions') }}</p>
        </div>
        <div v-else class="sessions-list">
            <div
                v-for="session in userData.sessions"
                :key="session.id"
                class="session-item"
                :class="{ 'current-session': session.id === userData.session_id }"
            >
                <div class="session-info">
                    <div class="session-header">
                        <div class="ip-address">
                            <Server :size="18" />
                            <span :title="session.ip_address">{{ session.ip_address }}</span>
                            <span
                                v-if="session.id === userData.session_id"
                                class="badge badge-current"
                            >
                                {{ t('current_session') }}
                            </span>
                        </div>
                    </div>
                    <div class="meta">
                        <span class="date">{{ formatDate(session.last_accessed_at) }}</span>
                        <span class="user-agent" :title="session.user_agent">{{ session.user_agent }}</span>
                    </div>
                </div>
                <button
                    class="logout-btn"
                    @click="handleLogoutSession(session.id)"
                    :disabled="loadingId === session.id"
                    :aria-label="`${session.ip_address} ${t('logout_session')}`"
                >
                    <LogOut :size="16" />
                    {{ loadingId === session.id ? t('logging_out') : t('logout') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { LogOut, Server } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const props = defineProps({
    userData: { type: Object, required: true },
})

const emit = defineEmits(['logout'])
const { t } = useI18n()

const loadingId = ref(null)

const handleLogoutSession = async (sessionId) => {
    loadingId.value = sessionId
    await emit('logout', sessionId)
    loadingId.value = null
}

const formatDate = (d) => {
    return new Date(d).toLocaleDateString('ko-KR', {
        month: 'short',
        day: 'numeric',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
    })
}
</script>

<style scoped lang="scss">
@use "@/styles/Dashboard/Setting.scss" as *;
</style>