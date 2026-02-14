/**
 * Format seconds into a human-readable time string
 * @param seconds - Total seconds to format
 * @returns Formatted time string (e.g., "2h 15m 30s")
 */
export function formatDuration(seconds: number): string {
  if (seconds === 0) return '0s';

  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remainingSeconds = seconds % 60;

  const parts: string[] = [];

  if (hours > 0) {
    parts.push(`${hours}h`);
  }

  if (minutes > 0) {
    parts.push(`${minutes}m`);
  }

  if (remainingSeconds > 0 || parts.length === 0) {
    parts.push(`${remainingSeconds}s`);
  }

  return parts.join(' ');
}

/**
 * Format seconds into hours and minutes only (no seconds)
 * Rounds to the nearest minute for more user-friendly display
 * @param seconds - Total seconds to format
 * @returns Formatted time string (e.g., "2h 15m" or "15m")
 */
export function formatDurationMinutes(seconds: number): string {
  if (seconds === 0) return '0m';

  const hours = Math.floor(seconds / 3600);
  const minutes = Math.round(seconds / 60);

  if (hours > 0) {
    const remainingMinutes = minutes % 60;
    return remainingMinutes > 0 ? `${hours}h ${remainingMinutes}m` : `${hours}h`;
  }

  return `${minutes}m`;
}
