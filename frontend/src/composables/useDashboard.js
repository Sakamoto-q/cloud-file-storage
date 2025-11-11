import { inject, ref, onMounted } from 'vue'
import axios from 'axios'
import { useI18n } from 'vue-i18n'

export function useDashboard() {
    const { t } = useI18n()

    const logined = ref(false)
    const userData = ref(null)
    const files = ref([])
    const loading = ref(true)
    const error = ref(null)
    const previewFile = ref(null)
    const shareModal = ref(null)
    const shareEmails = ref([])
    const shareLink = ref(null)
    const loadingAction = ref(null)
    const uploadProgress = ref(0)
    const isUploading = ref(false)
    const showSetting = ref(false)
    const headerRef = ref(null)
    const showUpload = ref(false)

    const authToken = ref(null)

    const API_BASE = inject('API_BASE')

    const getHeaders = () => {
        if (!authToken.value) return {}
        return { 'Authorization': `Bearer ${authToken.value}` }
    }

    async function Login(email = "", pw = "") {
        try {
            if (!email || !pw) {
                return
            }

            const response = await axios.post(`${API_BASE}/session`, { email, password: pw })

            if (response.status === 200 && response.data.data) {
                userData.value = response.data.data
                authToken.value = response.data.data.session_key
                logined.value = true
                await fetchFiles()
            }
        } catch (err) {
            console.error('Login error:', err)
            alert(t('login_error', { error: err.response?.data?.error?.detail || err.message }))
            logined.value = false
        }
    }

    async function Signup(email, pw, turnstile) {
        try {
            const safeEmail = email.trim().toLowerCase()
            const response = await axios.post(`${API_BASE}/user`, { email: safeEmail, password: pw, turnstile })
            if ([200, 201].includes(response.status)) {
                userData.value = response.data.data
                authToken.value = response.data.data.session_key
                logined.value = true
                await fetchFiles()
            }
        } catch (err) {
            console.error('Signup error:', err)
            alert(t('signup_error', { error: err.response?.data?.error?.detail || err.message }))
            logined.value = false
        }
    }

    async function fetchFiles() {
        try {
            loading.value = true
            const response = await axios.get(`${API_BASE}/content`, { headers: getHeaders() })
            files.value = response.data.data.files || []
            error.value = null
        } catch (err) {
            error.value = err.response?.data?.error?.detail || t('fetch_files_error')
        } finally {
            loading.value = false
        }
    }

    async function handleSetting() {
        showSetting.value = !showSetting.value
    }

    async function handlePreview(file) {
        try {
            loadingAction.value = 'preview-' + file.id
            const response = await axios.get(`${API_BASE}/content/${file.id}/share`, { headers: getHeaders() })
            previewFile.value = { ...file, downloadUrl: response.data.data.url }
        } catch (err) {
            console.error('Preview error:', err)
            alert(t('preview_fail', { error: err.response?.data?.error?.detail || err.message }))
        } finally {
            loadingAction.value = null
        }
    }

    async function handleShareClick(file) {
        try {
            loadingAction.value = 'share-' + file.id
            const response = await axios.get(`${API_BASE}/content/${file.id}`, { headers: getHeaders() })
            shareModal.value = file
            shareEmails.value = response.data.data.accessible_user_ids || []

            try {
                const linkResponse = await axios.get(`${API_BASE}/content/${file.id}/share`, { headers: getHeaders() })
                shareLink.value = linkResponse.data.data.url
            } catch {
                console.error('Share link retrieval failed')
            }
        } catch (err) {
            console.error('Share info error:', err)
            alert(t('share_info_fail', { error: err.response?.data?.error?.detail || err.message }))
        } finally {
            loadingAction.value = null
        }
    }

    async function handleSaveShare(updatedEmails) {
        if (!shareModal.value) return
        try {
            loadingAction.value = 'save-share'
            const response = await axios.put(
                `${API_BASE}/content/${shareModal.value.id}/share`,
                { accessible_user_ids: updatedEmails },
                { headers: getHeaders() }
            )
            if (response.status === 200) {
                shareModal.value = null
                shareEmails.value = []
                shareLink.value = null
                await fetchFiles()
            }
        } catch (err) {
            console.error('Save share error:', err)
            alert(err.response?.data?.error?.detail || t('save_share_fail'))
        } finally {
            loadingAction.value = null
        }
    }

    async function handleDelete(fileId) {
        if (!confirm(t('delete_confirm'))) return
        try {
            loadingAction.value = 'delete-' + fileId
            const response = await axios.delete(`${API_BASE}/content/${fileId}`, { headers: getHeaders() })
            if (response.status === 200) {
                await fetchFiles()
            }
        } catch (err) {
            console.error('Delete error:', err)
            alert(err.response?.data?.error?.detail || t('delete_fail'))
        } finally {
            loadingAction.value = null
        }
    }

    async function handleLogout(id) {
        try {
            loadingAction.value = 'logout-' + id
            await axios.delete(`${API_BASE}/session/${id}`, { headers: getHeaders() })
        } catch (err) {
            console.error('Logout error:', err)
            alert(t('logout_fail', { error: err.response?.data?.error?.detail || err.message }))
        } finally {
            loadingAction.value = null
        }
        try {
            const response = await axios.get(`${API_BASE}/session`, { headers: getHeaders() })

            if (response.status === 200 && response.data.data) {
                userData.value = response.data.data
                authToken.value = response.data.data.session_key
                logined.value = true
                await fetchFiles()
            }
        } catch (err) {
            console.error(err)
            userData.value = null
            authToken.value = null
            logined.value = false
        }
    }

    async function handleFileSelect(event) {
        const file = event.target.files[0]
        if (!file) return

        const allowedTypes = ['image/png', 'image/jpeg', 'text/plain']
        if (!allowedTypes.includes(file.type)) {
            alert(t('upload_invalid_type'))
            return
        }

        try {
            isUploading.value = true
            uploadProgress.value = 0

            const createResponse = await axios.post(`${API_BASE}/content`, { filename: file.name }, { headers: getHeaders() })
            const uploadUrl = createResponse.data.data.url

            await axios.put(uploadUrl, file, {
                headers: { 'Content-Type': file.type || 'application/octet-stream' },
                onUploadProgress: (progressEvent) => {
                    uploadProgress.value = Math.round((progressEvent.loaded / progressEvent.total) * 100)
                }
            })

            await fetchFiles()
            showUpload.value = false
        } catch (err) {
            console.error('Upload error:', err)
            alert(t('upload_fail', { error: err.response?.data?.error?.detail || err.message }))
        } finally {
            isUploading.value = false
            uploadProgress.value = 0
            event.target.value = ''
        }
    }

    function openUploadInput() {
        headerRef.value?.openFileInput()
    }

    return {
        authToken,
        logined,
        userData,
        files,
        loading,
        error,
        previewFile,
        shareModal,
        shareEmails,
        shareLink,
        loadingAction,
        uploadProgress,
        isUploading,
        headerRef,
        showUpload,
        showSetting,
        Login,
        Signup,
        fetchFiles,
        handleSetting,
        handlePreview,
        handleShareClick,
        handleSaveShare,
        handleDelete,
        handleLogout,
        handleFileSelect,
        openUploadInput
    }
}