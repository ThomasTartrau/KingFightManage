<script setup lang="ts">
import { push } from "notivue";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { resendVerificationEmail, verifyEmail } from "./UserServices";
import Separator from "@/components/ui/separator/Separator.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import router from "@/router/router";
import { routes } from "@/router/routes";
import { displayProblem } from "@/http";

const route = useRoute();

const token = ref<string>("");

async function _load() {
  if (!route.query.token) {
    return push.error({
      title: "Token manquant",
      message: "Le token est manquant, veuillez vérifier votre lien email",
      duration: 5000,
    });
  }

  token.value = route.query.token as string;

  await verifyEmail(token.value)
    .then(() => {
      push.success({
        title: "Email vérifié",
        message:
          "Votre email a été vérifié avec succès. Vous pouvez vous connecter",
        duration: 5000,
      });
      return router.push({ name: routes.Login });
    })
    .catch(displayProblem);
}

async function submit() {
  await resendVerificationEmail(token.value)
    .then(() => {
      push.success({
        title: "Email envoyé",
        message:
          "Email envoyé avec succès. Veuillez vérifier votre boite email",
        duration: 5000,
      });
    })
    .catch(displayProblem);
}

onMounted(() => {
  _load();
});
</script>

<template>
  <div class="flex items-center justify-center min-h-screen">
    <Card>
      <CardHeader>
        <CardTitle> Vérifier votre email </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="flex flex-col gap-2 mb-4">
          <div class="flex flex-col gap-1">
            <div class="text-sm">
              Veuillez vérifier votre email pour vérifier votre compte
            </div>
            <div class="text-sm">
              Si vous n'avez pas reçu un email, cliquez ici pour l'envoyer
            </div>
          </div>
        </div>

        <Separator class="my-4" />

        <div class="flex flex-col gap-2 mt-4">
          <Button variant="outline" @click="submit"> Renvoyer l'email </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
