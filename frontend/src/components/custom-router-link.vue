<script setup lang="ts">
import { useRoute } from "@/router/routes";

const props = defineProps({
  route: {
    type: String,
    required: true,
  },
});

const route = useRoute(props.route);
</script>

<template>
  <router-link :to="{ name: route }">
    <slot />
  </router-link>
</template>
