<template>
  <!-- TODO: highlight active tab -->
  <div class="tabs">
    <ul>
      <!-- <li><router-link to="/test">Greet</router-link></li> -->
      <li><router-link to="/review">Durchsicht</router-link></li>
      <li><router-link to="/settings">Einstellungen</router-link></li>
    </ul>
  </div>
  <Suspense>
    <router-view/>
  </Suspense>
</template>

<script setup lang="ts">
</script>
