import { createApp } from "vue";

// PrimeVue
import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';

// Styles
import 'primeicons/primeicons.css';
import "primevue/resources/primevue.min.css";
import 'primevue/resources/themes/aura-dark-noir/theme.css';
import "./styles.css";

// VueJs
import App from "./App.vue";
import router from "./router";

const app = createApp(App);
app.use(router);
app.use(PrimeVue);
app.use(ToastService);
app.mount("#app")
