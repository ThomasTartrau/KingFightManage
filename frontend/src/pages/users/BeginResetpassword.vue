<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import { push } from "notivue";
import { beginResetPassword } from "./UserServices";
import Button from "@/components/ui/button/Button.vue";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { displayError } from "@/http";
import router from "@/router/router";

const formSchema = toTypedSchema(
  z.object({
    email: z.string().email("Email invalide"),
  }),
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

async function submit(values: { email: string }) {
  await beginResetPassword(values.email)
    .then(() => {
      push.success({
        title: "Réinitialisation du mot de passe",
        message:
          "Un email vous a été envoyé avec les instructions pour réinitialiser votre mot de passe",
        duration: 5000,
      });
    })
    .catch(displayError);
}
</script>

<template>
  <div class="flex items-center justify-center min-h-screen">
    <Card class="mx-auto">
      <CardHeader>
        <CardTitle>Réinitialisation du mot de passe</CardTitle>
        <CardDescription>
          Veuillez entrer votre email pour réinitialiser votre mot de passe
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit">
          <FormField v-slot="{ componentField }" name="email">
            <FormItem v-auto-animate>
              <FormLabel>Email</FormLabel>
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
          <div class="flex justify-end mt-4 space-x-4">
            <Button type="button" variant="secondary" @click="router.back">
              Retour
            </Button>
            <Button type="submit"> Réinitialiser le mot de passe </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
