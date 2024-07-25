<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import { UUID } from "@/http";
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import { sendMessage } from "./StaffsService";

const props = defineProps<{
  user_id: UUID;
  username: string;
}>();

const emit = defineEmits(["closeMessageModal"]);

const formSchema = toTypedSchema(
  z.object({
    message: z
      .string({ message: "Ce champ est requis." })
      .min(1, {
        message: "Le message doit ne peut pas être nul.",
      })
      .max(1000, {
        message: "Le message ne peut pas dépasser 1000 caractères.",
      }),
  })
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit((values) => {
  submit(values);
});

function submit(values: { message: string }) {
  sendMessage(props.user_id, values.message);
  emit("closeMessageModal");
}
</script>

<template>
  <DialogContent class="sm:max-w-[425px]">
    <DialogHeader>
      <DialogTitle>Envoyer un message</DialogTitle>
      <DialogDescription>
        Envoyez un message privé à
        <span class="font-bold">{{ username }}</span>
      </DialogDescription>
    </DialogHeader>
    <form @submit.prevent="onSubmit">
      <div class="grid gap-4 py-4">
        <FormField v-slot="{ componentField }" name="message">
          <FormItem>
            <FormLabel>Message</FormLabel>
            <FormControl>
              <Textarea
                placeholder="Votre message..."
                v-bind="componentField"
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        </FormField>
      </div>
      <DialogFooter>
        <Button variant="secondary" @click="emit('closeMessageModal')">
          Fermer
        </Button>
        <Button type="submit"> Envoyer </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</template>
