
import {Store} from "tauri-plugin-store-api";

const store = new Store(".settings.dat");

export default class SettingService {
    async get(setting_name: string) {
        return await store.get(setting_name);
    }

    async set(setting_name: string, value: any) {
        await store.set(setting_name, {value: value});
        await store.save();
    }
}
