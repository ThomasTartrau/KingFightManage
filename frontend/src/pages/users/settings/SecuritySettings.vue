<script setup lang="ts">
import { push } from 'notivue'
import { ref } from 'vue'
import { changePassword } from '../UserServices'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { displayProblem } from '@/http'

const isPasswordDialogOpen = ref<boolean>(false)
const closePasswordDialog = () => (isPasswordDialogOpen.value = false)

const new_password = ref<string>('')
const confirm_password = ref<string>('')

async function submit() {
  if (new_password.value !== confirm_password.value) {
    return push.error({
      title: 'Mot de passe invalide',
      message: 'Les mots de passe ne correspondent pas',
      duration: 5000,
    })
  }

  await changePassword(new_password.value)
    .then(() => {
      push.success({
        title: 'Mot de passe changé',
        message: 'Votre mot de passe a été changé avec succès',
        duration: 5000,
      })
    })
    .catch(displayProblem)
}
</script>

<template>
  <Card class="shadow-2xl shadow-slate-900">
    <CardHeader>
      <CardTitle> Paramètres de sécurité </CardTitle>
    </CardHeader>
    <CardContent>
      <CardDescription>
        Changez votre mot de passe ici. Veuillez entrer votre nouveau mot de
        passe et le confirmer. Vous devez utiliser un mot de passe sécurisé avec
        au moins 12 caractères.
      </CardDescription>
      <div class="flex justify-end">
        <Dialog v-model:open="isPasswordDialogOpen">
          <form>
            <DialogTrigger as-child>
              <Button variant="outline" class="mt-8">
                Changer le mot de passe
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Changer le mot de passe</DialogTitle>
                <DialogDescription>
                  Changez votre mot de passe ici. Veuillez entrer votre nouveau
                  mot de passe et le confirmer. Vous devez utiliser un mot de
                  passe sécurisé avec au moins 12 caractères.
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="newPassword" class="text-right">
                    Nouveau mot de passe
                  </Label>
                  <Input
                    id="newPassword"
                    v-model="new_password"
                    type="password"
                    class="col-span-3"
                  />
                </div>
                <div class="grid grid-cols-4 items-center gap-4">
                  <Label for="confirmPassword" class="text-right">
                    Confirmer le mot de passe
                  </Label>
                  <Input
                    id="confirmPassword"
                    v-model="confirm_password"
                    type="password"
                    class="col-span-3"
                  />
                </div>
              </div>
              <DialogFooter>
                <Button variant="secondary" @click="closePasswordDialog">
                  Annuler
                </Button>
                <Button @click="submit">
                  Modifier
                </Button>
              </DialogFooter>
            </DialogContent>
          </form>
        </Dialog>
      </div>
    </CardContent>
  </Card>
</template>
