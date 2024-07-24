<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { components } from "@/types";
import axios from "axios";
import { ref } from "vue";
import { getStaffs } from "./StaffsService";
import { displayError } from "@/http";

type definitions = components["schemas"];
type User = definitions["User"];

const staffs = ref<User[]>([]);

function get() {
  getStaffs()
    .then((users: User[]) => {
      staffs.value = users;
    })
    .catch(displayError);
}
</script>

<template>
  <div>
    <button @click="get">Get Staffs</button>
  </div>
  <Promise>
    <p>test</p>
  </Promise>
</template>
