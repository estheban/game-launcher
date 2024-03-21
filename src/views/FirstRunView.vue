<script setup lang="ts">
import {open} from "@tauri-apps/api/dialog";
import SettingService from "../services/SettingService.ts";
import router from '../router';
import Button from 'primevue/button';
import Message from 'primevue/message';

const SettingRepository = new SettingService();

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
    <h1>Game Launcher</h1>
    <Message severity="warn" :closable="false">The install folder is not configured. Please select a folder to install games.</Message>
    <Button @click="selectFolder" label="Select Folder" />
  </div>
</template>
