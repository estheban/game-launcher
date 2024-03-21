<script setup lang="ts">
import {ref, onMounted, computed, reactive} from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import ProgressBar from 'primevue/progressbar';
import GameManifestService from "../services/GameManifestService.ts";
import {Manifest, Chunk} from "../types/Manifest.ts";
import SettingService from "../services/SettingService.ts";


const manifest = ref<Manifest>({
  install_directory: "",
  version: "",
  name: '__GAME_NAME__',
  files: []
});
const current_manifest = ref<Manifest | null>(null);

const GameManifestRepository = new GameManifestService();
const SettingRepository = new SettingService();

const platform = await invoke("get_platform");

async function downloadFile(fileDetail: Chunk, install_directory: string) {
  const saveFolder = await SettingRepository.get("storage_folder")
  // todo: dynamic url and path
  const downloadUrl = 'https://yulbrew-game-launcher-dev.s3.ca-central-1.amazonaws.com/73dd1271-d2d9-4db6-9618-13ddec1a073b/' + platform + '/' + fileDetail.hash;
  const savePath = saveFolder + '/' + install_directory +'/' + fileDetail.name;

  console.log(fileDetail);
  await invoke("download_file_to_path", {
    url: downloadUrl,
    path: savePath,
    hash: fileDetail.hash,
    filePermissions: fileDetail.file_permissions
  });
}

async function downloadGame() {
  const downloadPromises = manifest.value.files.map(file => downloadFile(file, manifest.value.install_directory));

  await Promise.all(downloadPromises);

  // All files have been downloaded, you can trigger the next process here
  console.log('All files have been downloaded');

  await SettingRepository.set("game_manifest", manifest.value);
  console.log('Game manifest has been saved');
}

async function getManifests() {
  manifest.value = await GameManifestRepository.get();
  current_manifest.value = await SettingRepository.get("game_manifest") as Manifest | null;

  console.log(manifest.value);
}

const totalSize = computed(() => {
  return manifest.value.files.reduce((total, file) => total + file.size, 0);
});

const downloadedSize = computed(() => {
  return Object.values(fileDownloadProgress).reduce((total, downloaded) => total + downloaded, 0);
});
const progressRatio = computed(() => {
  return Math.round(downloadedSize.value / totalSize.value * 100);
});

const canInstall = computed(() => {
  return manifest.value.version !== current_manifest.value?.version;
});

interface FileDownloadProgress {
  downloaded: number;
  hash: string;
}

const fileDownloadProgress = reactive<{ [key: string]: number }>({});

onMounted(() => {
  listen('file_download_progress', (event) => {
    const payload = event.payload as FileDownloadProgress;
    fileDownloadProgress[payload.hash] = payload.downloaded;
  });

  getManifests();
});

// onUnmounted(() => {
//   if (progressListener) {
//     progressListener.un
//     event.unlisten(progressListener);
//   }
// });

function formatBytes(bytes: number, decimals = 2) {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

function forceInstall() {
  current_manifest.value = null;
  downloadGame();
}

// - Get game version
// - if no installed game
// - get game manifest
// - enable install button
// - on click, need to accept EULA
// - else
// - get game version
// - if game version is different
// - enable update button
// - else
// - enable play button
</script>

<template>
  <h2>{{ manifest.name }}</h2>
  <h3>Total Size: {{ formatBytes(totalSize) }}</h3>
  <h3>Downloaded Size: {{ formatBytes(downloadedSize) }}</h3>
  <button @click="getManifests">Refresh</button>

  <button v-if="canInstall" @click="downloadGame()">Install</button>
  <button @click="forceInstall()">Force Install</button>

  <ProgressBar v-if="canInstall" :value="progressRatio"></ProgressBar>
</template>
