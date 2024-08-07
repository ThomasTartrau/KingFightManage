<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { TriangleAlert } from 'lucide-vue-next'
import { type Sanction, getSanctions } from './SanctionsService'
import Loader from '@/components/custom/loader.vue'
import PromisedError from '@/components/custom/promised-error.vue'
import Datatable from '@/components/custom/sanctions/sancitons/datatable.vue'
import { Alert, AlertTitle } from '@/components/ui/alert'
import AlertDescription from '@/components/ui/alert/AlertDescription.vue'

const sanctions$ = ref<Promise<Sanction[]>>()

async function get() {
  getSanctions()
    .then((response) => {
      sanctions$.value = Promise.resolve(response)
    })
    .catch((error) => {
      sanctions$.value = Promise.reject(error)
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
  <Loader v-if="!sanctions$" :size="36" />
  <Promised :promise="sanctions$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="sanctions">
      <Alert class="mb-6" variant="destructive">
        <TriangleAlert class="w-4 h-4" />
        <AlertTitle>ATTENTION</AlertTitle>
        <AlertDescription>
          Ne pas supprimer une sanction sans l'accord d'un développeur cela
          engendre le suppresion de logs des sanctions en rapport avec la
          sanction supprimé.
        </AlertDescription>
      </Alert>
      <Datatable :data="sanctions" @refresh-datatable="_onLoad" />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
