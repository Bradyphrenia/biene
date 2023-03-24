<template>
  <!-- TODO: highlight active tab -->
  <div class="tabs">
    <ul>
      <li><router-link to="/test">Greet</router-link></li>
      <li><router-link to="/review">Review</router-link></li>
      <li><router-link to="/settings">Settings</router-link></li>
    </ul>
  </div>
  <Suspense>
    <router-view/>
  </Suspense>
</template>

<script setup lang="ts">
</script>
