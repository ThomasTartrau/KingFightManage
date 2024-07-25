<script setup lang="ts">
import { vAutoAnimate } from "@formkit/auto-animate/vue";

import { push } from "notivue";
import type { AxiosError, AxiosResponse } from "axios";
import { useRouter } from "vue-router";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { routes } from "@/router/routes";
import { login } from "@/iam";
import type { Problem } from "@/http";
import { displayError } from "@/http";

const router = useRouter();

const formSchema = toTypedSchema(
  z.object({
    email: z.string().email("L'adresse e-mail n'est pas valide"),
    password: z.string().min(1, "Le mot de passe est requis"),
  })
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: { email: string; password: string }) {
  await login(values.email, values.password)
    .then(() => {
      push.success({
        title: "Connexion réussie",
        message: "Vous êtes connecté avec succès",
        duration: 5000,
      });
      return router.push({ name: routes.Home });
    })
    .catch((err: AxiosError<AxiosResponse<Problem>>) => {
      displayError(err);
    });
}
</script>

<template>
  <form
    class="flex items-center justify-center min-h-screen"
    @submit.prevent="onSubmit"
  >
    <Card class="mx-auto max-w-sm">
      <CardHeader>
        <CardTitle class="text-2xl"> Connexion </CardTitle>
        <CardDescription> Connectez-vous pour continuer </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid gap-4">
          <FormField v-slot="{ componentField }" name="email">
            <FormItem v-auto-animate>
              <FormLabel>Adresse e-mail</FormLabel>
              <FormControl>
                <Input
                  type="email"
                  placeholder="johndoe@example.com"
                  v-bind="componentField"
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          </FormField>
          <FormField v-slot="{ componentField }" name="password">
            <FormItem v-auto-animate>
              <FormLabel>
                <div class="text-left">Mot de passe</div>
                <div class="text-right text-xs">
                  <router-link
                    :to="{ name: routes.BeginResetpassword }"
                    class="underline"
                  >
                    Mot de passe oublié ?
                  </router-link>
                </div>
              </FormLabel>
              <FormControl>
                <Input
                  type="password"
                  placeholder="********"
                  v-bind="componentField"
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          </FormField>
          <Button type="submit" class="w-full"> Se connecter </Button>
          <!-- <Button variant="outline" class="w-full">
            Login with Google
          </Button> -->
        </div>
        <div class="mt-4 text-center text-sm">
          Vous n'avez pas de compte ?
          <router-link :to="{ name: routes.Register }" class="underline">
            S'inscrire
          </router-link>
        </div>
      </CardContent>
    </Card>
  </form>
</template>
