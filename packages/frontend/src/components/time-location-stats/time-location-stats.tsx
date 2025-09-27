import type { ZoneStats } from '@/types';
import { TimeDisplay } from '../time-display';
import { timeLocationStatsStyles } from './time-location-stats.styles';

interface LocationStatsProps {
  stats: ZoneStats[];
  className?: string;
}

export function LocationStats({ stats, className = '' }: LocationStatsProps) {
  if (stats.length === 0) {
    return (
      <div className={`${timeLocationStatsStyles.container} ${className}`}>
        <h3 className={timeLocationStatsStyles.title}>Location Statistics</h3>
        <div className={timeLocationStatsStyles.emptyState}>
          No location data available
        </div>
      </div>
    );
  }

  // Sort stats by duration (descending)
  const sortedStats = [...stats].sort((a, b) => b.duration - a.duration);

  return (
    <div className={`${timeLocationStatsStyles.container} ${className}`}>
      <h3 className={timeLocationStatsStyles.titleWithMargin}>
        Location Statistics
      </h3>
      <div className={timeLocationStatsStyles.statsContainer}>
        {sortedStats.map(stat => (
          <div
            key={stat.location_id}
            className={timeLocationStatsStyles.statItem}
          >
            <div className={timeLocationStatsStyles.statHeader}>
              <div className={timeLocationStatsStyles.statInfo}>
                <span className={timeLocationStatsStyles.statType}>
                  {stat.location_type}
                </span>
                <span className={timeLocationStatsStyles.statName}>
                  {stat.location_name}
                </span>
              </div>
              <div className={timeLocationStatsStyles.statTime}>
                <TimeDisplay seconds={stat.duration} showSeconds={false} />
              </div>
            </div>

            <div className={timeLocationStatsStyles.statGrid}>
              <div>
                <span className={timeLocationStatsStyles.statLabel}>
                  Visits
                </span>
                <span className={timeLocationStatsStyles.statValue}>
                  {stat.visits}
                </span>
              </div>
              <div>
                <span className={timeLocationStatsStyles.statLabel}>
                  Deaths
                </span>
                <span className={timeLocationStatsStyles.statValue}>
                  {stat.deaths}
                </span>
              </div>
              <div>
                <span className={timeLocationStatsStyles.statLabel}>
                  Last Visit
                </span>
                <span className={timeLocationStatsStyles.statValue}>
                  {stat.last_visited
                    ? new Date(stat.last_visited).toLocaleDateString()
                    : 'Never'}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
