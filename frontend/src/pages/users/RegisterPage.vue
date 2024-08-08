<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import { vAutoAnimate } from "@formkit/auto-animate/vue";

import { push } from "notivue";
import type { AxiosError, AxiosResponse } from "axios";
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
import { register } from "@/iam";
import type { Problem } from "@/http";
import { displayError } from "@/http";
import router from "@/router/router";

const formSchema = toTypedSchema(
  z.object({
    username: z.string().min(1, "Le nom d'utilisateur est requis"),
    email: z.string().email("L'adresse e-mail n'est pas valide"),
    password: z
      .string()
      .min(12, "Votre mot de passe doit contenir au moins 12 caractères"),
    registration_token: z.string().min(1, "Le token d'inscription est requis"),
  }),
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: {
  username: string;
  email: string;
  password: string;
  registration_token: string;
}) {
  await register(
    values.email,
    values.username,
    values.password,
    values.registration_token,
  )
    .then(() => {
      push.success({
        title: "Inscription réussie",
        message:
          "Vous êtes inscrit avec succès. Veuillez vérifier votre e-mail pour activer votre compte.",
        duration: 5000,
      });
      return router.push({ name: routes.Login });
    })
    .catch((err: AxiosError<AxiosResponse<Problem>>) => {
      displayError(err);
    });
}
</script>

<template>
  <form
    class="flex items-center justify-center min-h-screen"
    @submit="onSubmit"
  >
    <Card class="mx-auto max-w-sm">
      <CardHeader>
        <CardTitle class="text-2xl"> Inscription </CardTitle>
        <CardDescription> Inscrivez-vous pour créer un compte </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="grid gap-4">
          <FormField v-slot="{ componentField }" name="username">
            <FormItem v-auto-animate>
              <FormLabel>Nom d'utilisateur</FormLabel>
              <FormControl>
                <Input type="text" placeholder="John" v-bind="componentField" />
              </FormControl>
              <FormMessage />
            </FormItem>
          </FormField>
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
              <FormLabel>Mot de passe</FormLabel>
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
          <FormField v-slot="{ componentField }" name="registration_token">
            <FormItem v-auto-animate>
              <FormLabel>Token d'inscription</FormLabel>
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
          <Button type="submit" class="w-full"> S'inscrire </Button>
        </div>
        <div class="mt-4 text-center text-sm">
          Vous avez déjà un compte ?
          <router-link :to="{ name: routes.Login }" class="underline">
            Connectez-vous
          </router-link>
        </div>
      </CardContent>
    </Card>
  </form>
</template>
