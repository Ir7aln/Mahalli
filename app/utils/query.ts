import type { LocationQueryValue, LocationQueryValueRaw } from "vue-router";

export function firstQueryValue(value: LocationQueryValue | LocationQueryValue[] | undefined) {
  if (Array.isArray(value)) return value[0];
  return value ?? undefined;
}

export function queryString(value: LocationQueryValue | LocationQueryValue[] | undefined) {
  const v = firstQueryValue(value);
  return v == null ? "" : String(v);
}

export function queryNumber(
  value: LocationQueryValue | LocationQueryValue[] | undefined,
  fallback: number,
) {
  const v = firstQueryValue(value);
  const n = Number(v);
  return Number.isFinite(n) ? n : fallback;
}

export function queryBoolean(
  value: LocationQueryValue | LocationQueryValue[] | undefined,
  fallback?: boolean,
) {
  const v = firstQueryValue(value);
  if (v == null || v === "") return fallback;
  if (v === "true") return true;
  if (v === "false") return false;
  return fallback;
}

export function toQueryValueRaw(value: string | number | null): LocationQueryValueRaw | undefined {
  if (value === null) return undefined;
  return value;
}
