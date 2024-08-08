<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getOnlinesPlayers } from "./PlayersService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import Datatable from "@/components/custom/players/datatable.vue";
import type { components } from "@/types";
import { getSanctions } from "../sanctions/SanctionsService";

type definitions = components["schemas"];
type Player = definitions["Player"];
type Sanction = definitions["Sanction"];

export interface promise$ {
  players$: Player[];
  sanctions$: Sanction[];
}

const promise$ = ref<Promise<promise$>>();

async function get() {
  const players = await getOnlinesPlayers().catch((problem) => {
    promise$.value = Promise.reject(problem);
  });

  const sanctions = await getSanctions().catch((problem) => {
    promise$.value = Promise.reject(problem);
  });

  if (players && sanctions) {
    promise$.value = Promise.resolve({
      players$: players,
      sanctions$: sanctions,
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
  <Loader v-if="!promise$" :size="36" />
  <Promised :promise="promise$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="promise">
      <Datatable :data="promise" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
