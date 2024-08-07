<script setup lang="ts">
import type { ColumnDef, TableOptions } from "@tanstack/vue-table";
import {
  FlexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useVueTable,
} from "@tanstack/vue-table";
import { h, onMounted, reactive, ref } from "vue";

import { KeyRound } from "lucide-vue-next";
import { generateRegistrationToken } from "./StaffsService";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import DropdownAction from "@/components/custom/staffs/dropdown-action.vue";
import OnlineIcons from "@/components/custom/staffs/online_icons.vue";
import type { components } from "@/types";
import { Input } from "@/components/ui/input";
import { getRole } from "@/iam";
import type { Roles } from "@/utils/perms";
import perms, { Actions } from "@/utils/perms";

type definitions = components["schemas"];
type User = definitions["User"];

const props = defineProps<{
  data: User[];
}>();
const emit = defineEmits(["refreshDatatable"]);

const datas = ref<User[]>(props.data || []);

function emitRefresh() {
  emit("refreshDatatable");
}

const role = ref<null | Roles>();

async function handleGenerateRegistrationToken() {
  await generateRegistrationToken();
}

const columns: ColumnDef<User>[] = [
  {
    accessorKey: "user_id",
    header: "UUID",
    cell: ({ row }) => {
      return h("div", { class: "capitalize" }, row.getValue("user_id"));
    },
  },
  {
    accessorKey: "username",
    header: "Username",
    cell: ({ row }) =>
      h("div", { class: "lowercase" }, row.getValue("username")),
  },
  {
    accessorKey: "role",
    header: "Role",
    cell: ({ row }) => {
      return h("div", { class: "capitalize" }, row.getValue("role"));
    },
  },
  {
    accessorKey: "is_online",
    header: "Online",
    cell: ({ row }) => {
      const user = row.original;

      return h(OnlineIcons, { isOnline: user.is_online });
    },
  },
  {
    id: "actions",
    enableHiding: false,
    cell: ({ row }) => {
      const user = row.original;

      return h(DropdownAction, {
        userId: user.user_id,
        username: user.username,
      });
    },
  },
];

function onSearch(username: string) {
  datas.value = props.data.filter((user) =>
    user.username.toLowerCase().includes(username.toLowerCase()),
  );
}

const tableOptions = reactive<TableOptions<User>>({
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
    <div class="flex justify-between md:items-center mb-8 flex-col sm:flex-row">
      <Input
        id="search"
        type="text"
        placeholder="Rechercher un utilisateur"
        class="relative w-full max-w-sm items-center"
        @input="onSearch($event.target.value)"
      />
      <Button
        v-if="
          role &&
          perms.hasPermission(role, Actions.StaffsGenerateRegistrationToken)
        "
        class="mt-4 sm:mt-0"
        type="button"
        @click="handleGenerateRegistrationToken"
      >
        Générer un token
        <span class="ml-2">
          <KeyRound class="size-4" />
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
