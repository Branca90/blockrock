import * as buffer from 'buffer';
import { Buffer } from 'buffer';
if (typeof window !== 'undefined') {
  window.buffer = buffer;
  window.Buffer = Buffer;
  window.global ||= window; // Suggerito da TronWeb
}

import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { createPinia } from 'pinia';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import 'vuetify/styles';
import Toast from 'vue-toastification';
import 'vue-toastification/dist/index.css';
import { Vue3Lottie } from 'vue3-lottie';

const vuetify = createVuetify({
  components,
  directives,
});

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(vuetify);
app.use(Toast);
app.component('Vue3Lottie', Vue3Lottie);
app.mount('#app');
