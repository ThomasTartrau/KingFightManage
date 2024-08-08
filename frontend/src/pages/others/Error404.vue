<script setup lang="ts">
import Code from "@/components/custom/code.vue";
import CustomButton from "@/components/custom-button.vue";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Problem } from "@/http";
import router from "@/router/router";
import { routes } from "@/router/routes";

const error: Problem = {
  id: "Error404",
  title: "Page not found",
  status: 404,
  detail:
    "An error occurred while processing the request. The requested resource was not found.",
};
const jsonError = JSON.stringify(error, null, 2);
</script>

<template>
  <div
    class="flex flex-col items-center justify-center"
    style="min-height: calc(100vh - 3.5rem)"
  >
    <Card class="shadow-2xl shadow-slate-900">
      <CardHeader>
        <CardTitle> Vous vous êtes perdu ? </CardTitle>

        <CardDescription>
          Il semble que vous ayez atterri sur une page qui n'existe pas. Ne vous
          inquiétez pas, je suis là pour vous aider.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Code :content="jsonError" />
      </CardContent>
      <CardFooter class="flex justify-end">
        <CustomButton
          variant="ringHover"
          size="lg"
          @click="router.push({ name: routes.Home })"
        >
          Retourner en lieu sûr
        </CustomButton>
      </CardFooter>
    </Card>
  </div>
</template>
