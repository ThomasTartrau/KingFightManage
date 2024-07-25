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
import { defineProps } from "vue";
import { deleteUser, setRole } from "@/components/custom/staffs/StaffsService";

const props = defineProps<{
  user_id: UUID;
}>();

const emit = defineEmits(["refreshDatatable"]);

const ranks: String[] = ["Moderateur", "Administrateur", "Developpeur"];

function handleSetRole(role: String) {
  setRole(props.user_id, role).then(() => {
    emit("refreshDatatable");
  });
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
      <DropdownMenuLabel>Manage</DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuSub>
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
        <DropdownMenuItem @click="handleDelete">
          <span>Delete</span>
        </DropdownMenuItem>
      </DropdownMenuGroup>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
