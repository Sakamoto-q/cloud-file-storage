<template>
    <div>
        <Account v-if="!logined" :Login="Login" :Signup="Signup"/>
        <div v-else class="dashboard-container">
            <Header 
                ref="headerRef"
                :userData="userData"
                @setting="handleSetting"
                @show-upload="showUpload = true"
            />

            <transition name="fade">
                <Upload 
                    v-if="showUpload"
                    :visible="showUpload"
                    :isUploading="isUploading"
                    :uploadProgress="uploadProgress"
                    @file-select="handleFileSelect"
                    @close="showUpload = false"
                />
            </transition>

            <Table
                v-if="!showSetting"
                :files="files"
                :loading="loading"
                :error="error"
                :loadingAction="loadingAction"
                @preview="handlePreview"
                @share="handleShareClick"
                @delete="handleDelete"
            />

            <Setting
                v-if="showSetting"
                :userData="userData"
                @logout="handleLogout"
            />

            <transition name="fade">
                <Preview 
                    :previewFile="previewFile"
                    @close="previewFile = null"
                />
            </transition>

            <Share 
                :file="shareModal"
                :shareLink="shareLink"
                :initialEmails="shareEmails"
                :loading="loadingAction === 'save-share'"
                @close="shareModal = null"
                @save="handleSaveShare"
            />
        </div>
    </div>
</template>

<script setup>
import Account from '@/components/Account.vue'
import Upload from '@/components/Dashboard/Upload.vue'
import Header from '@/components/Dashboard/Header.vue'
import Table from '@/components/Dashboard/Table.vue'
import Preview from '@/components/Dashboard/Preview.vue'
import Share from '@/components/Dashboard/Share.vue'
import Setting from '@/components/Dashboard/Setting.vue'
import { useDashboard } from '@/composables/useDashboard.js'

const {
    logined,
    userData,
    files,
    loading,
    error,
    authToken,
    previewFile,
    shareModal,
    shareEmails,
    shareLink,
    loadingAction,
    uploadProgress,
    isUploading,
    headerRef,
    showSetting,
    showUpload,
    Login,
    Signup,
    handleLogout,
    handleSetting,
    handlePreview,
    handleShareClick,
    handleSaveShare,
    handleDelete,
    handleFileSelect,
    openUploadInput
} = useDashboard()
</script>