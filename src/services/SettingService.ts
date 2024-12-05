import { load } from '@tauri-apps/plugin-store';
const store = await load('settings.json', { autoSave: false });

// import {Store} from "tauri-plugin-store-api";
//
// const store = new Store("settings.json");

export default class SettingService {
    async get(setting_name: string) {
        const value = await store.get(setting_name);
        console.log('store get:', setting_name, value);
        return value;
    }

    async set(setting_name: string, value: any) {
        await store.set(setting_name, value);
        await store.save();
    }
}
