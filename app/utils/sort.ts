export type SortDirection = "asc" | "desc";

export type SortValue = string | number | Date | null | undefined;

function normalizeSortValue(value: SortValue) {
  if (value == null) return "";
  if (value instanceof Date) return value.getTime();
  return value;
}

export function sortItems<T>(
  items: T[],
  direction: SortDirection,
  getter: (item: T) => SortValue,
) {
  const factor = direction === "asc" ? 1 : -1;

  return [...items].sort((left, right) => {
    const a = normalizeSortValue(getter(left));
    const b = normalizeSortValue(getter(right));

    if (typeof a === "number" && typeof b === "number") {
      return (a - b) * factor;
    }

    return String(a).localeCompare(String(b), undefined, {
      numeric: true,
      sensitivity: "base",
    }) * factor;
  });
}
