import { ref } from "vue";
import type { PbLogs } from "@/pages/boutique/BoutiqueService";
import dateConverter from "@/utils/dateConverter";

const buyInThisPeriod = ref(0);
const actualDate = ref(new Date());

function getPerDayPbBuys(pbLogs: PbLogs): { date: string; total: number }[] {
  const map = new Map<string, number>();
  let total = 0;
  pbLogs.forEach((log) => {
    const date = dateConverter.timestampToDate(log.created_at);
    if (isToday(date)) {
      total += log.amount;
    }
    const day = dateConverter.getDayString(date);
    if (map.has(day)) {
      map.set(day, (map.get(day) ?? 0) + log.amount);
    } else {
      map.set(day, log.amount);
    }
  });

  const perDayPbBuys: { date: string; total: number }[] = [];
  map.forEach((amount, day) => {
    perDayPbBuys.push({ date: day, total: amount });
  });

  buyInThisPeriod.value = total;
  return dateConverter.sortByDayString(perDayPbBuys);
}

function getPerWeekPbBuys(pbLogs: PbLogs): { date: string; total: number }[] {
  const map = new Map<string, number>();
  let total = 0;
  pbLogs.forEach((log) => {
    const date = dateConverter.timestampToDate(log.created_at);
    if (isThisWeek(date)) {
      total += log.amount;
    }
    const week = dateConverter.getWeekString(date);
    if (map.has(week)) {
      map.set(week, (map.get(week) ?? 0) + log.amount);
    } else {
      map.set(week, log.amount);
    }
  });

  const perWeekPbBuys: { date: string; total: number }[] = [];
  map.forEach((amount, week) => {
    perWeekPbBuys.push({ date: week, total: amount });
  });

  buyInThisPeriod.value = total;
  return dateConverter.sortByWeekString(perWeekPbBuys);
}

function getPerMonthPbBuys(pbLogs: PbLogs): { date: string; total: number }[] {
  const map = new Map<string, number>();
  let total = 0;
  pbLogs.forEach((log) => {
    const date = dateConverter.timestampToDate(log.created_at);
    if (isThisMonth(date)) {
      total += log.amount;
    }
    const month = dateConverter.getMonthString(date);
    if (map.has(month)) {
      map.set(month, (map.get(month) ?? 0) + log.amount);
    } else {
      map.set(month, log.amount);
    }
  });

  const perMonthPbBuys: { date: string; total: number }[] = [];
  map.forEach((amount, month) => {
    perMonthPbBuys.push({ date: month, total: amount });
  });

  buyInThisPeriod.value = total;
  return dateConverter.sortByMonthString(perMonthPbBuys);
}

function getLineData(
  pbLogs: PbLogs,
  period: "journalier" | "hebdomadaire" | "mensuel",
) {
  if (period === "journalier") {
    return getPerDayPbBuys(pbLogs);
  } else if (period === "hebdomadaire") {
    return getPerWeekPbBuys(pbLogs);
  } else {
    return getPerMonthPbBuys(pbLogs);
  }
}

function getAverageBuyInPeriod(data: { date: string; total: number }[]) {
  return data.reduce((acc, curr) => acc + curr.total, 0) / data.length;
}

function isToday(date: Date) {
  if (
    date.getDate() === actualDate.value.getDate() &&
    date.getMonth() === actualDate.value.getMonth() &&
    date.getFullYear() === actualDate.value.getFullYear()
  ) {
    return true;
  }
  return false;
}

function isThisWeek(date: Date) {
  const week = dateConverter.getWeekNumber(date);
  const weekNow = dateConverter.getWeekNumber(actualDate.value);
  return (
    week === weekNow && date.getFullYear() === actualDate.value.getFullYear()
  );
}

function isThisMonth(date: Date) {
  if (
    date.getMonth() === actualDate.value.getMonth() &&
    date.getFullYear() === actualDate.value.getFullYear()
  ) {
    return true;
  }
  return false;
}

export default {
  getLineData,
  getAverageBuyInPeriod,
  buyInThisPeriod,
};
