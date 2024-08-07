<script setup lang="ts">
import { z } from "zod";
import { onMounted, ref } from "vue";
import { push } from "notivue";
import { banPlayer, kickPlayer, mutePlayer } from "./PlayersService";
import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { AutoForm } from "@/components/ui/auto-form";
import type { UUID } from "@/http";
import type { components } from "@/types";
import type { UserInfo } from "@/iam";
import { getUserInfo } from "@/iam";
import DialogDescription from "@/components/ui/dialog/DialogDescription.vue";

const props = defineProps<{
  playerId: UUID;
  playerName: string;
  sanctions: Sanction[];
  sanctionType: string;
}>();

const emit = defineEmits(["closeSanctionDialog"]);

type definitions = components["schemas"];
export type Sanction = definitions["Sanction"];

const user_info = ref<UserInfo | null>(null);

const schema = z.object({
  sanction_name: z.enum(["AUCUNE"] as [string, ...string[]]),
  motif: z
    .string({ required_error: "Le motif est requis" })
    .min(1, "Le motif est requis")
    .max(1000, "Le motif ne doit pas dépasser 1000 caractères"),
});

async function _onLoad() {
  user_info.value = getUserInfo().value;
  const newSanctions = [];
  for (const sanction of props.sanctions) {
    if (sanction.type_ === props.sanctionType) {
      newSanctions.push(sanction.name);
    }
  }

  if (newSanctions.length >= 1) {
    schema.shape.sanction_name = z.enum(newSanctions as [string, ...string[]]);
  }
}

function onSubmit(values: Record<string, any>) {
  if (values.sanction_name && user_info.value) {
    const sanction = props.sanctions.find(
      (sanction) => sanction.name === values.sanction_name,
    );

    if (!sanction) {
      push.error({
        title: "Erreur",
        message: "Sanction inconnue",
      });
      emit("closeSanctionDialog");
      return;
    }

    switch (props.sanctionType) {
      case "mute":
        mutePlayer(
          props.playerId,
          user_info.value?.username || "Undefined",
          sanction.sanction_id,
          values.motif,
          true,
        );
        break;
      case "kick":
        kickPlayer(
          props.playerId,
          user_info.value?.username || "Undefined",
          sanction.sanction_id,
          values.motif,
          true,
        );
        break;
      case "ban":
        banPlayer(
          props.playerId,
          user_info.value?.username || "Undefined",
          sanction.sanction_id,
          values.motif,
          true,
        );
        break;
      default:
        push.error({
          title: "Erreur",
          message: "Type de sanction inconnu",
        });
        break;
    }

    emit("closeSanctionDialog");
  } else {
    push.error({
      title: "Erreur",
      message: "Veuillez remplir tous les champs",
    });
  }
}

onMounted(_onLoad);
</script>

<template>
  <DialogContent class="sm:max-w-[425px]">
    <DialogHeader>
      <DialogTitle>Attribuer une sanction</DialogTitle>
      <DialogDescription>
        Sanctionner <span class="font-bold">{{ props.playerName }}</span>
      </DialogDescription>
    </DialogHeader>
    <AutoForm
      :schema="schema"
      :field-config="{
        motif: {
          label: 'Motif',
          inputProps: {
            placeholder: 'A spamer le chat (discord.gg/kingfight)',
          },
        },
      }"
      class="space-y-6"
      @submit="onSubmit"
    >
      <DialogFooter class="flex flex-col sm:flex-row">
        <Button variant="secondary" @click="emit('closeSanctionDialog')">
          Fermer
        </Button>
        <Button type="submit" class="mt-2 sm:mt-0"> Envoyer </Button>
      </DialogFooter>
    </AutoForm>
  </DialogContent>
</template>
