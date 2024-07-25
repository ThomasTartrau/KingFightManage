<script setup lang="ts">
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { UUID } from "@/http";
import { Ellipsis } from "lucide-vue-next";
import { defineProps, ref } from "vue";
import {
  deleteUser,
  sendMessage,
  setRole,
} from "@/components/custom/staffs/StaffsService";
import { getRole } from "@/iam";
import MessageDialog from "./message-dialog.vue";
import { Dialog } from "@/components/ui/dialog";

const props = defineProps<{
  user_id: UUID;
  username: string;
}>();

const emit = defineEmits(["refreshDatatable"]);
const role = getRole();

const isMessageDialogOpen = ref(false);
const closeMessageDialog = () => {
  isMessageDialogOpen.value = false;
};
const openMessageDialog = () => {
  isMessageDialogOpen.value = true;
};

const ranks: String[] = ["Moderateur", "Administrateur", "Developpeur"];

function handleSetRole(role: String) {
  setRole(props.user_id, role).then(() => {
    emit("refreshDatatable");
  });
}

function ifHasRole(roles: Array<String>): Boolean {
  if (role.value === null) return false;
  return roles.includes(role.value);
}

function handleDelete() {
  deleteUser(props.user_id).then(() => {
    emit("refreshDatatable");
  });
}
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Ellipsis class="cursor-pointer text-right" />
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-56">
      <DropdownMenuLabel v-if="ifHasRole(['developpeur'])"
        >Manage</DropdownMenuLabel
      >
      <DropdownMenuSeparator v-if="ifHasRole(['developpeur'])" />
      <DropdownMenuGroup>
        <DropdownMenuSub v-if="ifHasRole(['developpeur'])">
          <DropdownMenuSubTrigger>
            <span>Set ranks</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem
              v-for="rank in ranks"
              @click="handleSetRole(rank)"
            >
              <span>{{ rank }}</span>
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuItem
          v-if="ifHasRole(['developpeur'])"
          @click="handleDelete"
        >
          <span>Delete</span>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuLabel>
        <span>General</span>
      </DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuItem @click="openMessageDialog">
        <span>Envoyer un message</span>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>

  <Dialog v-model:open="isMessageDialogOpen">
    <MessageDialog
      :user_id="props.user_id"
      :username="props.username"
      @closeMessageModal="closeMessageDialog"
    />
  </Dialog>
</template>
