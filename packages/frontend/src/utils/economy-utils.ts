/**
 * Calculate items sold per hour from volume and primary value.
 * Includes guard checks for division by zero and invalid values.
 *
 * @param volume - Total volume of items
 * @param primaryValue - Primary value for calculation
 * @returns Formatted string with 2 decimal places
 */
export function calculateItemsSoldPerHour(volume: number, primaryValue: number): string {
  if (primaryValue === 0 || !Number.isFinite(primaryValue)) {
    return '0.00';
  }
  const itemsSold = volume / primaryValue;
  if (!Number.isFinite(itemsSold)) {
    return '0.00';
  }
  return itemsSold.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}
