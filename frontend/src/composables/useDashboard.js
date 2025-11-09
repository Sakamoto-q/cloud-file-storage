import { ref, onMounted } from 'vue'
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
    const headerRef = ref(null)
    const showUpload = ref(false)

    function header(email, pw) {
        return "Basic " + btoa(`${email}:${pw}`)
    }

    const getHeaders = () => ({
        'Authorization': localStorage.getItem('header')
    })

    async function Login(email = "", pw = "") {
        try {
            let headers = ""
            if (!email || !pw) {
                headers = localStorage.getItem('header')
                if (!headers) return
            } else {
                headers = header(email, pw)
            }
            const response = await axios.get(window.api + "/user", {
                headers: { 'Authorization': headers }
            })
            if (response.status === 200) {
                localStorage.setItem('header', headers)
                userData.value = response.data.data
                localStorage.setItem('userData', JSON.stringify(response.data.data))
                logined.value = true
                await fetchFiles()
            }
        } catch (err) {
            alert(t('login_error', { error: err.response?.data?.error?.detail }))
            logined.value = false
        }
    }

    async function Signup(email, pw, turnstile) {
        try {
            const response = await axios.post(window.api + "/user", { email, password: pw, turnstile })
            if ([200, 201].includes(response.status)) {
                const headers = header(email, pw)
                localStorage.setItem('header', headers)
                userData.value = response.data.data
                localStorage.setItem('userData', JSON.stringify(response.data.data))
                logined.value = true
                await fetchFiles()
            }
        } catch (err) {
            alert(t('signup_error', { error: err.response?.data?.error?.detail }))
            logined.value = false
        }
    }

    async function fetchFiles() {
        try {
            loading.value = true
            const response = await axios.get(window.api + "/content", { headers: getHeaders() })
            files.value = response.data.data.files || []
            error.value = null
        } catch (err) {
            error.value = err.response?.data?.error?.detail || t('fetch_files_error')
        } finally {
            loading.value = false
        }
    }

    async function handlePreview(file) {
        try {
            loadingAction.value = 'preview-' + file.id
            const response = await axios.get(window.api + `/content/${file.id}/share`, { headers: getHeaders() })
            previewFile.value = { ...file, downloadUrl: response.data.data.url }
        } catch (err) {
            alert(t('preview_fail', { error: err.response?.data?.error?.detail }))
        } finally {
            loadingAction.value = null
        }
    }

    async function handleShareClick(file) {
        try {
            loadingAction.value = 'share-' + file.id
            const response = await axios.get(window.api + `/content/${file.id}`, { headers: getHeaders() })
            shareModal.value = file
            shareEmails.value = response.data.data.accessible_user_ids || []

            try {
                const linkResponse = await axios.get(window.api + `/content/${file.id}/share`, { headers: getHeaders() })
                shareLink.value = linkResponse.data.data.url
            } catch {}
        } catch (err) {
            alert(t('share_info_fail', { error: err.response?.data?.error?.detail }))
        } finally {
            loadingAction.value = null
        }
    }

    async function handleSaveShare(updatedEmails) {
        if (!shareModal.value) return
        try {
            loadingAction.value = 'save-share'
            const response = await axios.put(window.api + `/content/${shareModal.value.id}/share`,
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
            alert(err.response?.data?.error?.detail)
        } finally {
            loadingAction.value = null
        }
    }

    async function handleDelete(fileId) {
        if (!confirm(t('delete_confirm'))) return
        try {
            loadingAction.value = 'delete-' + fileId
            const response = await axios.delete(window.api + `/content/${fileId}`, { headers: getHeaders() })
            if (response.status === 200) {
                await fetchFiles()
            }
        } catch (err) {
            alert(err.response?.data?.error?.detail)
        } finally {
            loadingAction.value = null
        }
    }

    function handleLogout() {
        localStorage.removeItem('header')
        localStorage.removeItem('userData')
        logined.value = false
        userData.value = null
        files.value = []
    }

    async function handleFileSelect(event) {
        const file = event.target.files[0]
        if (!file) return

        try {
            isUploading.value = true
            uploadProgress.value = 0

            const createResponse = await axios.post(window.api + "/content", { filename: file.name }, { headers: getHeaders() })
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
            alert(t('upload_fail', { error: err.response?.data?.error?.detail || err.message }))
        } finally {
            isUploading.value = false
            uploadProgress.value = 0
            event.target.value = ''
        }
    }

    onMounted(() => {
        Login()
    })

    return {
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
        Login,
        Signup,
        fetchFiles,
        handlePreview,
        handleShareClick,
        handleSaveShare,
        handleDelete,
        handleLogout,
        handleFileSelect,
        openUploadInput: () => headerRef.value?.openFileInput()
    }
}
