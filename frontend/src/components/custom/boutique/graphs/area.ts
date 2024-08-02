import { PbLogs } from "@/pages/boutique/BoutiqueService";
import dateConverter from "@/utils/dateConverter";

function getPerDayPbBuys(pbLogs: PbLogs): { date: string; total: number }[] {
  const map = new Map<string, number>();
  pbLogs.forEach((log) => {
    const day = dateConverter.getDayString(
      dateConverter.timestampToDate(log.created_at)
    );
    map.set(day.toString(), log.amount);
  });

  const perDayPbBuys: { date: string; total: number }[] = [];
  map.forEach((amount, day) => {
    perDayPbBuys.push({ date: day, total: amount });
  });

  return dateConverter.sortByDayString(perDayPbBuys);
}

function getPerWeekPbBuys(pbLogs: PbLogs): { date: string; total: number }[] {
  const map = new Map<string, number>();
  pbLogs.forEach((log) => {
    const week = dateConverter.getWeekString(
      dateConverter.timestampToDate(log.created_at)
    );
    map.set(week.toString(), log.amount);
  });

  const perWeekPbBuys: { date: string; total: number }[] = [];
  map.forEach((amount, week) => {
    perWeekPbBuys.push({ date: week, total: amount });
  });

  return dateConverter.sortByWeekString(perWeekPbBuys);
}

function getPerMonthPbBuys(pbLogs: PbLogs): { date: string; total: number }[] {
  const map = new Map<string, number>();
  pbLogs.forEach((log) => {
    const month = dateConverter.getMonthString(
      dateConverter.timestampToDate(log.created_at)
    );
    map.set(month.toString(), log.amount);
  });

  const perMonthPbBuys: { date: string; total: number }[] = [];
  map.forEach((amount, month) => {
    perMonthPbBuys.push({ date: month, total: amount });
  });

  return dateConverter.sortByMonthString(perMonthPbBuys);
}

function getLineData(
  pbLogs: PbLogs,
  period: "Journalier" | "Hebdomadaire" | "Mensuel"
) {
  if (period === "Journalier") {
    return getPerDayPbBuys(pbLogs);
  } else if (period === "Hebdomadaire") {
    return getPerWeekPbBuys(pbLogs);
  } else {
    return getPerMonthPbBuys(pbLogs);
  }
}

export default {
  getLineData,
};
