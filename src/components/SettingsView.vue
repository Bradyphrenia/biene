<template>
  <div class="p-6 lg:flex-1">
    <div class="content card">
      <div class="card-header-title">
        Database Settings
      </div>
      <div class="card-content">

        <c-configuration label="Database URL">
          <c-input
            class="input"
            type="text"
            placeholder="database url"
            v-model="url"
            @blur="saveSettings" />
        </c-configuration>
        <c-configuration label="Database Port">
          <c-input
            class="input"
            type="text"
            placeholder="database port"
            v-model="port"
            @blur="saveSettings" />
        </c-configuration>
        <c-configuration label="Database">
          <c-input
            class="input"
            type="text"
            placeholder="database"
            v-model="database"
            @blur="saveSettings" />
        </c-configuration>
        <c-configuration label="Database User">
          <c-input
            class="input"
            type="text"
            placeholder="database user"
            v-model="user"
            @blur="saveSettings" />
        </c-configuration>
        <c-configuration label="Database Password">
          <c-input
            class="input"
            type="password"
            placeholder="database password"
            v-model="password"
            @blur="saveSettings" />
        </c-configuration>

      </div>
    </div>
  </div>
</template>

<script setup lan="ts">
import { ref } from "vue";
import { getTauriStore, setTauriStore } from "../store/store.ts";

const dbSettings = ref({})
dbSettings.value = await getTauriStore("test");

const url = ref(dbSettings.value?.url || '');
const port = ref(dbSettings.value?.port || '');
const database = ref(dbSettings.value?.database || '');
const user = ref(dbSettings.value?.user || '');
const password = ref(dbSettings.value?.password || '');

async function saveSettings () {
  const settings = {
    url: url.value,
    port: port.value,
    database: database.value,
    user: user.value,
    password: password.value
  };
  await setTauriStore("test", settings);
}
</script>
