<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import ProgressBar from 'primevue/progressbar';

const greetMsg = ref("");
const name = ref("foobar");
const progressRatio = ref(0);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}



async function downloadFile() {
  const downloadUrl = 'http://gus.estheban.com:3000/games/SuivivalGameBuild/media/branch/main/Windows/SurvivalGame/Content/Paks/pakchunk0-Windows.pak';
  const savePath = '/Users/estheban/git/games/testInstall/pakchunk0-Windows.pak';

  console.log(downloadUrl, savePath);
  await invoke("download_file_to_path", {
    url: downloadUrl,
    path: savePath
  });

  console.log('Download complete');
}



// let progressListener;
interface FileDownloadProgress {
  downloaded: number;
  total_size: number;
}

onMounted(() => {
  listen('file_download_progress', (event) => {
    const payload = event.payload as FileDownloadProgress;
    progressRatio.value = Math.round(payload.downloaded / payload.total_size * 100);
  });
});

// onUnmounted(() => {
//   if (progressListener) {
//     progressListener.un
//     event.unlisten(progressListener);
//   }
// });

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <h2>Game Image</h2>
<!--  <button @click="downloadAndInstallGame()">Install</button>-->
<!--  <button @click="installGame()">Update</button>-->
  <button @click="downloadFile()">downloadFile</button>

  <ProgressBar :value="progressRatio"></ProgressBar>
</template>
