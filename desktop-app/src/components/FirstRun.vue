<script setup lang="ts">
import {Store} from "tauri-plugin-store-api";
import {open} from "@tauri-apps/api/dialog";

const store = new Store(".settings.dat");

async function getSettings() {
  const val = await store.get("storage_folder");
  console.log(val);
}

const selectFolder = async () => {
  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
    });
    await store.set("storage_folder", {value: selectedPath});
    await store.save();
    console.log(selectedPath);
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
