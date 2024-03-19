<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import GameDetails from "./components/GameDetails.vue";
import FirstRun from "./components/FirstRun.vue";
</script>

<template>
  <div class="container">
    <h1>Game Launcher</h1>

    <FirstRun />

    <GameDetails />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
