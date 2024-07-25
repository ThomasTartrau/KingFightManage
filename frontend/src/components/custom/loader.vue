<script setup lang="ts">
import { LoaderCircle } from 'lucide-vue-next'

const props = defineProps<{
  size?: number
}>()
</script>

<template>
  <div class="flex items-center justify-center">
    <LoaderCircle class="animate-spin" :size="props.size" />
  </div>
</template>
