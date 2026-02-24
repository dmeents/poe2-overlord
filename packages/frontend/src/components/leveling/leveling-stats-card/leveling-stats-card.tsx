import { ChartBarIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import { Card } from '@/components/ui/card/card';
import { useCharacter } from '@/contexts/CharacterContext';
import { useElapsedTime } from '@/hooks/useElapsedTime';
import { useLevelingStats } from '@/queries/leveling';
import { formatDuration } from '@/utils/format-duration';
import { formatXpAmount, formatXpRate } from '@/utils/format-xp';
import { levelingStatsCardStyles as styles } from './leveling-stats-card.styles';

export const LevelingStatsCard = memo(function LevelingStatsCard() {
  const { activeCharacter } = useCharacter();
  const { data: stats } = useLevelingStats(activeCharacter?.id);

  // Live "time at current level" counter
  const timeAtLevelSeconds = useElapsedTime({
    entryTimestamp: stats?.last_level_reached_at ?? undefined,
    baseDuration: 0,
    isActive: !!stats?.last_level_reached_at,
  });

  if (!activeCharacter) {
    return (
      <Card title="Leveling" icon={<ChartBarIcon />}>
        <div className={styles.emptyState}>Select a character to track leveling</div>
      </Card>
    );
  }

  if (!stats) {
    return (
      <Card title="Leveling" icon={<ChartBarIcon />} accentColor="ember">
        <div className={styles.emptyState}>No leveling data yet</div>
      </Card>
    );
  }

  const hasXpRate = stats.xp_per_hour !== null;
  const hasHistory = stats.recent_events.length >= 2;

  return (
    <Card title="Leveling" icon={<ChartBarIcon />} accentColor="ember">
      <div className={styles.container}>
        {/* XP/hr — primary metric */}
        <div className={styles.xpRateSection}>
          <div className={styles.xpRateLabel}>XP / hr</div>
          {hasXpRate ? (
            <div className={styles.xpRateValue}>{formatXpRate(stats.xp_per_hour!)}</div>
          ) : (
            <div className={styles.xpRateEmpty}>{hasHistory ? '—' : 'Need 2+ level-ups'}</div>
          )}
        </div>

        {/* Stats grid */}
        <div className={styles.statsGrid}>
          <div className={styles.statBox}>
            <div className={styles.statLabel}>Time at level</div>
            <div className={styles.statValue}>
              {stats.last_level_reached_at ? formatDuration(timeAtLevelSeconds) : '—'}
            </div>
          </div>
          <div className={styles.statBox}>
            <div className={styles.statLabel}>Est. next level</div>
            <div className={styles.statValue}>
              {stats.estimated_seconds_to_next_level !== null
                ? formatDuration(stats.estimated_seconds_to_next_level)
                : '—'}
            </div>
          </div>
          <div className={styles.statBox}>
            <div className={styles.statLabel}>Deaths this level</div>
            <div
              className={
                stats.deaths_at_current_level > 0 ? styles.statValueHighlight : styles.statValue
              }>
              {stats.deaths_at_current_level}
            </div>
          </div>
          <div className={styles.statBox}>
            <div className={styles.statLabel}>Levels / hr</div>
            <div className={styles.statValue}>{stats.levels_gained_last_hour}</div>
          </div>
        </div>

        {/* XP to next level */}
        {stats.current_level < 100 && (
          <div className={styles.statBox + ' mb-4'}>
            <div className={styles.statLabel}>XP to next level</div>
            <div className={styles.statValue}>{formatXpAmount(stats.xp_to_next_level)}</div>
          </div>
        )}

        {/* Recent level history */}
        {stats.recent_events.length > 0 && (
          <>
            <div className={styles.divider} />
            <div className={styles.historyTitle}>Recent levels</div>
            <div className={styles.historyList}>
              {stats.recent_events.map(event => (
                <div key={event.level} className={styles.historyRow}>
                  <span className={styles.historyLevel}>Lv {event.level}</span>
                  <span className={styles.historyTime}>
                    {event.time_from_previous_level_seconds !== null
                      ? formatDuration(event.time_from_previous_level_seconds)
                      : '—'}
                  </span>
                  <span className={styles.historyXphr}>
                    {event.xp_per_hour !== null ? formatXpRate(event.xp_per_hour) : '—'}
                  </span>
                  {event.deaths_at_level > 0 && (
                    <span className={styles.historyDeaths}>☠ {event.deaths_at_level}</span>
                  )}
                </div>
              ))}
            </div>
          </>
        )}
      </div>
    </Card>
  );
});
