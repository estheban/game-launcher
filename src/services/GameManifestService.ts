
import { fetch } from '@tauri-apps/plugin-http';
import {Manifest} from "../types/Manifest.ts";
import { invoke } from "@tauri-apps/api/core";

export default class GameManifestService {
    async get_download_prefix(): Promise<string> {
        return 'https://yulbrew-game-launcher-dev.s3.ca-central-1.amazonaws.com/73dd1271-d2d9-4db6-9618-13ddec1a073b/' + await invoke('get_platform') + '/';
    }

    async get(): Promise<Manifest> {
        // @todo: handle errors
        // const response = await fetch('http://127.0.0.1:8000/games/73dd1271-d2d9-4db6-9618-13ddec1a073b', {
        const response = await fetch(await this.get_download_prefix() + 'manifest.json', {
            method: 'GET',
            timeout: 10,
            // responseType: ResponseType.JSON,
        });
        console.log(response.status); // e.g. 200

        const jsonData: Manifest = await response.json();

        return jsonData;
    }
}
