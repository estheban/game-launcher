<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { RouterView } from "vue-router";
import Toast from 'primevue/toast';
</script>

<template>
  <Toast />
  <div class="container">
    <RouterView />
  </div>
</template>

<style scoped>
div.container {
  padding-top: 0px;
}
</style>
