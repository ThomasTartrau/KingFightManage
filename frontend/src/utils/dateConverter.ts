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

export default {
  timestampToDateString,
  timestampToDate,
  monthDayLeft,
  getActualWeek,
  getDayString,
  getWeekString,
  getMonthString,
};
