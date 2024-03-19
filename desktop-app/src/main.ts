import { createApp } from "vue";

// PrimeVue
import PrimeVue from 'primevue/config';
import 'primevue/resources/themes/aura-light-green/theme.css';

import "./styles.css";
import App from "./App.vue";

// createApp(App).mount("#app");
const app = createApp(App);
app.use(PrimeVue);
app.mount("#app")