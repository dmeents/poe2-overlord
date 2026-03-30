import { memo } from 'react';
import type { LevelEventResponse, LevelingStats } from '@/types/leveling';
import { formatDuration } from '@/utils/format-duration';
import { formatXpAmount, formatXpRate } from '@/utils/format-xp';
import { levelHistoryTableStyles as styles } from './level-history-table.styles';

interface LevelHistoryTableProps {
  events: LevelEventResponse[];
  liveStats?: LevelingStats;
  currentTimeSeconds?: number;
}

function formatTimestamp(iso: string): string {
  const date = new Date(iso);
  return date.toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export const LevelHistoryTable = memo(function LevelHistoryTable({
  events,
  liveStats,
  currentTimeSeconds,
}: LevelHistoryTableProps) {
  // Build a map of level → time spent AT that level.
  // event[N].time_from_previous_level_seconds is the time spent at level N-1,
  // so "time at level N" = event[N+1].time_from_previous_level_seconds.
  const timeAtLevel = new Map<number, number | null>();
  for (const event of events) {
    if (event.time_from_previous_level_seconds != null) {
      timeAtLevel.set(event.level - 1, event.time_from_previous_level_seconds);
    }
  }

  return (
    <div className={styles.wrapper}>
      <table className={styles.table}>
        <thead className={styles.thead}>
          <tr>
            <th className={styles.th}>Level</th>
            <th className={styles.th}>Reached At</th>
            <th className={styles.thRight}>Time at Level</th>
            <th className={styles.thRight}>Deaths</th>
            <th className={styles.thRight}>Effective XP</th>
            <th className={styles.thRight}>XP / hr</th>
          </tr>
        </thead>
        <tbody className={styles.tbody}>
          {/* Current (in-progress) level row */}
          {liveStats && (
            <tr className={styles.currentRow}>
              <td className={styles.currentTd}>
                <span className={styles.currentLevelBadge}>{liveStats.current_level}</span>
                <span className={styles.currentLiveDot} />
              </td>
              <td className={styles.currentTd}>
                {liveStats.last_level_reached_at
                  ? formatTimestamp(liveStats.last_level_reached_at)
                  : '—'}
              </td>
              <td className={styles.currentTdRight}>
                {currentTimeSeconds !== undefined && currentTimeSeconds > 0
                  ? formatDuration(currentTimeSeconds)
                  : '—'}
              </td>
              <td className={styles.currentTdRight}>
                {liveStats.deaths_at_current_level > 0 ? (
                  <span className={styles.deathsValue}>☠ {liveStats.deaths_at_current_level}</span>
                ) : (
                  <span className="text-ember-900">0</span>
                )}
              </td>
              <td className={styles.currentTdRight}>
                <span className={styles.currentInProgress}>In Progress</span>
              </td>
              <td className={styles.currentTdRight}>
                {liveStats.xp_per_hour !== null ? (
                  <span className={styles.xphrValue}>{formatXpRate(liveStats.xp_per_hour)}</span>
                ) : (
                  '—'
                )}
              </td>
            </tr>
          )}

          {events.length === 0 ? (
            <tr>
              <td colSpan={6} className={styles.emptyRow}>
                No level-up history recorded yet
              </td>
            </tr>
          ) : (
            // Render descending (highest level first) so most recent is at the top.
            // Exclude the current level event when a live row is shown — it's already represented there.
            [...events]
              .filter(event => !liveStats || event.level !== liveStats.current_level)
              .reverse()
              .map(event => (
                <tr key={event.level} className={styles.tr}>
                  <td className={styles.td}>
                    <span className={styles.levelBadge}>{event.level}</span>
                  </td>
                  <td className={styles.td}>{formatTimestamp(event.reached_at)}</td>
                  <td className={styles.tdRight}>
                    {timeAtLevel.get(event.level) != null
                      ? formatDuration(timeAtLevel.get(event.level)!)
                      : '—'}
                  </td>
                  <td className={styles.tdRight}>
                    {event.deaths_at_level > 0 ? (
                      <span className={styles.deathsValue}>☠ {event.deaths_at_level}</span>
                    ) : (
                      <span className="text-stone-600">0</span>
                    )}
                  </td>
                  <td className={styles.tdRight}>
                    {event.effective_xp_earned != null
                      ? formatXpAmount(event.effective_xp_earned)
                      : '—'}
                  </td>
                  <td className={styles.tdRight}>
                    {event.xp_per_hour != null ? (
                      <span className={styles.xphrValue}>{formatXpRate(event.xp_per_hour)}</span>
                    ) : (
                      '—'
                    )}
                  </td>
                </tr>
              ))
          )}
        </tbody>
      </table>
    </div>
  );
});
