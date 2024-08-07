<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import donuts from './graphs/donuts'
import line from './graphs/area'
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import type { GraphsLogs } from '@/pages/boutique/BoutiqueGraphs.vue'
import { DonutChart } from '@/components/ui/chart-donut'
import dateConverter from '@/utils/dateConverter'
import { Progress } from '@/components/ui/progress'
import AreaChart from '@/components/ui/chart-area/AreaChart.vue'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import CardDescription from '@/components/ui/card/CardDescription.vue'

const props = defineProps<{
  logs: GraphsLogs
}>()
const actualDate = new Date()
const totalBuys = props.logs.pbLogs.reduce((acc, curr) => acc + curr.amount, 0)

// Donuts
const donutsData = ref<{ name: string, total: number }[]>([])
const mensualyPbBuys = ref(0)
const monthlyObjectivePercentage = ref(0)

// Area
const period = ref<'journalier' | 'hebdomadaire' | 'mensuel'>('journalier')
const areaData = ref<{ date: string, total: number }[]>([])
const averageBuyInPeriod = ref(0)
const buyInThisPeriod = ref(0)

function _onLoad() {
  // Donuts
  donutsData.value = donuts.getDonutsData(props.logs, actualDate)
  mensualyPbBuys.value = donuts.getMensualyPbBuys(props.logs, actualDate)
  const tempMonthlyObjectivePercentage = donuts.getMonthlyObjectivePercentage(
    mensualyPbBuys.value,
    props.logs.objective,
  )
  monthlyObjectivePercentage.value
    = tempMonthlyObjectivePercentage > 100 ? 100 : tempMonthlyObjectivePercentage

  // Area
  areaData.value = line.getLineData(props.logs.pbLogs, period.value)
  averageBuyInPeriod.value = line.getAverageBuyInPeriod(areaData.value)
  buyInThisPeriod.value = line.buyInThisPeriod.value

  watch(period, () => {
    areaData.value = line.getLineData(props.logs.pbLogs, period.value)
    averageBuyInPeriod.value = line.getAverageBuyInPeriod(areaData.value)
    buyInThisPeriod.value = line.buyInThisPeriod.value
  })
}

onMounted(_onLoad)
</script>

<template>
  <div class="grid gap-4 grid-cols-1 lg:grid-cols-2 xl:grid-cols-3">
    <Card class="shadow-2xl shadow-slate-900 xl:col-span-2">
      <CardHeader>
        <CardDescription>Achats boutique</CardDescription>
        <CardTitle class="flex justify-between">
          {{ new Intl.NumberFormat("fr-FR").format(totalBuys) }}€ récolté(s)
          <Select v-model="period">
            <SelectTrigger class="w-1/3">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="journalier">
                Journalier
              </SelectItem>
              <SelectItem value="hebdomadaire">
                Hebdomadaire
              </SelectItem>
              <SelectItem value="mensuel">
                Mensuel
              </SelectItem>
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
                ? `${new Intl.NumberFormat('fr-FR').format(tick).toString()} €`
                : '';
            }
          "
        />
      </CardContent>
      <CardFooter>
        <div class="flex flex-col gap-4 w-full">
          <Card class="dark:border-gray-500">
            <CardHeader>
              <CardDescription>En moyenne {{ period }}</CardDescription>
              <CardTitle>
                {{ new Intl.NumberFormat("fr-FR").format(averageBuyInPeriod) }}€
              </CardTitle>
            </CardHeader>
            <CardContent>
              <h2>
                <span class="font-bold text-xl">{{
                  new Intl.NumberFormat("fr-FR").format(buyInThisPeriod)
                }}</span>€ récolté(s) durant cette période
              </h2>
            </CardContent>
          </Card>
        </div>
      </CardFooter>
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
        <div class="w-full flex flex-col items-center lg:mt-32">
          <h2 class="mb-6 text-2xl text-center">
            <span class="font-medium">{{ monthlyObjectivePercentage }}%</span>
            atteint actuellement -
            <span class="font-medium">{{
              dateConverter.monthDayLeft(actualDate)
            }}</span>
            jours restants
          </h2>
          <Progress v-model="monthlyObjectivePercentage" class="w-2/3" />
        </div>
      </CardFooter>
    </Card>
  </div>
</template>
