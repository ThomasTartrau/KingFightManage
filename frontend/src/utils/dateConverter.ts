function timestampToDateString(timestamp: string): string {
  return new Date(timestamp).toLocaleDateString("fr-FR", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

function timestampToDate(timestamp: string): Date {
  return new Date(timestamp);
}

function monthDayLeft(date: Date): number {
  const daysInMonth = new Date(
    date.getFullYear(),
    date.getMonth() + 1,
    0
  ).getDate();
  return daysInMonth - date.getDate();
}

function getActualWeek(date: Date): number {
  const firstDayOfYear = new Date(date.getFullYear(), 0, 1);
  const pastDaysOfYear = (date.getTime() - firstDayOfYear.getTime()) / 86400000;
  return Math.ceil((pastDaysOfYear + firstDayOfYear.getDay() + 1) / 7);
}

function getDayString(date: Date): string {
  return date.toLocaleDateString("fr-FR");
}

function getWeekString(date: Date): string {
  const firstDayOfWeek = new Date(date.getFullYear(), 0, 1);
  const pastDaysOfYear = (date.getTime() - firstDayOfWeek.getTime()) / 86400000;
  const week = Math.ceil((pastDaysOfYear + firstDayOfWeek.getDay() + 1) / 7);
  return `Semaine ${week}`;
}

function getMonthString(date: Date): string {
  return date
    .toLocaleDateString("fr-FR", { month: "long" })
    .replace(/^\w/, (c) => c.toUpperCase());
}

function sortByDayString(
  data: { date: string; total: number }[]
): { date: string; total: number }[] {
  const sorted = data.sort((a, b) => {
    const dateA = convertSlashDateToDate(a.date);
    const dateB = convertSlashDateToDate(b.date);
    return dateA.getTime() - dateB.getTime();
  });
  return sorted;
}

function sortByWeekString(
  data: { date: string; total: number }[]
): { date: string; total: number }[] {
  return data.sort((a, b) => {
    const weekA = convertWeekStringToNumber(a.date);
    const weekB = convertWeekStringToNumber(b.date);
    return weekA - weekB;
  });
}

function sortByMonthString(
  data: { date: string; total: number }[]
): { date: string; total: number }[] {
  const monthMap: { [key: string]: number } = {
    Janvier: 1,
    Février: 2,
    Mars: 3,
    Avril: 4,
    Mai: 5,
    Juin: 6,
    Juillet: 7,
    Août: 8,
    Septembre: 9,
    Octobre: 10,
    Novembre: 11,
    Décembre: 12,
  };

  return data.sort((a, b) => monthMap[a.date] - monthMap[b.date]);
}

function convertSlashDateToDate(date: string): Date {
  const [day, month, year] = date.split("/").map(Number);
  return new Date(year, month - 1, day);
}

function convertWeekStringToNumber(date: string): number {
  const week = date.split(" ")[1];
  return parseInt(week);
}

export default {
  timestampToDateString,
  timestampToDate,
  monthDayLeft,
  getActualWeek,
  getDayString,
  getWeekString,
  getMonthString,
  sortByDayString,
  sortByWeekString,
  sortByMonthString,
};
