<script setup lang="ts">
import { Check, X } from 'lucide-vue-next'

const props = defineProps<{
  isOnline: boolean
}>()
</script>

<template>
  <div>
    <Check v-if="props.isOnline" color="green" />
    <X v-else color="red" />
  </div>
</template>
