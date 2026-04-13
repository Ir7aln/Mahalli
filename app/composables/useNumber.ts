/**
 * Convert a value that may be a string (from rust_decimal::Decimal) to a number.
 * SeaORM's Decimal serializes to JSON strings, but i18n currency formatting expects numbers.
 */
export function toNumber(value: string | number | undefined | null): number {
  if (value === undefined || value === null) return 0;
  if (typeof value === "number") return Number.isFinite(value) ? value : 0;
  const parsed = parseFloat(value);
  return Number.isFinite(parsed) ? parsed : 0;
}
