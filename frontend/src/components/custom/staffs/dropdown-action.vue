<script setup lang="ts">
import { Ellipsis } from "lucide-vue-next";
import { onMounted, ref } from "vue";
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
import type { Roles } from "@/utils/perms";
import perms, { Actions } from "@/utils/perms";
import { Dialog } from "@/components/ui/dialog";

const props = defineProps<{
  userId: UUID;
  username: string;
}>();

const emit = defineEmits(["refreshDatatable"]);
const role = ref<Roles | null>(null);

const isMessageDialogOpen = ref(false);
function closeMessageDialog() {
  isMessageDialogOpen.value = false;
}
function openMessageDialog() {
  isMessageDialogOpen.value = true;
}

function handleSetRole(role: string) {
  setRole(props.userId, role).then(() => {
    emit("refreshDatatable");
  });
}

function handleDelete() {
  deleteUser(props.userId).then(() => {
    emit("refreshDatatable");
  });
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
        v-if="perms.hasPermission(role, Actions.StaffsSetRank)"
      >
        Gestion
      </DropdownMenuLabel>
      <DropdownMenuSeparator
        v-if="perms.hasPermission(role, Actions.StaffsSetRank)"
      />
      <DropdownMenuGroup>
        <DropdownMenuSub
          v-if="perms.hasPermission(role, Actions.StaffsSetRank)"
        >
          <DropdownMenuSubTrigger>
            <span>Gérer les rôles</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem
              v-for="rank in perms.getStringRoles()"
              :key="rank"
              @click="handleSetRole(rank)"
            >
              <span>{{ rank }}</span>
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuItem
          v-if="perms.hasPermission(role, Actions.StaffsDeleteUser)"
          @click="handleDelete"
        >
          <span>Supprimer</span>
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
