<script setup lang="ts">
import { Check, X } from "lucide-vue-next";

const props = defineProps<{
  isOnline: boolean;
}>();
</script>

<template>
  <div>
    <Check color="green" v-if="props.isOnline" />
    <X color="red" v-else />
  </div>
</template>
