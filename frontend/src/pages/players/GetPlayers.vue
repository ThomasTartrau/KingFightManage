<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getSanctions } from "../sanctions/SanctionsService";
import { getOnlinesPlayers } from "./PlayersService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import Datatable from "@/components/custom/players/datatable.vue";
import type { components } from "@/types";

type definitions = components["schemas"];
type Player = definitions["Player"];
type Sanction = definitions["Sanction"];

export interface promise$ {
  players$: Player[];
  sanctions$: Sanction[];
}

const playersPromise$ = ref<Promise<promise$>>();

async function get() {
  const players = await getOnlinesPlayers().catch((problem) => {
    playersPromise$.value = Promise.reject(problem);
  });

  const sanctions = await getSanctions().catch((problem) => {
    playersPromise$.value = Promise.reject(problem);
  });

  if (players && sanctions) {
    playersPromise$.value = Promise.resolve({
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
  <Loader v-if="!playersPromise$" :size="36" />
  <Promised :promise="playersPromise$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="playersPromise">
      <Datatable :data="playersPromise" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
