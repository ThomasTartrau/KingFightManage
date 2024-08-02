<script setup lang="ts">
import { Ellipsis } from "lucide-vue-next";
import { onMounted, ref } from "vue";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { UUID } from "@/http";
import { getRole } from "@/iam";
import type { Roles } from "@/utils/perms";
import perms, { Actions } from "@/utils/perms";
import { deleteServiceAccessToken } from "./StaffsService";
import { push } from "notivue";

const props = defineProps<{
  tokenId: UUID;
  biscuit: string;
}>();

const emit = defineEmits(["refreshDatatable"]);
const role = ref<Roles | null>(null);

async function copyBiscuit() {
  try {
    await navigator.clipboard.writeText(props.biscuit);
    push.success({
      title: "Copié",
      message: "Le biscuit a été copié dans le presse-papiers.",
    });
  } catch {
    push.error({
      title: "Erreur",
      message: "Une erreur est survenue lors de la copie du biscuit.",
    });
  }
}

function handleDelete() {
  deleteServiceAccessToken(props.tokenId)
    .then(() => {
      emit("refreshDatatable");
    })
    .catch();
}

function _onLoad() {
  role.value = getRole().value;
}

onMounted(_onLoad);
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Ellipsis class="cursor-pointer text-right" />
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-56">
      <DropdownMenuLabel
        v-if="
          perms.hasPermission(role, Actions.ServiceAccessCopy) ||
          perms.hasPermission(role, Actions.ServiceAccessDelete)
        "
      >
        Actions
      </DropdownMenuLabel>
      <DropdownMenuSeparator
        v-if="
          perms.hasPermission(role, Actions.ServiceAccessCopy) ||
          perms.hasPermission(role, Actions.ServiceAccessDelete)
        "
      />
      <DropdownMenuItem @click="copyBiscuit">
        <span>Copier le biscuit</span>
      </DropdownMenuItem>
      <DropdownMenuItem @click="handleDelete">
        <span>Supprimer</span>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
