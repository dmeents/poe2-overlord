import type { LocationStats } from '@/types';
import { TimeDisplay } from '../time-display';
import { timeLocationStatsStyles } from './time-location-stats.styles';

interface LocationStatsProps {
  stats: LocationStats[];
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

  // Sort stats by total time (descending)
  const sortedStats = [...stats].sort(
    (a, b) => b.total_time_seconds - a.total_time_seconds
  );

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
                <TimeDisplay
                  seconds={stat.total_time_seconds}
                  showSeconds={false}
                />
              </div>
            </div>

            <div className={timeLocationStatsStyles.statGrid}>
              <div>
                <span className={timeLocationStatsStyles.statLabel}>
                  Visits
                </span>
                <span className={timeLocationStatsStyles.statValue}>
                  {stat.total_visits}
                </span>
              </div>
              <div>
                <span className={timeLocationStatsStyles.statLabel}>
                  Avg Session
                </span>
                <span className={timeLocationStatsStyles.statValue}>
                  <TimeDisplay
                    seconds={Math.round(stat.average_session_seconds)}
                  />
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
