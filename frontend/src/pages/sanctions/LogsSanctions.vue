<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getSanctionsLogs } from "./SanctionsService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import Datatable from "@/components/custom/sanctions/sanctions-logs/datatable.vue";
import type { components } from "@/types";

type definitions = components["schemas"];
type GetSanctionsLogs = definitions["GetSanctionsLogs"];

const sanctionsLogs$ = ref<Promise<GetSanctionsLogs[]>>();

async function get() {
  getSanctionsLogs()
    .then((data) => {
      sanctionsLogs$.value = Promise.resolve(data);
    })
    .catch((error) => {
      sanctionsLogs$.value = Promise.reject(error);
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
  <Loader v-if="!sanctionsLogs$" :size="36" />
  <Promised :promise="sanctionsLogs$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="sanctionsLogs">
      <Datatable :data="sanctionsLogs" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
