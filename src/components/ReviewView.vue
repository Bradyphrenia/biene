<template>
  <div v-if="!hasDatabase" class="p-6 lg:flex-1">
    No database was found. Please check if you have the right settings and reload the application.
  </div>
  <div v-if="hasDatabase" class="p-6 lg:flex-1">
    <table class="table is-narrow is-fullwidth is-hoverable is-striped">
      <thead>
        <tr>
          <th>Datum</th>
          <th>Volk</th>
          <th>Königin</th>
          <th>Stifte</th>
          <th>Offene</th>
          <th>Verdeckelte</th>
          <th>Weiselzelle</th>
          <th>Spielnäpfe</th>
          <th>Sanftmut</th>
          <th>Volksstärke</th>
          <th># Brutwaben</th>
          <th>Memo</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="elem in current_review_table" :key="elem.id">
	  <td></td>
	  <td>{{ elem.volk }}</td>
	  <td>{{ elem.koenigin }}</td>
	  <td>{{ elem.stifte }}</td>
	  <td>{{ elem.offene }}</td>
	  <td>{{ elem.verdeckelte }}</td>
	  <td>{{ elem.weiselzelle }}</td>
	  <td>{{ elem.spielnaepfe }}</td>
	  <td>{{ elem.sanftmut }}</td>
	  <td>{{ elem.volksstaerke }}</td>
	  <td>{{ elem.anz_brutwaben }}</td>
	  <td>{{ elem }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useReviewStore } from "../store/review";

// TODO: move interface to separate file and export
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

const hasDatabase = ref(true);

const current_review_table = ref<Durchsicht[]>([])
const reviewStore = useReviewStore();
try {
  current_review_table.value = await reviewStore.get_review_table();
} catch (err) {
  hasDatabase.value = false;
  console.error(err)
}
</script>
