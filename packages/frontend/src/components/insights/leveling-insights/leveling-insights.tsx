import { ChartBarIcon } from '@heroicons/react/24/outline';
import { getThemeHexColor } from '@poe2-overlord/theme';
import type { LevelEventResponse, LevelingStats } from '@/types/leveling';
import { formatDuration, formatDurationMinutes } from '@/utils/format-duration';
import { formatXpAmount, formatXpRate } from '@/utils/format-xp';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';
import { levelingInsightsStyles as s } from './leveling-insights.styles';

interface LevelingInsightsProps {
  events: LevelEventResponse[];
  currentLevel: number;
  liveStats?: LevelingStats;
}

export function LevelingInsights({ events, currentLevel, liveStats }: LevelingInsightsProps) {
  if (events.length === 0) {
    return (
      <Card title="Leveling Insights" icon={<ChartBarIcon />}>
        <div className={s.emptyText}>No level history yet</div>
      </Card>
    );
  }

  const totalDeaths = events.reduce((sum, e) => sum + e.deaths_at_level, 0);

  const eventsWithTime = events.filter(e => e.time_from_previous_level_seconds != null);
  const eventsWithXp = events.filter(e => e.xp_per_hour != null);

  const fastestLevel =
    eventsWithTime.length > 0
      ? eventsWithTime.reduce((min, e) =>
          (e.time_from_previous_level_seconds ?? 0) < (min.time_from_previous_level_seconds ?? 0)
            ? e
            : min,
        )
      : null;

  const slowestLevel =
    eventsWithTime.length > 0
      ? eventsWithTime.reduce((max, e) =>
          (e.time_from_previous_level_seconds ?? 0) > (max.time_from_previous_level_seconds ?? 0)
            ? e
            : max,
        )
      : null;

  const peakXpHr =
    eventsWithXp.length > 0
      ? eventsWithXp.reduce((max, e) => ((e.xp_per_hour ?? 0) > (max.xp_per_hour ?? 0) ? e : max))
      : null;

  const avgXpHr =
    eventsWithXp.length > 0
      ? eventsWithXp.reduce((sum, e) => sum + (e.xp_per_hour ?? 0), 0) / eventsWithXp.length
      : null;

  const totalActiveSeconds = events.reduce(
    (sum, e) => sum + (e.time_from_previous_level_seconds ?? 0),
    0,
  );

  return (
    <Card title="Leveling Insights" icon={<ChartBarIcon />} className="py-0">
      {liveStats && (
        <>
          {liveStats.estimated_seconds_to_next_level !== null && (
            <DataItem
              label="Est. next level"
              value={formatDuration(liveStats.estimated_seconds_to_next_level)}
              color={getThemeHexColor('ember-500')}
            />
          )}
          {liveStats.current_level < 100 && (
            <DataItem label="XP to next level" value={formatXpAmount(liveStats.xp_to_next_level)} />
          )}
          <div className={s.divider} />
        </>
      )}
      <DataItem label="Current Level" value={currentLevel} />
      <DataItem label="Levels Tracked" value={events.length} />
      <DataItem label="Total Deaths" value={totalDeaths} subValue="across all levels" />
      {totalActiveSeconds > 0 && (
        <DataItem
          label="Total Level Time"
          value={formatDurationMinutes(totalActiveSeconds)}
          subValue="active grinding only"
        />
      )}
      {peakXpHr && (
        <DataItem
          label="Peak XP / hr"
          value={formatXpRate(peakXpHr.xp_per_hour ?? 0)}
          subValue={`at level ${peakXpHr.level}`}
        />
      )}
      {avgXpHr != null && (
        <DataItem label="Avg XP / hr" value={formatXpRate(avgXpHr)} subValue="all levels" />
      )}
      {fastestLevel && (
        <DataItem
          label="Fastest Level"
          value={`Lv ${fastestLevel.level}`}
          subValue={formatDurationMinutes(fastestLevel.time_from_previous_level_seconds ?? 0)}
        />
      )}
      {slowestLevel && (
        <DataItem
          label="Slowest Level"
          value={`Lv ${slowestLevel.level}`}
          subValue={formatDurationMinutes(slowestLevel.time_from_previous_level_seconds ?? 0)}
        />
      )}
    </Card>
  );
}
