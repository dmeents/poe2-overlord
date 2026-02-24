/**
 * Formats an XP-per-hour rate for display.
 * Examples: 1_200_000 → "1.2M XP/hr", 524_000 → "524K XP/hr", 900 → "900 XP/hr"
 */
export function formatXpRate(xpPerHour: number): string {
  return `${formatXpAmount(xpPerHour)} XP/hr`;
}

/**
 * Formats an XP amount with K/M suffix.
 * Examples: 1_200_000 → "1.2M", 524_000 → "524K", 900 → "900"
 */
export function formatXpAmount(xp: number): string {
  if (xp >= 1_000_000) {
    const millions = xp / 1_000_000;
    return millions % 1 === 0 ? `${millions}M` : `${millions.toFixed(1)}M`;
  }
  if (xp >= 1_000) {
    const thousands = xp / 1_000;
    return thousands % 1 === 0 ? `${thousands}K` : `${thousands.toFixed(0)}K`;
  }
  return `${Math.round(xp)}`;
}
