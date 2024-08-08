<script setup lang="ts">
import { Ellipsis } from "lucide-vue-next";
import { onMounted, ref } from "vue";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { getRole } from "@/iam";
import type { Roles } from "@/utils/perms";
import perms, { Actions } from "@/utils/perms";
import { Dialog } from "@/components/ui/dialog";
import { UUID } from "@/http";
import MessageDialog from "../staffs/message-dialog.vue";
import SanctionDialog from "./sanction-dialog.vue";
import { components } from "@/types";
import router from "@/router/router";

type definitions = components["schemas"];
type Sanction = definitions["Sanction"];

const props = defineProps<{
  player_id: UUID;
  player_name: string;
  sanctions: Sanction[];
}>();

const role = ref<Roles | null>(null);

const isMessageDialogOpen = ref(false);
function closeMessageDialog() {
  isMessageDialogOpen.value = false;
}
function openMessageDialog() {
  isMessageDialogOpen.value = true;
}

const isMuteDialogOpen = ref(false);
function closeMuteDialog() {
  isMuteDialogOpen.value = false;
}
function openMuteDialog() {
  isMuteDialogOpen.value = true;
}

const isKickDialogOpen = ref(false);
function closeKickDialog() {
  isKickDialogOpen.value = false;
}
function openKickDialog() {
  isKickDialogOpen.value = true;
}

const isBanDialogOpen = ref(false);
function closeBanDialog() {
  isBanDialogOpen.value = false;
}
function openBanDialog() {
  isBanDialogOpen.value = true;
}

function handlePlayerSanctionsLogs() {
  router.push({
    name: "SanctionsPlayersLogs",
    params: { player_id: props.player_id },
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
      <DropdownMenuLabel v-if="perms.hasPermission(role, Actions.PlayersMute)">
        Sanctions
      </DropdownMenuLabel>
      <DropdownMenuSeparator
        v-if="perms.hasPermission(role, Actions.PlayersMute)"
      />
      <DropdownMenuGroup>
        <DropdownMenuItem
          v-if="perms.hasPermission(role, Actions.PlayersMute)"
          @click="openMuteDialog"
        >
          <span>Mute</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          v-if="perms.hasPermission(role, Actions.PlayersKick)"
          @click="openKickDialog"
        >
          <span>Kick</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          v-if="perms.hasPermission(role, Actions.PlayersBan)"
          @click="openBanDialog"
        >
          <span>Ban</span>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuSeparator
        v-if="perms.hasPermission(role, Actions.PlayersSendMessage)"
      />
      <DropdownMenuItem
        v-if="perms.hasPermission(role, Actions.PlayersSendMessage)"
        @click="openMessageDialog"
      >
        <span>Envoyer un message</span>
      </DropdownMenuItem>
      <DropdownMenuItem
        v-if="perms.hasPermission(role, Actions.PlayersGetSanctionLogs)"
        @click="handlePlayerSanctionsLogs"
      >
        <span>Logs sanctions</span>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>

  <Dialog v-model:open="isMessageDialogOpen">
    <MessageDialog
      :user-id="props.player_id"
      :username="props.player_name"
      @close-message-modal="closeMessageDialog"
    />
  </Dialog>

  <Dialog v-model:open="isMuteDialogOpen">
    <SanctionDialog
      :player_id="props.player_id"
      :player_name="props.player_name"
      :sanctions="props.sanctions"
      sanction_type="mute"
      @close-sanction-dialog="closeMuteDialog"
    />
  </Dialog>

  <Dialog v-model:open="isKickDialogOpen">
    <SanctionDialog
      :player_id="props.player_id"
      :player_name="props.player_name"
      :sanctions="props.sanctions"
      sanction_type="kick"
      @close-sanction-dialog="closeKickDialog"
    />
  </Dialog>

  <Dialog v-model:open="isBanDialogOpen">
    <SanctionDialog
      :player_id="props.player_id"
      :player_name="props.player_name"
      :sanctions="props.sanctions"
      sanction_type="ban"
      @close-sanction-dialog="closeBanDialog"
    />
  </Dialog>
</template>
