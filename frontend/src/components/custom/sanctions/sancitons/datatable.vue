<script setup lang="ts">
import type { ColumnDef, TableOptions } from '@tanstack/vue-table'
import {
  FlexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useVueTable,
} from '@tanstack/vue-table'
import { h, onMounted, reactive, ref } from 'vue'

import { CirclePlus } from 'lucide-vue-next'
import DropdownAction from './dropdown-action.vue'
import CreateSanctionDialog from './create-sanction-dialog.vue'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type { components } from '@/types'
import { Input } from '@/components/ui/input'
import { getRole } from '@/iam'
import type { Roles } from '@/utils/perms'
import perms, { Actions } from '@/utils/perms'
import { Dialog } from '@/components/ui/dialog'

type definitions = components['schemas']
type Sanction = definitions['Sanction']

const props = defineProps<{
  data: Sanction[]
}>()
const emit = defineEmits(['refreshDatatable'])

const isCreateSanctionOpen = ref(false)
function openCreateSanctionDialog() {
  isCreateSanctionOpen.value = true
}
function closeCreateSanctionDialog() {
  isCreateSanctionOpen.value = false
}
function closeAndRefresh() {
  closeCreateSanctionDialog()
  emitRefresh()
}

const datas = ref<Sanction[]>(props.data || [])

function emitRefresh() {
  emit('refreshDatatable')
}

const role = ref<null | Roles>()

const columns: ColumnDef<Sanction>[] = [
  {
    accessorKey: 'type_',
    header: 'Type',
    cell: ({ row }) => h('div', { class: 'capitalize' }, row.getValue('type_')),
  },
  {
    accessorKey: 'name',
    header: 'Nom',
    cell: ({ row }) => {
      return h('div', row.getValue('name'))
    },
  },
  {
    accessorKey: 'duration',
    header: 'Duration',
    cell: ({ row }) => {
      const sanction = row.original

      return h('div', sanction.duration)
    },
  },
  {
    id: 'actions',
    enableHiding: false,
    cell: ({ row }) => {
      const sanction = row.original

      return h(DropdownAction, {
        sanction,
      })
    },
  },
]

function onSearch(username: string) {
  datas.value = props.data.filter(
    sanction =>
      sanction.type_.toLowerCase().includes(username.toLowerCase())
      || sanction.name.toLowerCase().includes(username.toLowerCase()),
  )
}

const tableOptions = reactive<TableOptions<Sanction>>({
  get data() {
    return datas.value
  },
  get columns() {
    return columns
  },
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
})

const table = useVueTable(tableOptions)

function _onLoad() {
  role.value = getRole().value
}

onMounted(_onLoad)
</script>

<template>
  <div class="w-full">
    <div class="flex justify-between md:items-center mb-8 flex-col sm:flex-row">
      <Input
        id="search"
        type="text"
        placeholder="Rechercher un type ou un nom de sanction"
        class="relative w-full max-w-sm items-center"
        @input="onSearch($event.target.value)"
      />
      <Button
        v-if="role && perms.hasPermission(role, Actions.SanctionsCreate)"
        type="button"
        class="mt-4 sm:mt-0"
        @click="openCreateSanctionDialog"
      >
        Créer une sanction
        <span class="ml-2">
          <CirclePlus class="size-4" />
        </span>
      </Button>
    </div>
    <div class="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow
            v-for="headerGroup in table.getHeaderGroups()"
            :key="headerGroup.id"
          >
            <TableHead v-for="header in headerGroup.headers" :key="header.id">
              <FlexRender
                v-if="!header.isPlaceholder"
                :render="header.column.columnDef.header"
                :props="header.getContext()"
              />
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="table.getRowModel().rows?.length">
            <template v-for="row in table.getRowModel().rows" :key="row.id">
              <TableRow :data-state="row.getIsSelected() && 'selected'">
                <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id">
                  <FlexRender
                    :render="cell.column.columnDef.cell"
                    :props="cell.getContext()"
                    @refresh-datatable="emitRefresh"
                  />
                </TableCell>
              </TableRow>
              <TableRow v-if="row.getIsExpanded()">
                <TableCell :colspan="row.getAllCells().length">
                  {{ JSON.stringify(row.original) }}
                </TableCell>
              </TableRow>
            </template>
          </template>

          <TableRow v-else>
            <TableCell :colspan="columns.length" class="h-24 text-center">
              Aucun résultat trouvé
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <div class="flex items-center justify-end space-x-2 py-4">
      <div class="flex-1 text-sm text-muted-foreground">
        Affichage de
        {{
          table.getFilteredRowModel().rows.length
            < (table.getState().pagination.pageIndex + 1)
            * table.getState().pagination.pageSize
            ? table.getFilteredRowModel().rows.length
            : (table.getState().pagination.pageIndex + 1)
              * table.getState().pagination.pageSize
        }}
        sur {{ table.getFilteredRowModel().rows.length }} donnée(s).
      </div>
      <div class="space-x-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="!table.getCanPreviousPage()"
          @click="table.previousPage()"
        >
          Précedente
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="!table.getCanNextPage()"
          @click="table.nextPage()"
        >
          Suivante
        </Button>
      </div>
    </div>
  </div>
  <Dialog v-model:open="isCreateSanctionOpen">
    <CreateSanctionDialog
      @close-create-sanciton-dialog="closeCreateSanctionDialog"
      @close-and-refresh-sanction-dialog="closeAndRefresh"
    />
  </Dialog>
</template>
