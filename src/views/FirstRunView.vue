<script setup lang="ts">
import {open} from "@tauri-apps/api/dialog";
import SettingService from "../services/SettingService.ts";
import router from '../router';

const SettingRepository = new SettingService();

async function getSettings() {
  const val = await SettingRepository.get("storage_folder");
  console.log(val);
}

const selectFolder = async () => {
  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
    });
    await SettingRepository.set("storage_folder", selectedPath);
    await router.push({name: 'home'});
  } catch (error) {
    console.error(error);
  }
}
</script>

<template>
  <div>
    <h1>First Run</h1>
    <button @click="getSettings">Get Settings</button>
    <button @click="selectFolder">Select Folder</button>
  </div>
</template>
