<script setup lang="ts">
import { Ellipsis } from 'lucide-vue-next'
import { onMounted, ref } from 'vue'
import { deleteSanction } from './SanctionsService'
import UpdateSanctionDialog from './update-sanction-dialog.vue'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { getRole } from '@/iam'
import type { Roles } from '@/utils/perms'
import perms, { Actions } from '@/utils/perms'
import { Dialog } from '@/components/ui/dialog'
import type { components } from '@/types'

type definitions = components['schemas']
type Sanction = definitions['Sanction']

const props = defineProps<{
  sanction: Sanction
}>()
const emit = defineEmits(['refreshDatatable'])
const isUpdateSanctionDialogOpen = ref(false)
function closeUpdateSanctionDialog() {
  isUpdateSanctionDialogOpen.value = false
}
function openUpdateSanctionDialog() {
  isUpdateSanctionDialogOpen.value = true
}

function closeAndRefresh() {
  closeUpdateSanctionDialog()
  emit('refreshDatatable')
}

const role = ref<Roles | null>(null)

function handleDelete() {
  deleteSanction(props.sanction.sanction_id).then(() => {
    emit('refreshDatatable')
  })
}

function _onLoad() {
  role.value = getRole().value
}

onMounted(_onLoad)
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Ellipsis class="cursor-pointer text-right" />
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-56">
      <DropdownMenuLabel
        v-if="perms.hasPermission(role, Actions.SanctionsDelete)"
      >
        Gestion
      </DropdownMenuLabel>
      <DropdownMenuSeparator
        v-if="perms.hasPermission(role, Actions.SanctionsDelete)"
      />
      <DropdownMenuGroup>
        <DropdownMenuItem
          v-if="perms.hasPermission(role, Actions.SanctionsUpdate)"
          @click="openUpdateSanctionDialog"
        >
          <span>Modifier</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          v-if="perms.hasPermission(role, Actions.StaffsDeleteUser)"
          @click="handleDelete"
        >
          <span>Supprimer</span>
        </DropdownMenuItem>
      </DropdownMenuGroup>
    </DropdownMenuContent>
  </DropdownMenu>

  <Dialog v-model:open="isUpdateSanctionDialogOpen">
    <UpdateSanctionDialog
      :sanction-id="props.sanction.sanction_id"
      :type="props.sanction.type_"
      :name="props.sanction.name"
      :duration="props.sanction.duration"
      @close-update-sanciton-dialog="closeUpdateSanctionDialog"
      @close-and-refresh-sanction-dialog="closeAndRefresh"
    />
  </Dialog>
</template>
