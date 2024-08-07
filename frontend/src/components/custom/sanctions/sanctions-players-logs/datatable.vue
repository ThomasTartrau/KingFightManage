<script setup lang="ts">
import type { ColumnDef, TableOptions } from '@tanstack/vue-table'
import {
  FlexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useVueTable,
} from '@tanstack/vue-table'
import { h, reactive, ref } from 'vue'

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
import dateConverter from '@/utils/dateConverter'

type definitions = components['schemas']
type GetSanctionsLogs = definitions['GetSanctionsLogs']

const props = defineProps<{
  data: GetSanctionsLogs[]
}>()
const emit = defineEmits(['refreshDatatable'])

const datas = ref<GetSanctionsLogs[]>(props.data || [])

function emitRefresh() {
  emit('refreshDatatable')
}

const columns: ColumnDef<GetSanctionsLogs>[] = [
  {
    accessorKey: 'player_name',
    header: 'Nom du joueur',
    cell: ({ row }) => h('div', row.getValue('player_name')),
  },
  {
    accessorKey: 'staff_name',
    header: 'Sanctionné par',
    cell: ({ row }) => {
      return h('div', row.getValue('staff_name'))
    },
  },
  {
    accessorKey: 'sanction_type_and_name',
    header: 'Type & Nom de la sanction',
    cell: ({ row }) => {
      const sanction = row.original

      return h('div', `${sanction.sanction_type} - ${sanction.sanction_name}`)
    },
  },
  {
    accessorKey: 'sanction_motif',
    header: 'Motif',
    cell: ({ row }) => {
      return h('div', row.getValue('sanction_motif'))
    },
  },
  {
    accessorKey: 'sanction_created_at',
    header: 'Date',
    cell: ({ row }) => {
      return h(
        'div',
        dateConverter.timestampToDateString(row.getValue('sanction_created_at')),
      )
    },
  },
]

function onSearch(search: string) {
  datas.value = props.data.filter(
    sanction =>
      sanction.sanction_motif.toLowerCase().includes(search.toLowerCase())
      || sanction.staff_name.toLowerCase().includes(search.toLowerCase()),
  )
}

const tableOptions = reactive<TableOptions<GetSanctionsLogs>>({
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
</script>

<template>
  <div class="w-full">
    <div class="relative w-full max-w-sm items-center mb-8">
      <Input
        id="search"
        type="text"
        placeholder="Rechercher un staff ou un motif"
        class="relative w-full max-w-sm items-center"
        @input="onSearch($event.target.value)"
      />
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
</template>
