<script setup lang="ts">
import { push } from "notivue";
import { deleteUser } from "../UserServices";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { displayProblem } from "@/http";
import { removeStateFromStorage } from "@/iam";

async function submit() {
  await deleteUser()
    .then(() => {
      removeStateFromStorage();
      push.success({
        title: "Compte supprimé",
        message: "Votre compte a été supprimé avec succès",
        duration: 5000,
      });
      setTimeout(() => {
        window.location.reload();
      }, 1000);
    })
    .catch(displayProblem);
}
</script>

<template>
  <Card class="shadow-2xl shadow-slate-900">
    <CardHeader>
      <CardTitle> Supprimer mon compte </CardTitle>
    </CardHeader>
    <CardContent>
      <CardDescription>
        En supprimant votre compte, vous perdrez l'accès à tous nos services et
        données. Vous ne pourrez pas récupérer votre compte.
      </CardDescription>
      <div class="flex justify-end mt-6">
        <AlertDialog>
          <AlertDialogTrigger as-child>
            <Button variant="destructive"> Supprimer mon compte </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle
                >Êtes-vous absolument certain ?</AlertDialogTitle
              >
              <AlertDialogDescription>
                Cette action est irréversible. Cela supprimera définitivement
                votre compte et supprimera vos données de nos serveurs.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Annuler</AlertDialogCancel>
              <AlertDialogAction @click="submit"> Supprimer </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </div>
    </CardContent>
  </Card>
</template>
