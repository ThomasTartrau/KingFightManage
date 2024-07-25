<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import { z } from 'zod'
import { push } from 'notivue'
import { onMounted, ref } from 'vue'
import { resetPassword } from './UserServices'
import Error404 from '@/pages/others/Error404.vue'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import type { Problem } from '@/http'
import { displayProblem } from '@/http'
import router from '@/router/router'
import { routes } from '@/router/routes'
import { Button } from '@/components/ui/button'

const token = ref<string>('')

const formSchema = toTypedSchema(
  z.object({
    new_password: z.string().min(1, 'Le mot de passe est requis'),
    confirm_password: z.string().min(1, 'Le mot de passe est requis'),
  }),
)

const { handleSubmit } = useForm({
  validationSchema: formSchema,
})

const onSubmit = handleSubmit((values) => {
  submit(values)
})

async function submit(values: {
  new_password: string
  confirm_password: string
}) {
  if (!values.new_password || !values.confirm_password) {
    return push.warning({
      title: 'Champs manquant',
      message: 'Tous les champs sont requis',
      duration: 5000,
    })
  }

  if (values.new_password !== values.confirm_password) {
    return push.warning({
      title: 'Mot de passe non correspondant',
      message: 'Les mots de passe ne correspondent pas',
      duration: 5000,
    })
  }

  if (!token.value) {
    return push.error({
      title: 'Token manquant',
      message: 'Le token est requis',
      duration: 5000,
    })
  }

  await resetPassword(token.value, values.new_password)
    .then(() => {
      push.success({
        title: 'Mot de passe réinitialisé avec succès',
        message: 'Vous pouvez vous connecter avec votre nouveau mot de passe',
        duration: 5000,
      })
      return router.push({ name: routes.Login })
    })
    .catch((problem: Problem) => {
      displayProblem(problem)
    })
}

function _load() {
  token.value = router.currentRoute.value.query.token as string
  if (!token.value) {
    push.error({
      title: 'Token manquant',
      message: 'Le token est requis',
      duration: 5000,
    })
  }
}

onMounted(() => {
  _load()
})
</script>

<template>
  <div v-if="token" class="flex items-center justify-center min-h-screen">
    <Card class="mx-auto">
      <CardHeader>
        <CardTitle>Réinitialisation du mot de passe</CardTitle>
        <CardDescription>
          Réinitialisez votre mot de passe en saisissant un nouveau mot de passe
          et en confirmant celui-ci.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form @submit.prevent="onSubmit">
          <div class="mb-4">
            <FormField v-slot="{ componentField }" name="new_password">
              <FormItem v-auto-animate>
                <FormLabel>Nouveau mot de passe</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    placeholder="************"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>

          <div class="mb-4">
            <FormField
              v-slot="{ componentField }"
              name="confirm_password"
              class="mb-6"
            >
              <FormItem v-auto-animate>
                <FormLabel>Confirmation du mot de passe</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    placeholder="************"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
          </div>

          <Button type="submit" class="w-full">
            Réinitialiser
          </Button>
        </form>

        <div class="mt-4 text-center text-sm">
          Une erreur ?
          <router-link
            :to="{ name: routes.BeginResetpassword }"
            class="underline"
          >
            Réessayer
          </router-link>
        </div>
      </CardContent>
    </Card>
  </div>
  <div v-else class="flex items-center justify-center min-h-screen">
    <component :is="Error404" />
  </div>
</template>
