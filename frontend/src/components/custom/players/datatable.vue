<script setup lang="ts">
import type { ColumnDef, TableOptions } from "@tanstack/vue-table";
import {
  FlexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useVueTable,
} from "@tanstack/vue-table";
import { h, onMounted, reactive, ref } from "vue";

import DropdownAction from "./dropdown-action.vue";
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
import { getRole } from "@/iam";
import type { Roles } from "@/utils/perms";
import dateConverter from "@/utils/dateConverter";
import type { promise$ } from "@/pages/players/GetPlayers.vue";

type definitions = components["schemas"];
type Player = definitions["Player"];

const props = defineProps<{
  data: promise$;
}>();

const datas = ref<Player[]>(props.data.players$ || []);

const role = ref<null | Roles>();

const columns: ColumnDef<Player>[] = [
  {
    accessorKey: "player_id",
    header: "UUID",
    cell: ({ row }) => h("div", row.getValue("player_id")),
  },
  {
    accessorKey: "name",
    header: "Nom",
    cell: ({ row }) => {
      return h("div", row.getValue("name"));
    },
  },
  {
    accessorKey: "created_at",
    header: "Première connexion",
    cell: ({ row }) => {
      const sanction = row.original;

      return h("div", dateConverter.timestampToDateString(sanction.created_at));
    },
  },
  {
    id: "actions",
    enableHiding: false,
    cell: ({ row }) => {
      const player = row.original;

      return h(DropdownAction, {
        playerId: player.player_id,
        playerName: player.name,
        sanctions: props.data.sanctions$,
      });
    },
  },
];

function onSearch(search: string) {
  datas.value = props.data.players$.filter(
    (player) =>
      player.player_id.toLowerCase().includes(search.toLowerCase()) ||
      player.name.toLowerCase().includes(search.toLowerCase()),
  );
}

const tableOptions = reactive<TableOptions<Player>>({
  get data() {
    return datas.value;
  },
  get columns() {
    return columns;
  },
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
});

const table = useVueTable(tableOptions);

function _onLoad() {
  role.value = getRole().value;
}

onMounted(_onLoad);
</script>

<template>
  <div class="w-full">
    <div class="relative w-full max-w-sm items-center mb-8">
      <Input
        id="search"
        type="text"
        placeholder="Rechercher un id ou nom de joueur"
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
          table.getFilteredRowModel().rows.length <
          (table.getState().pagination.pageIndex + 1) *
            table.getState().pagination.pageSize
            ? table.getFilteredRowModel().rows.length
            : (table.getState().pagination.pageIndex + 1) *
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
