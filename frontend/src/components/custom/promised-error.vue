<script setup lang="ts">
import Code from "./code.vue";
import CustomButton from "@/components/custom-button.vue";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type ProblemFactory from "@/utils/problemFactory";

const props = defineProps<{
  onReload: () => void;
  content: ProblemFactory;
}>();

const error = JSON.stringify(props.content, null, 2);
</script>

<template>
  <div
    class="flex flex-col items-center justify-center"
    style="min-height: calc(100vh - 3.5rem)"
  >
    <Card class="shadow-2xl shadow-slate-900">
      <CardHeader>
        <CardTitle> Une erreur est surevenue </CardTitle>

        <CardDescription>
          Si vous souhaitez accéder à la ressource demandée, vous pouvez cliquer
          sur le bouton ci-dessous pour recharger la page.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Code :content="error" />
      </CardContent>
      <CardFooter class="flex justify-end">
        <CustomButton variant="ringHover" size="lg" @click="props.onReload">
          Recharger
        </CustomButton>
      </CardFooter>
    </Card>
  </div>
</template>
