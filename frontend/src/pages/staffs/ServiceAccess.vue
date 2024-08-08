<script setup lang="ts">
import { onMounted, ref } from "vue";
import type { ServicesAccess } from "./StaffsService";
import { getServiceAccess } from "./StaffsService";
import Loader from "@/components/custom/loader.vue";
import PromisedError from "@/components/custom/promised-error.vue";
import ServiceAccessDatatable from "@/components/custom/staffs/service-access-datatable.vue";

const serviceAccess$ = ref<Promise<ServicesAccess>>();

async function get() {
  getServiceAccess()
    .then((serviceAccess) => {
      serviceAccess$.value = Promise.resolve(serviceAccess);
    })
    .catch((problem) => {
      serviceAccess$.value = Promise.reject(problem);
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
  <Loader v-if="!serviceAccess$" :size="36" />
  <Promised :promise="serviceAccess$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="serviceAccess">
      <ServiceAccessDatatable
        :data="serviceAccess"
        @refresh-datatable="_onLoad"
      />
    </template>
    <template #rejected="error">
      <PromisedError :content="error" @reload="_onLoad" />
    </template>
  </Promised>
</template>
