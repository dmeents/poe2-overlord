import { ChartBarIcon } from '@heroicons/react/24/outline';
import { memo } from 'react';
import { Card } from '@/components/ui/card/card';
import { useCharacter } from '@/contexts/CharacterContext';
import { useGameProcess } from '@/contexts/GameProcessContext';
import { useActiveLevelTime } from '@/hooks/useActiveLevelTime';
import { useLevelingStats } from '@/queries/leveling';
import { formatDuration } from '@/utils/format-duration';
import { formatXpAmount, formatXpRate } from '@/utils/format-xp';
import { LevelingChart } from '../leveling-chart/leveling-chart';
import { levelingStatsCardStyles as styles } from './leveling-stats-card.styles';

export const LevelingStatsCard = memo(function LevelingStatsCard() {
  const { activeCharacter } = useCharacter();
  const { gameRunning } = useGameProcess();
  const { data: stats } = useLevelingStats(activeCharacter?.id);

  // Backend computes is_actively_grinding from zone state — no need to re-derive it here.
  // gameRunning guards against stale cache keeping the timer alive after the process stops.
  const isTimerActive = !!stats?.is_actively_grinding && !!stats?.last_level_reached_at && gameRunning;

  const timeAtLevelSeconds = useActiveLevelTime({
    lastLevelTimestamp: stats?.last_level_reached_at ?? undefined,
    isActive: isTimerActive,
    activeSecondsAtLevel: stats?.active_seconds_at_level ?? 0,
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
        {/* Est. next level — primary metric */}
        <div className={styles.primarySection}>
          <div className={styles.primaryLabel}>
            Est. next level
          </div>
          {stats.estimated_seconds_to_next_level !== null ? (
            <div className={styles.primaryValue}>
              {formatDuration(stats.estimated_seconds_to_next_level)}
            </div>
          ) : (
            <div className={styles.primaryEmpty}>{hasHistory ? '—' : 'Need 2+ level-ups'}</div>
          )}
        </div>

        {/* Stats grid */}
        <div className={styles.statsGrid}>
          <div className={isTimerActive ? styles.statBox : styles.statBoxPaused}>
            <div className={styles.statLabel}>
              Time at level
              {!isTimerActive && <span className={styles.pausedTag}> (paused)</span>}
            </div>
            <div className={styles.statValue}>
              {stats.last_level_reached_at ? formatDuration(timeAtLevelSeconds) : '—'}
            </div>
          </div>
          <div className={styles.statBox}>
            <div className={styles.statLabel}>XP / hr</div>
            <div className={styles.statValue}>
              {hasXpRate ? formatXpRate(stats.xp_per_hour!) : hasHistory ? '—' : 'Need 2+ level-ups'}
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

        {/* Leveling chart */}
        {stats.chart_events.length >= 2 && (
          <>
            <div className={styles.divider} />
            <div className={styles.historyTitle}>XP / hr history</div>
            <LevelingChart data={stats.chart_events} />
          </>
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
                  {event.deaths_at_level > 0 && (
                    <span className={styles.historyDeaths}>☠ {event.deaths_at_level}</span>
                  )}
                  <span className={styles.historyXphr}>
                    {event.xp_per_hour !== null ? formatXpRate(event.xp_per_hour) : '—'}
                  </span>
                </div>
              ))}
            </div>
          </>
        )}
      </div>
    </Card>
  );
});
