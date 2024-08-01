<script setup lang="ts">
import type { ColumnDef, TableOptions } from "@tanstack/vue-table";
import {
  FlexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useVueTable,
} from "@tanstack/vue-table";
import { h, reactive, ref } from "vue";

import { Search } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type { components } from "@/types";
import { Input } from "@/components/ui/input";
import dateConverter from "@/utils/dateConverter";

type definitions = components["schemas"];
type Log = definitions["Log"];

const props = defineProps<{
  data: Log[];
}>();
const emitRefresh = defineEmits(["refreshDatatable"]);

const datas = ref<Log[]>(props.data || []);

const columns: ColumnDef<Log>[] = [
  {
    accessorKey: "log_id",
    header: "Log ID",
    cell: ({ row }) => {
      return h("div", { class: "capitalize" }, row.getValue("log_id"));
    },
  },
  {
    accessorKey: "username",
    header: "Username",
    cell: ({ row }) =>
      h("div", { class: "lowercase" }, row.getValue("username")),
  },
  {
    accessorKey: "action",
    header: "Action",
    cell: ({ row }) => {
      return h("div", { class: "capitalize" }, row.getValue("action"));
    },
  },
  {
    accessorKey: "created_at",
    header: "Date",
    cell: ({ row }) => {
      return h(
        "div",
        { class: "capitalize" },
        dateConverter.timestampToDateString(row.getValue("created_at"))
      );
    },
  },
];

function onSearch(search: string) {
  datas.value = props.data.filter(
    (log) =>
      log.log_id.toLowerCase().includes(search.toLowerCase()) ||
      log.username.toLowerCase().includes(search.toLowerCase()) ||
      log.action.toLowerCase().includes(search.toLowerCase())
  );
}

const tableOptions = reactive<TableOptions<Log>>({
  get data() {
    return datas.value;
  },
  get columns() {
    return columns;
  },
  getCoreRowModel: getCoreRowModel(),
  rowCount: datas.value.length,
  getPaginationRowModel: getPaginationRowModel(),
});

const table = useVueTable(tableOptions);
</script>

<template>
  <div class="w-full">
    <div class="relative w-full max-w-sm items-center mb-8">
      <Input
        id="search"
        type="text"
        placeholder="Rechercher un utilisateur / une action / une id de log"
        class="pl-10"
        @input="onSearch($event.target.value)"
      />
      <span
        class="absolute start-0 inset-y-0 flex items-center justify-center px-2"
      >
        <Search class="size-6 text-muted-foreground" />
      </span>
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
          (table.getState().pagination.pageIndex + 1) *
          table.getState().pagination.pageSize
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
