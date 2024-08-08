<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getErrors } from "./ErrorsService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import Datatable from "@/components/custom/players/datatable.vue";
import type { components } from "@/types";

type definitions = components["schemas"];
type Error = definitions["Error"];

const errors$ = ref<Promise<Error[]>>();

async function get() {
  getErrors()
    .then((response) => {
      errors$.value = Promise.resolve(response);
    })
    .catch((error) => {
      errors$.value = Promise.reject(error);
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
  <Loader v-if="!errors$" :size="36" />
  <Promised :promise="errors$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="errors">
      <Datatable :data="errors" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
