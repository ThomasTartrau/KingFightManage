<script setup lang="ts">
import { onMounted, ref } from "vue";
import type { Logs, PbLogs } from "./BoutiqueService";
import { getLogs, getPbLogs } from "./BoutiqueService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import graphs from "@/components/custom/boutique/graphs.vue";
import { config } from "@/lib/config";

export interface GraphsLogs {
  boutiqueLogs: Logs;
  pbLogs: PbLogs;
  objective: number;
}

const graphsLogs$ = ref<Promise<GraphsLogs>>();

async function get() {
  const boutiqueLogs = await getLogs().catch((problem) => {
    graphsLogs$.value = Promise.reject(problem);
  });

  const pbLogs = await getPbLogs().catch((problem) => {
    graphsLogs$.value = Promise.reject(problem);
  });

  if (boutiqueLogs && pbLogs) {
    graphsLogs$.value = Promise.resolve({
      boutiqueLogs,
      pbLogs,
      objective: config.BOUTIQUE_OBJECTIVE_MONTHLY,
    });
  }
}

async function _onLoad() {
  await get();
}

onMounted(async () => {
  await _onLoad();
});
</script>

<template>
  <Loader v-if="!graphsLogs$" :size="36" />
  <Promised :promise="graphsLogs$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="logs">
      <graphs :logs="logs" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
>
