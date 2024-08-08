<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { getPlayerSanctions } from "./SanctionsService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import Datatable from "@/components/custom/sanctions/sanctions-players-logs/datatable.vue";
import type { components } from "@/types";
import type { Problem, UUID } from "@/http";

const route = useRoute();

type definitions = components["schemas"];
type GetPlayerSanctions = definitions["GetPlayerSanctions"];

const playersSanctions$ = ref<Promise<GetPlayerSanctions[]>>();

onMounted(_onLoad);

async function get(player_id: UUID) {
  getPlayerSanctions(player_id)
    .then((data) => {
      playersSanctions$.value = Promise.resolve(data);
    })
    .catch((error) => {
      playersSanctions$.value = Promise.reject(error);
    });
}

async function _onLoad() {
  if (route.params.player_id) {
    await get(route.params.player_id as UUID);
    return;
  }

  const problem: Problem = {
    id: "bad_request",
    status: 400,
    title: "Bad Request",
    detail: "Player ID is required",
  };
  playersSanctions$.value = Promise.reject(problem);
}

onMounted(async () => {
  await _onLoad();
});
</script>

<template>
  <Loader v-if="!playersSanctions$" :size="36" />
  <Promised :promise="playersSanctions$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="playersSanctions">
      <Datatable :data="playersSanctions" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
