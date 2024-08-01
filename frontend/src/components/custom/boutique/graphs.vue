<script setup lang="ts">
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { GraphsLogs } from "@/pages/boutique/BoutiqueGraphs.vue";
import { DonutChart } from "@/components/ui/chart-donut";
import dateConverter from "@/utils/dateConverter";
import { Progress } from "@/components/ui/progress";
import { onMounted, ref, watch } from "vue";
import donuts from "./graphs/donuts";
import line from "./graphs/area";
import AreaChart from "@/components/ui/chart-area/AreaChart.vue";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const actualDate = new Date();
const props = defineProps<{
  logs: GraphsLogs;
}>();

// Donuts
const donutsData = ref<{ name: string; total: number }[]>([]);
const mensualyPbBuys = ref(0);
const monthlyObjectivePercentage = ref(0);

// Area
const period = ref<"Journalier" | "Hebdomadaire" | "Mensuel">("Journalier");
watch(period, () => {
  areaData.value = line.getLineData(props.logs.pbLogs, period.value);
});
const areaData = ref<{ date: string; total: number }[]>([]);

function _onLoad() {
  // Donuts
  donutsData.value = donuts.getDonutsData(props.logs, actualDate);
  mensualyPbBuys.value = donuts.getMensualyPbBuys(props.logs, actualDate);
  monthlyObjectivePercentage.value = donuts.getMonthlyObjectivePercentage(
    mensualyPbBuys.value,
    props.logs.objective
  );

  // Area
  areaData.value = line.getLineData(props.logs.pbLogs, period.value);
}

onMounted(_onLoad);
</script>

<template>
  <div class="grid gap-4 grid-cols-1 lg:grid-cols-2 xl:grid-cols-3">
    <Card class="shadow-2xl shadow-slate-900 xl:col-span-2">
      <CardHeader>
        <CardTitle class="flex justify-between">
          Achats {{ period }}
          <Select v-model="period">
            <SelectTrigger class="w-1/3">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Journalier">Journalier</SelectItem>
              <SelectItem value="Hebdomadaire">Hebdomadaire</SelectItem>
              <SelectItem value="Mensuel">Mensuel</SelectItem>
            </SelectContent>
          </Select>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <AreaChart
          :data="areaData"
          index="date"
          category="total"
          :categories="['total']"
          :colors="['#c0ba']"
          :y-formatter="
            (tick, _i) => {
              return typeof tick === 'number'
                ? `${new Intl.NumberFormat('us').format(tick).toString()} €`
                : '';
            }
          "
        />
      </CardContent>
    </Card>
    <Card class="shadow-2xl shadow-slate-900">
      <CardHeader>
        <CardTitle>Objectif mensuel</CardTitle>
      </CardHeader>
      <CardContent>
        <DonutChart
          :data="donutsData"
          index="name"
          category="total"
          :colors="['#120469', '#872bbd']"
          type="pie"
        />
      </CardContent>
      <CardFooter>
        <div class="w-full flex flex-col items-center lg:mt-16">
          <h2 class="mb-6">
            {{ monthlyObjectivePercentage }}% atteint actuellement -
            {{ dateConverter.monthDayLeft(actualDate) }} jours restants
          </h2>
          <Progress v-model="monthlyObjectivePercentage" class="w-2/3" />
        </div>
      </CardFooter>
    </Card>
  </div>
</template>
