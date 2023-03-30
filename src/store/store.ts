import { Store } from "tauri-plugin-store-api";

async function initEmptyStore (storeKey: string) {
  const tauriStore = new Store(".settings");
  if (!tauriStore.has(storeKey)) {
    await tauriStore.set(storeKey, {});
    await tauriStore.save();
  }
}

// function initStore () {
//     const ts = new Store(".settings.dat");
//     ts.set("database", {
//
//             url: "127.0.0.1",
//             port: "5432",
//             database: "biene",
//             user: "postgres",
//             password: "postgres"
//     })
//     ts.save();
// }

export async function getTauriStore (storeKey: string) {
  const tauriStore = new Store(".settings.dat");
  if (!tauriStore.has(storeKey)) initEmptyStore(storeKey);

  return await tauriStore.get(storeKey);
}

export async function setTauriStore (
  storeKey: string,
  data: object
) {
  console.log(data)
  const tauriStore = new Store(".settings.dat");
  await tauriStore.set(storeKey, data);
  await tauriStore.save();
}
