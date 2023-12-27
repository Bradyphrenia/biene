import { defineStore } from "pinia";
import { ref } from "vue";
import { getTauriStore, setTauriStore } from "./store";

export const useDatabaseStore = defineStore('database', () => {
  console.log(getTauriStore('database'))
  // const settings = ref(computed(async () => await getTauriStore('database')));
  const settings = ref(0)
  async function saveStore (data: Object) {
    await setTauriStore('database', {
      ...getTauriStore('database'),
      ...data,
    });
  };

  return {
    settings: Number,
    saveStore: Function,
  }
})
