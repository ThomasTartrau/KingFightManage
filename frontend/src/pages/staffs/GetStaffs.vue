<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { Staffs } from './StaffsService'
import { getStaffs } from './StaffsService'
import Loader from '@/components/custom/loader.vue'
import StaffsDatatable from '@/components/custom/staffs/datatable.vue'

const staffs$ = ref<Promise<Staffs>>()

async function get() {
  staffs$.value = getStaffs()
}

async function _onLoad() {
  await get()
}

onMounted(async () => {
  await _onLoad()
})
</script>

<template>
  <Loader v-if="!staffs$" :size="36" />
  <Promised :promise="staffs$">
    <template #pending>
      <Loader :size="36" />
    </template>
    <template #default="staffs">
      <StaffsDatatable :data="staffs" @refresh-datatable="_onLoad" />
    </template>
  </Promised>
</template>
