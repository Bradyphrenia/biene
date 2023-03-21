<template>
  <div class="tabs">
    <ul>
      <li><router-link to="/test">Greet</router-link></li>
      <li><router-link to="/test2">Review</router-link></li>
    </ul>
  </div>
  <router-view/>
</template>

<script setup lang="ts">
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
