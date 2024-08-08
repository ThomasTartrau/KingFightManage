<script setup lang="ts">
import { push } from "notivue";
import { onMounted, onUpdated, ref } from "vue";
import { Upload } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { UserInfo } from "@/iam";
import { getUserInfo, refresh } from "@/iam";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import profilePictureUpload from "@/components/custom/profile-picture-upload.vue";
import http, { displayError } from "@/http";

const isProfileDialogOpen = ref<boolean>(false);
const closeProfileDialog = () => (isProfileDialogOpen.value = false);

const isNameDialogOpen = ref<boolean>(false);
const closeNameDialog = () => (isNameDialogOpen.value = false);

const user_info = ref<UserInfo | null>(null);

const username = ref<string>("");
const image_link = ref<string>("");

async function changeName() {
  if (username.value !== user_info.value?.username) {
    if (username.value.length < 2) {
      push.error({
        title: "Nom invalide",
        message: "Le nom doit contenir au moins 2 caractères",
        duration: 5000,
      });
    }

    await http
      .post("/user/profile/username", {
        username: username.value,
      })
      .then(() => {
        push.success({
          title: "Profil mis à jour",
          message: "Votre profil a été mis à jour avec succès",
          duration: 5000,
        });
        closeNameDialog();
        refresh()
          .then(() => _load())
          .catch(displayError);
      })
      .catch(displayError);
  } else {
    push.error({
      title: "Aucun changement",
      message: "Aucun changement n'a été apporté à votre profil",
      duration: 5000,
    });
  }
}

function _load() {
  user_info.value = getUserInfo().value;
  username.value = user_info.value?.username || "";
  image_link.value = `/profile-pictures/${user_info.value?.user_id}.jpeg` || "";
}

onMounted(_load);
onUpdated(_load);
</script>

<template>
  <Card class="shadow-2xl shadow-slate-900">
    <CardHeader>
      <CardTitle> Personnal information </CardTitle>
    </CardHeader>
    <CardContent>
      <div class="mb-6">
        <div class="flex flex-col">
          <Label class="mb-4">Profile picture</Label>
          <div class="flex items-center">
            <Avatar size="base" class="mb-6">
              <AvatarImage :src="image_link" />
              <AvatarFallback>
                {{ user_info?.username.charAt(0).toUpperCase() }}
                {{ user_info?.username.charAt(1).toUpperCase() }}
              </AvatarFallback>
            </Avatar>
            <Dialog v-model:open="isProfileDialogOpen">
              <DialogTrigger>
                <Button class="ml-8">
                  <Upload class="mr-2" />
                  Changer la photo de profil
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle class="text-center">
                    Charger votre photo de profil
                  </DialogTitle>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                  <profilePictureUpload
                    @close-profile-dialog="closeProfileDialog"
                  />
                </div>
              </DialogContent>
            </Dialog>
          </div>
        </div>
        <FormField name="Email">
          <FormItem>
            <FormLabel>Email</FormLabel>
            <FormControl>
              <Input
                disabled
                type="email"
                placeholder="Email"
                :model-value="user_info?.email"
              />
            </FormControl>
            <FormDescription>
              Cet email est associé à votre compte. Malheuresement, vous ne
              pouvez pas le changer.
            </FormDescription>
          </FormItem>
        </FormField>
      </div>

      <div class="mb-6">
        <FormField name="First name">
          <FormItem>
            <FormLabel>Nom d'utilisateur</FormLabel>
            <FormControl>
              <Input
                disabled
                type="text"
                placeholder="Nom d'utilisateur"
                :model-value="user_info?.username"
              />
            </FormControl>
            <FormDescription>
              C'est le nom d'utilisateur que vous avez utilisé pour vous
              inscrire.
            </FormDescription>
          </FormItem>
        </FormField>
      </div>

      <div class="flex justify-end">
        <Dialog v-model:open="isNameDialogOpen">
          <form>
            <DialogTrigger as-child>
              <Button variant="outline"> Changer le nom d'utilisateur </Button>
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
              <DialogHeader>
                <DialogTitle>Changer le nom d'utilisateur</DialogTitle>
                <DialogDescription>
                  Vous pouvez changer votre nom d'utilisateur ici. Cliquez sur
                  enregistrer lorsque vous aurez fini.
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="username" class="text-right">
                    Nom d'utilisateur
                  </Label>
                  <Input id="username" v-model="username" class="col-span-3" />
                </div>
              </div>
              <DialogFooter>
                <Button variant="secondary" @click="closeNameDialog">
                  Annuler
                </Button>
                <Button @click="changeName"> Enregistrer </Button>
              </DialogFooter>
            </DialogContent>
          </form>
        </Dialog>
      </div>
    </CardContent>
  </Card>
</template>
