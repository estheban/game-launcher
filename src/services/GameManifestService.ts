
import { fetch, ResponseType } from '@tauri-apps/api/http';
import {Manifest} from "../types/Manifest.ts";

export default class GameManifestService {
    async get(): Promise<Manifest> {
        // const response = await fetch('http://127.0.0.1:8000/games/73dd1271-d2d9-4db6-9618-13ddec1a073b', {
        const response = await fetch('https://yulbrew-game-launcher-dev.s3.ca-central-1.amazonaws.com/73dd1271-d2d9-4db6-9618-13ddec1a073b/macos/manifest.json', {
            method: 'GET',
            timeout: 10,
            responseType: ResponseType.JSON,
        });
        return await response.data as Manifest;
    }
}
