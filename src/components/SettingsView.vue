<template>
  <div class="p-6 lg:flex-1">
    <div class="content card">
      <div class="card-header-title">
        Database Settings <c-icon :icon="icons.databaseOutline"/>
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
        <c-configuration label="Test Database">
          <div class="is-horizontal">
            <c-button
              label="test"
              @click="testDbConnection"/>
              <div class="has-test-danger">
                {{ connectionTestResponse.msg }}
              </div>
          </div>
        </c-configuration>
      </div>
    </div>
            <c-button
            label="ttt"
            @click="test"/>
  </div>
</template>

<script setup lan="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { getTauriStore, setTauriStore } from "../store/store.ts";
import { useDatabaseStore } from "../store/database.ts";

const connectionTestResponse = ref('')
function testDbConnection () {
  invoke('test_db_connection')
    .then(response => {
      connectionTestResponse.value = response
  })
}

async function test () {
    // console.log(await invoke('connect_to_db'))
    connectionTestResponse.value = await invoke('connect_to_db')
}

const databaseStore = useDatabaseStore()
console.log(databaseStore)

const dbSettings = ref({})
dbSettings.value = await getTauriStore("database");

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
  await setTauriStore("database", settings);
}
</script>
