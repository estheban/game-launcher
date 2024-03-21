<script setup lang="ts">
import {ref, onMounted, computed, reactive} from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import Button from 'primevue/button';
import SplitButton from 'primevue/splitbutton';
import ProgressBar from 'primevue/progressbar';
// import Skeleton from 'primevue/skeleton';
import { useToast } from "primevue/usetoast";

import GameManifestService from "../services/GameManifestService.ts";
import {Manifest, Chunk} from "../types/Manifest.ts";
import SettingService from "../services/SettingService.ts";

const toast = useToast();

const manifest = ref<Manifest>({
  install_directory: "",
  version: "",
  name: '__GAME_NAME__',
  files: []
});
const current_manifest = ref<Manifest | null>(null);
const installInProgress = ref(false);

const GameManifestRepository = new GameManifestService();
const SettingRepository = new SettingService();

async function downloadFile(fileDetail: Chunk, install_directory: string) {
  const saveFolder = await SettingRepository.get("storage_folder");

  // todo: dynamic url and path
  const downloadUrl = await GameManifestRepository.get_download_prefix() + fileDetail.hash;
  const savePath = saveFolder + '/' + install_directory +'/' + fileDetail.name;

  await invoke("download_file_to_path", {
    url: downloadUrl,
    path: savePath,
    hash: fileDetail.hash,
    filePermissions: fileDetail.file_permissions
  });
}

async function downloadGame() {
  installInProgress.value = true;

  const downloadPromises = manifest.value.files.map(file => downloadFile(file, manifest.value.install_directory));

  await Promise.all(downloadPromises);

  // All files have been downloaded, you can trigger the next process here
  console.log('All files have been downloaded');

  await SettingRepository.set("game_manifest", manifest.value);
  console.log('Game manifest has been saved');
  installInProgress.value = false;
  await getManifests();

  // Cleanup
  Object.keys(fileDownloadProgress).forEach(key => {
    delete fileDownloadProgress[key];
  });

  toast.add({ severity: 'success', summary: 'Game Installed', detail: 'All files have been downloaded', life: 3000 });
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

// function formatBytes(bytes: number, decimals = 2) {
//   if (bytes === 0) return '0 Bytes';
//
//   const k = 1024;
//   const dm = decimals < 0 ? 0 : decimals;
//   const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
//
//   const i = Math.floor(Math.log(bytes) / Math.log(k));
//
//   return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
// }

function forceInstall() {
  current_manifest.value = null;
  downloadGame();
}

async function startGame() {
  const baseFolder = await SettingRepository.get("storage_folder");
  toast.add({ severity: 'success', summary: 'Starting Game', detail: '', life: 3000 });
  await invoke('run_program', {
    appName: 'SurvivalGame',
    baseFolder: baseFolder,
    installDirectory: manifest.value.install_directory
  });
}

async function uninstallGame() {
  toast.add({ severity: 'warn', summary: 'Uninstalling Game', detail: '', life: 6000 });

  const baseFolder = await SettingRepository.get("storage_folder");
  await invoke('uninstall_game', {
    baseFolder: baseFolder,
    installDirectory: manifest.value.install_directory
  });

  await SettingRepository.set("game_manifest", null);
  await getManifests();

  toast.add({ severity: 'success', summary: 'Game Uninstalled', detail: '', life: 6000 });
}

const backgroundImage = ref('https://source.unsplash.com/random/800x500');
// const backgroundImage = ref('https://cdn.cloudflare.steamstatic.com/steam/apps/1149460/library_hero.jpg');

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

const items = [
  {
    label: 'Re-Install',
    icon: 'pi pi-refresh',
    command: () => {
      toast.add({ severity: 'warn', summary: 'Updated', detail: 'Force Install Started', life: 3000 });
      forceInstall();
    }
  },
  {
    label: 'Uninstall',
    icon: 'pi pi-times',
    command: () => {
      uninstallGame();
    }
  }
];
</script>

<template>
  <div class="image-container">
    <img :src="backgroundImage" alt="Background Image" />
<!--    <Skeleton width="100%" height="200px"></Skeleton>-->
    <div class="overlay">
      <h2>{{ manifest.name }}</h2>
<!--      <p>{{ manifest.description }}</p>-->
      <Button v-if="canInstall" :disabled="installInProgress" @click="downloadGame()" label="Install" />

      <SplitButton severity="success" v-if="!canInstall" label="Play" icon="pi pi-check" menuButtonIcon="pi pi-cog" :model="items" @click="startGame()" />
    </div>
  </div>
<!--  <h3>Total Size: {{ formatBytes(totalSize) }}</h3>-->
<!--  <h3>Downloaded Size: {{ formatBytes(downloadedSize) }}</h3>-->
  <ProgressBar v-if="canInstall" :value="progressRatio"></ProgressBar>
</template>

<style scoped>
.image-container {
  position: relative;
  width: 100%;
}

.image-container img {
  width: 100%;
  object-fit: cover;
}

.image-container .overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
</style>
