import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";

export const useReviewStore = defineStore('review', () => {
  const val = async () => await get_review_table()


  async function get_review_table() : Promise<Durchsicht[]> {
      return await invoke<Durchsicht[]>('request_review')
  }
  return { get_review_table, val }
})


interface Durchsicht {
  id: number,
  volk: string,
  koenigin: boolean,
  stifte: boolean,
  offene: boolean,
  verdeckelte: boolean,
  weiselzelle: number,
  spielnaepfe: boolean,
  sanftmut: boolean,
  volksstaerke: number,
  anz_brutwaben: number,
};
