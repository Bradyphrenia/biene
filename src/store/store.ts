import { Store } from "tauri-plugin-store-api";

export async function initStore () {
  const store = new Store(".settings.dat");
  store.set("some-key", { value: 123 });
  return store;
}
