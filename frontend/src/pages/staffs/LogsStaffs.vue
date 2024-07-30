<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Logs, getLogs } from "./StaffsService";
import Loader from "@/components/custom/loader.vue";
import LogsDatatable from "@/components/custom/staffs/logs-datatable.vue";
import { displayProblem } from "@/http";

const logs$ = ref<Promise<Logs>>();

async function get() {
  logs$.value = getLogs().catch((problem) => {
    displayProblem(problem);
    return [];
  });
}

async function _onLoad() {
  await get();
}

onMounted(async () => {
  await _onLoad();
});
</script>

<template>
  <Loader v-if="!logs$" :size="36" />
  <Promised :promise="logs$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="logs">
      <LogsDatatable :data="logs" @refresh-datatable="_onLoad" />
    </template>
  </Promised>
</template>
