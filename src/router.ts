
import {createRouter, createWebHistory, NavigationGuardNext, RouteLocationNormalized} from 'vue-router';
import FirstRunView from "./views/FirstRunView.vue";
import SettingService from "./services/SettingService.ts";
import GameDetailView from "./views/GameDetailView.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: GameDetailView,
        },
        {
            path: "/first-run",
            name: "first-run",
            component: FirstRunView,
        },
    ],
});

const settingService = new SettingService();

router.beforeEach(async (to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) => {
    const setting = await settingService.get('storage_folder');
    console.log(setting);
    console.log(from);
    if (setting === null && to.name !== 'first-run') {
        next({ name: 'first-run' }); // replace 'first-run' with the name of the route you want to redirect to
    } else {
        next();
    }
});

export default router;
