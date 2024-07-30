<script setup lang="ts">
import { Ellipsis } from "lucide-vue-next";
import { ref } from "vue";
import MessageDialog from "./message-dialog.vue";
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
import type { UUID } from "@/http";
import { deleteUser, setRole } from "@/components/custom/staffs/StaffsService";
import { getRole } from "@/iam";
import { Dialog } from "@/components/ui/dialog";

const props = defineProps<{
  userId: UUID;
  username: string;
}>();

const emit = defineEmits(["refreshDatatable"]);
const role = getRole();

const isMessageDialogOpen = ref(false);
function closeMessageDialog() {
  isMessageDialogOpen.value = false;
}
function openMessageDialog() {
  isMessageDialogOpen.value = true;
}

const ranks: string[] = ["Moderateur", "Administrateur", "Developpeur"];

function handleSetRole(role: string) {
  setRole(props.userId, role).then(() => {
    emit("refreshDatatable");
  });
}

function ifHasRole(roles: Array<string>): boolean {
  if (role.value === null) return false;
  return roles.includes(role.value);
}

function handleDelete() {
  deleteUser(props.userId).then(() => {
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
      <DropdownMenuLabel v-if="ifHasRole(['developpeur'])">
        Manage
      </DropdownMenuLabel>
      <DropdownMenuSeparator v-if="ifHasRole(['developpeur'])" />
      <DropdownMenuGroup>
        <DropdownMenuSub v-if="ifHasRole(['developpeur'])">
          <DropdownMenuSubTrigger>
            <span>Set ranks</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem
              v-for="rank in ranks"
              :key="rank"
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
      :user-id="props.userId"
      :username="props.username"
      @close-message-modal="closeMessageDialog"
    />
  </Dialog>
</template>
