<template>
  <div class="block">
    {{ value2 }}
    Settings are read from a settings file :)
    <div class="content">
      <h2>General Settings</h2>
      <div class="level">
        <div class="level-item">
          <div class="label">
            Path to settings file
          </div>
        </div>
        <div class="level-item">
          <div class="control">
            <input class="input" type="text" placeholder="Text input">
          </div>
        </div>
      </div>
      <h2>Database</h2>
        <div class="columns">
          <div class="column">
            test
          </div>
          <div class="column">
            sflkjh
          </div>
        </div>
    </div>
    Settings
  </div>
  <div class="block">
    Database
  </div>
</template>

<script setup lan="ts">
// import { Store } from "tauri-plugin-store-api";
import { ref } from "vue";
import { initStore } from "../store/store.ts";
const value2 = ref(0)

// const store = ref(new Store(".settings.dat"));
// // console.log('HERE')
// console.log(store.value);

const test = await initStore()
console.log(await test.get("some-key"))
console.log(await test.get("a"))
// async function testSet () {
//   store.value.set("some-key", { value: 0 });
// }
//
// await testSet();
//
// const newVal = await store.get("some-key");
// console.log(newVal)
//
// store.set("some-key", { value: newVal.value + 1 });
// const val = await store.get("some-key");
// // assert(val, { value: 5 });
// console.log(val);
// async () => await store.save();
// test()
// console.log(store)
// async function test () {
//   const tt = await store.get("some-key");
//   console.log(tt);
// }
//
</script>
