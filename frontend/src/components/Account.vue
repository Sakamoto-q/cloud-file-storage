<template>
    <div>
        <div class="login-container" v-if="view === ViewLogin" @click="$emit('close')">
            <div class="login-box" @click.stop>
                <h1>{{ $t('login') }}</h1>
                <form @submit.prevent="login">
                    <div class="input-container">
                        <input
                            type="email"
                            v-model="email"
                            :placeholder="$t('email_placeholder')"
                            required
                        />
                    </div>
                    <div class="input-container">
                        <input
                            :type="showPassword ? 'text' : 'password'"
                            v-model="pw"
                            :placeholder="$t('password_placeholder')"
                            required
                        />
                    </div>
                    <button type="submit" :disabled="!checklogin">{{ $t('login_button') }}</button>
                </form>
                <div class="line"></div>
                <div class="links">
                    <a @click.prevent="view = ViewSignup">{{ $t('signup_link') }}</a>
                </div>
            </div>
        </div>
        <div class="login-container" v-if="view === ViewSignup" @click="$emit('close')">
            <div class="login-box" @click.stop>
                <h1>{{ $t('signup') }}</h1>
                <form @submit.prevent="signup">
                    <div class="input-container">
                        <input type="email" v-model="email" :placeholder="$t('email_placeholder')" required />
                    </div>
                    <div class="input-container">
                        <input :type="showPassword ? 'text' : 'password'" v-model="pw" :placeholder="$t('password_placeholder')" required />
                    </div>
                    <div class="input-container">
                        <input :type="showPasswordConfirm ? 'text' : 'password'" v-model="pwConfirm" :placeholder="$t('password_confirm_placeholder')" required />
                    </div>
                    <div class="turnstile-container" v-if="sitekey && turnstileToken === ''">
                        <Turnstile :sitekey="sitekey" :onSuccess="onTurnstileSuccess" />
                    </div>
                    <button v-if="turnstileToken !== ''" class="submit-button" type="submit" :disabled="!checksignup">
                        {{ $t('signup_button') }}
                    </button>
                </form>
                <div class="line"></div>
                <div class="links">
                    <a @click.prevent="view = ViewLogin">{{ $t('login_link') }}</a>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import Turnstile from './Turnstile.vue';

const ViewLogin  = 'login';
const ViewSignup = 'signup';

const view = ref(ViewLogin);
const email = ref('');
const pw = ref('');
const pwConfirm = ref('');
const turnstileToken = ref('');
const sitekey = ref('');

const showPassword = ref(false);
const showPasswordConfirm = ref(false);

const checklogin = computed(() => email.value !== '' && pw.value !== '');
const checksignup = computed(() =>
    email.value !== '' &&
    pw.value !== '' &&
    pw.value === pwConfirm.value
);

const props = defineProps({
    Login: { type: Function, required: true },
    Signup: { type: Function, required: true }
});

function login() {
    props.Login(email.value, pw.value);
}

function signup() {
    props.Signup(email.value, pw.value, turnstileToken.value);
}

function onTurnstileSuccess(token) {
    turnstileToken.value = token;
}

onMounted(async () => {
    try {
        const res = await fetch(window.api + '/turnstile');
        if (!res.ok) throw new Error('Failed to fetch sitekey');
        sitekey.value = await res.text();
    } catch (err) {
        console.error('Turnstile sitekey fetch error:', err);
    }
});
</script>

<style lang="scss" scoped>
@use "@/styles/Account.scss" as *;
</style>
