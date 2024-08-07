<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { PbLogs } from './BoutiqueService'
import { getPbLogs } from './BoutiqueService'
import Loader from '@/components/custom/loader.vue'
import LogsPbDatatable from '@/components/custom/boutique/logs-pb-datatable.vue'
import PromisedError from '@/components/custom/promised-error.vue'

const logs$ = ref<Promise<PbLogs>>()

async function get() {
  getPbLogs()
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
      <LogsPbDatatable :data="logs" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
>
