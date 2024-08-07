<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getOnlinesPlayers } from "./PlayersService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import Datatable from "@/components/custom/players/datatable.vue";
import type { components } from "@/types";

type definitions = components["schemas"];
type Player = definitions["Player"];

const players$ = ref<Promise<Player[]>>();

onMounted(_onLoad);

async function get() {
  getOnlinesPlayers()
    .then((data) => {
      players$.value = Promise.resolve(data);
    })
    .catch((error) => {
      players$.value = Promise.reject(error);
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
  <Loader v-if="!players$" :size="36" />
  <Promised :promise="players$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="players">
      <Datatable :data="players" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
