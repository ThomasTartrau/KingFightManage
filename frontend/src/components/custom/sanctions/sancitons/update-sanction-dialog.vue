<script setup lang="ts">
import { z } from "zod";
import { updateSanction } from "./SanctionsService";
import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import type { UUID } from "@/http";
import type { components } from "@/types";
import { AutoForm } from "@/components/ui/auto-form";

type definitions = components["schemas"];
type UpdateSanctionPost = definitions["UpdateSanctionPost"];

const props = defineProps<{
  sanctionId: UUID;
  type: string;
  name: string;
  duration: number;
}>();

const emit = defineEmits([
  "closeUpdateSancitonDialog",
  "closeAndRefreshSanctionDialog",
]);

enum SanctionType {
  MUTE = "mute",
  KICK = "kick",
  BAN = "ban",
}

const schema = z.object({
  type_: z
    .nativeEnum(SanctionType)
    .describe("Type")
    .default(props.type as SanctionType | SanctionType.MUTE),
  name: z
    .string({ required_error: "Le nom est requis" })
    .min(1, "Le nom est requis")
    .default(props.name),
  duration: z
    .number({
      invalid_type_error: "La durée doit être un nombre",
      required_error: "La durée est requise",
    })
    .min(1, "La durée doit être supérieure à 0")
    .default(props.duration),
});

function onSubmit(values: Record<string, any>) {
  updateSanction(props.sanctionId, values as UpdateSanctionPost).then(() => {
    emit("closeAndRefreshSanctionDialog");
  });
}
</script>

<template>
  <DialogContent class="sm:max-w-[425px]">
    <DialogHeader>
      <DialogTitle>Modifier la sanction</DialogTitle>
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
        <Button variant="secondary" @click="emit('closeUpdateSancitonDialog')">
          Fermer
        </Button>
        <Button class="mt-2 sm:mt-0" type="submit"> Envoyer </Button>
      </DialogFooter>
    </AutoForm>
  </DialogContent>
</template>
