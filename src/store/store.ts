import { Store } from "tauri-plugin-store-api";

export async function initStore () {
  const store = new Store(".settings.dat");
  store.set("database", {
    url: "database url",
    port: "database port",
    user: "database user",
  });
  return store;
}
