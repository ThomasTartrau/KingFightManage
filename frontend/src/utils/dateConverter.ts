const months = new Map([
  ["Janvier", 1],
  ["Février", 2],
  ["Mars", 3],
  ["Avril", 4],
  ["Mai", 5],
  ["Juin", 6],
  ["Juillet", 7],
  ["Août", 8],
  ["Septembre", 9],
  ["Octobre", 10],
  ["Novembre", 11],
  ["Décembre", 12],
]);

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
  return `${date.getFullYear()} - Semaine ${getWeekNumber(date)}`;
}

function getMonthString(date: Date): string {
  return `${date.toLocaleDateString("fr-FR", { month: "long" }).replace(/^\w/, (c) => c.toUpperCase())} - ${date.getFullYear()}`;
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
    const yearWeekA = convertWeekStringToNumber(a.date);
    const yearWeekB = convertWeekStringToNumber(b.date);
    return yearWeekA - yearWeekB;
  });
}

function sortByMonthString(
  data: { date: string; total: number }[]
): { date: string; total: number }[] {
  return data.sort(
    (a, b) =>
      convertMonthStringToNumber(a.date) - convertMonthStringToNumber(b.date)
  );
}

function convertSlashDateToDate(date: string): Date {
  const [day, month, year] = date.split("/").map(Number);
  return new Date(year, month - 1, day);
}

function convertWeekStringToNumber(date: string): number {
  const year = Number.parseInt(date.split(" ")[0]);
  const week = Number.parseInt(date.split(" ")[3]);
  return year * 100 + week;
}

function convertMonthStringToNumber(date: string): number {
  const month = Number.parseInt(
    months.get(date.split(" ")[0])?.toString() ?? "0"
  );
  const year = Number.parseInt(date.split(" ")[2]);
  return year * 100 + month;
}

function getWeekNumber(date: Date): number {
  const tempDate = new Date(date.valueOf());
  const dayNum = (date.getDay() + 6) % 7;
  tempDate.setDate(tempDate.getDate() - dayNum + 3);
  const firstThursday = tempDate.valueOf();
  tempDate.setMonth(0, 1);

  if (tempDate.getDay() !== 4) {
    tempDate.setMonth(0, 1 + ((4 - tempDate.getDay() + 7) % 7));
  }

  return 1 + Math.ceil((firstThursday - tempDate.valueOf()) / 604800000);
}

function convertSecondToTime(second: number): string {
  const hours = Math.floor(second / 3600);
  const minutes = Math.floor((second % 3600) / 60);
  const seconds = second % 60;
  return `${hours > 0 ? `${hours}h ` : ""}${minutes > 0 ? `${minutes}m ` : ""}${seconds > 0 ? `${seconds}s` : ""}`;
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
  getWeekNumber,
  convertSecondToTime,
};
