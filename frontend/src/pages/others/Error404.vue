<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { routes } from "@/router/routes";
import router from "@/router/router";

function back_to_home() {
  router.push({ name: routes.Home });
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen">
    <Card class="mx-auto max-w-sm">
      <CardHeader>
        <CardTitle>Erreur 404</CardTitle>
        <CardDescription>
          La page que vous cherchez n'existe pas.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Separator class="my-4" />
        <p>Oops! Je pense que vous êtes perdu.</p>
        <Button class="mt-4" @click="back_to_home">
          Revenir en lieu sûr
        </Button>
      </CardContent>
    </Card>
  </div>
</template>
