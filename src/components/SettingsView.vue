<template>
  <div class="block">
    <div class="content">
      <!-- <h2>General Settings</h2> -->
      <h2>Database</h2>
      <div class="level">
        <div class="level-item">
          <div class="label">
            Database url
          </div>
        </div>
        <div class="level-item">
          <div class="control">
            <input class="input" type="text" placeholder="database url" :value="dbSettings.url">
          </div>
        </div>
      </div>

      <div class="level">
        <div class="level-item">
          <div class="label">
            Database port
          </div>
        </div>
        <div class="level-item">
          <div class="control">
          <input class="input" type="text" placeholder="database port" :value="dbSettings.port">
          </div>
        </div>
      </div>

      <div class="level">
        <div class="level-item">
          <div class="label">
            Database user
          </div>
        </div>
        <div class="level-item">
          <div class="control">
          <input class="input" type="text" placeholder="database user" :value="dbSettings.user">
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lan="ts">
// import { Store } from "tauri-plugin-store-api";
import { ref } from "vue";
import { initStore } from "../store/store.ts";

const dbSettings = ref({})

async function loadDbSettings () {
  const store = await initStore()
  dbSettings.value = await store.get("database")
}

await loadDbSettings()
</script>
