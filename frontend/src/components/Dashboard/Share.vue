<template>
    <div v-if="file" class="modal-overlay" @click.self="$emit('close')">
        <div class="modal share-modal">
            <div class="modal-header">
                <h2>{{ $t('share_settings') }}</h2>
                <button class="close-btn" @click="$emit('close')" aria-label="닫기">
                    <X :size="22" />
                </button>
            </div>
            <div class="modal-body">
                <div v-if="shareLink" class="section">
                    <label class="label">{{ $t('share_link') }}</label>
                    <div class="input-group">
                        <input :value="shareLink" readonly class="input" />
                        <button class="btn copy" @click="copyLink" :aria-label="copied ? $t('copied') : $t('copy')">
                            <Check v-if="copied" :size="18" />
                            <Copy v-else :size="18" />
                            {{ copied ? $t('copied') : $t('copy') }}
                        </button>
                    </div>
                </div>
                <div class="section">
                    <label class="label">{{ $t('add_user') }}</label>
                    <div class="input-group">
                        <input
                            v-model="newUserId"
                            :placeholder="$t('user_id_placeholder')"
                            class="input"
                            @keyup.enter="addUser"
                        />
                        <button class="btn add" @click="addUser">
                            <Plus :size="18" />
                            {{ $t('add') }}
                        </button>
                    </div>
                </div>
                <div class="section">
                    <label class="label">{{ $t('add_user') }} ({{ shareEmails.length }})</label>
                    <div class="users-list">
                        <div v-if="!shareEmails.length" class="empty">{{ $t('no_shared_users') }}</div>
                        <div v-for="email in shareEmails" :key="email" class="user-item">
                            <span>{{ email }}</span>
                            <button class="remove-btn" @click="removeUser(email)" aria-label="제거">
                                <X :size="18" />
                            </button>
                        </div>
                    </div>
                </div>
                <div class="actions">
                    <button class="btn cancel" @click="$emit('close')">{{ $t('cancel') }}</button>
                    <button class="btn save" @click="save" :disabled="loading">
                        <Save v-if="!loading" :size="18" />
                        {{ loading ? $t('saving') : $t('save') }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { X, Copy, Check, Plus, Save } from 'lucide-vue-next'

const props = defineProps({
    file: Object,
    shareLink: String,
    initialEmails: Array,
    loading: Boolean
})

const emit = defineEmits(['close', 'save'])

const shareEmails = ref([])
const newUserId = ref('')
const copied = ref(false)

watch(() => props.initialEmails, (v) => {
    shareEmails.value = v ? [...v] : []
}, { immediate: true })

const addUser = () => {
    if (!newUserId.value.trim()) return
    if (shareEmails.value.includes(newUserId.value)) {
        alert($t('already_added'))
        return
    }
    shareEmails.value.push(newUserId.value.trim())
    newUserId.value = ''
}

const removeUser = (email) => {
    shareEmails.value = shareEmails.value.filter(e => e !== email)
}

const save = () => {
    emit('save', shareEmails.value)
}

const copyLink = async () => {
    await navigator.clipboard.writeText(props.shareLink)
    copied.value = true
    setTimeout(() => copied.value = false, 2000)
}
</script>

<style scoped lang="scss">
@use "@/styles/Dashboard/Share.scss" as *;
</style>