<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { Logs } from './BoutiqueService'
import { getLogs } from './BoutiqueService'
import Loader from '@/components/custom/loader.vue'
import LogsDatatable from '@/components/custom/boutique/logs-datatable.vue'
import PromisedError from '@/components/custom/promised-error.vue'

const logs$ = ref<Promise<Logs>>()

async function get() {
  getLogs()
    .then((logs) => {
      logs$.value = Promise.resolve(logs)
    })
    .catch((problem) => {
      logs$.value = Promise.reject(problem)
    })
}

async function _onLoad() {
  await get()
}

onMounted(async () => {
  await _onLoad()
})
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
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
>
