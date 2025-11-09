import { createI18n } from 'vue-i18n'
import ko from './locales/ko.json'
import en from './locales/en.json'

const messages = {
    en,
    ko
};

const browser_locale = () => {
    const browserLocale = navigator.language || navigator.userLanguage;
    const locale = browserLocale.split('-')[0];
    
    if (messages[locale]) {
        return locale;
    }
    
    return 'en';
}

export const i18n = createI18n({
    locale: browser_locale(),
    fallbackLocale: 'en',
    messages: messages
})

export default i18n