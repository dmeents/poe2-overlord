import { memo } from 'react';
import type { LevelEventResponse } from '@/types/leveling';
import { formatDuration } from '@/utils/format-duration';
import { formatXpAmount, formatXpRate } from '@/utils/format-xp';
import { levelHistoryTableStyles as styles } from './level-history-table.styles';

interface LevelHistoryTableProps {
  events: LevelEventResponse[];
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

export const LevelHistoryTable = memo(function LevelHistoryTable({ events }: LevelHistoryTableProps) {
  return (
    <div className={styles.wrapper}>
      <table className={styles.table}>
        <thead className={styles.thead}>
          <tr>
            <th className={styles.th}>Level</th>
            <th className={styles.th}>Reached At</th>
            <th className={styles.thRight}>Time to Level</th>
            <th className={styles.thRight}>Deaths</th>
            <th className={styles.thRight}>Effective XP</th>
            <th className={styles.thRight}>XP / hr</th>
          </tr>
        </thead>
        <tbody className={styles.tbody}>
          {events.length === 0 ? (
            <tr>
              <td colSpan={6} className={styles.emptyRow}>
                No level-up history recorded yet
              </td>
            </tr>
          ) : (
            // Render descending (highest level first) so most recent is at the top
            [...events].reverse().map(event => (
              <tr key={event.level} className={styles.tr}>
                <td className={styles.td}>
                  <span className={styles.levelBadge}>{event.level}</span>
                </td>
                <td className={styles.td}>{formatTimestamp(event.reached_at)}</td>
                <td className={styles.tdRight}>
                  {event.time_from_previous_level_seconds != null
                    ? formatDuration(event.time_from_previous_level_seconds)
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
