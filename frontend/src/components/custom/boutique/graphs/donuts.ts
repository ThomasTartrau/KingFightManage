import type { GraphsLogs } from '@/pages/boutique/BoutiqueGraphs.vue'
import dateConverter from '@/utils/dateConverter'

function getMensualyPbBuys(logs: GraphsLogs, actualDate: Date) {
  return logs.pbLogs.reduce((acc, log) => {
    const month = dateConverter.timestampToDate(log.created_at).getMonth()
    if (month === actualDate.getMonth()) {
      acc += log.amount
    }
    return acc
  }, 0)
}

function getMonthlyObjectivePercentage(
  mensualyPbBuys: number,
  objective: number,
) {
  return (mensualyPbBuys / objective) * 100
}

function getDonutsData(logs: GraphsLogs, actualDate: Date) {
  const mensualyPbBuys = getMensualyPbBuys(logs, actualDate)
  return [
    {
      name: 'Achats (en €)',
      total: mensualyPbBuys,
    },
    { name: 'Objectif (en €)', total: logs.objective },
  ]
}

export default {
  getDonutsData,
  getMensualyPbBuys,
  getMonthlyObjectivePercentage,
}
