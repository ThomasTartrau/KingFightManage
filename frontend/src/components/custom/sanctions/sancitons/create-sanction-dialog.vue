<script setup lang="ts">
import { z } from "zod";
import { createSanction } from "./SanctionsService";
import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import type { components } from "@/types";
import { AutoForm } from "@/components/ui/auto-form";

type definitions = components["schemas"];
type CreateSanctionPost = definitions["CreateSanctionPost"];

const emit = defineEmits([
  "closeCreateSancitonDialog",
  "closeAndRefreshSanctionDialog",
]);

enum SanctionType {
  MUTE = "mute",
  KICK = "kick",
  BAN = "ban",
}

const schema = z.object({
  type_: z.nativeEnum(SanctionType).describe("Type").default(SanctionType.MUTE),
  name: z
    .string({ required_error: "Le nom est requis" })
    .min(1, "Le nom est requis"),
  duration: z
    .number({
      invalid_type_error: "La durée doit être un nombre",
      required_error: "La durée est requise",
    })
    .min(1, "La durée doit être supérieure à 0"),
});

function onSubmit(values: Record<string, any>) {
  createSanction(values as CreateSanctionPost).then(() => {
    emit("closeAndRefreshSanctionDialog");
  });
}
</script>

<template>
  <DialogContent class="sm:max-w-[425px]">
    <DialogHeader>
      <DialogTitle>Créer une sanction</DialogTitle>
    </DialogHeader>
    <AutoForm
      :schema="schema"
      :field-config="{
        name: {
          label: 'Nom',
          inputProps: { placeholder: 'Spam, Insulte staff, ...' },
        },
        duration: {
          label: 'Durée (en secondes)',
          inputProps: { placeholder: '1' },
        },
      }"
      class="space-y-6"
      @submit="onSubmit"
    >
      <DialogFooter class="flex flex-col sm:flex-row">
        <Button variant="secondary" @click="emit('closeCreateSancitonDialog')">
          Fermer
        </Button>
        <Button type="submit" class="mt-2 sm:mt-0"> Envoyer </Button>
      </DialogFooter>
    </AutoForm>
  </DialogContent>
</template>
