import { memo, useState } from 'react';
import {
  Bar,
  ComposedChart,
  Legend,
  Line,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';
import type { LevelEventResponse } from '@/types/leveling';
import { formatDurationMinutes } from '@/utils/format-duration';
import { formatXpAmount } from '@/utils/format-xp';
import { levelHistoryChartStyles as styles } from './level-history-chart.styles';

interface LevelHistoryChartProps {
  events: LevelEventResponse[];
  /** When true the time/level overlay is always shown and the toggle button is hidden. */
  alwaysShowTime?: boolean;
}

interface TooltipPayloadItem {
  dataKey: string;
  value: number | null;
  color: string;
}

interface CustomTooltipProps {
  active?: boolean;
  payload?: TooltipPayloadItem[];
  label?: number;
}

function CustomTooltip({ active, payload, label }: CustomTooltipProps) {
  if (!active || !payload || !payload.length) return null;

  const xpItem = payload.find(p => p.dataKey === 'xp_per_hour');
  const deathsItem = payload.find(p => p.dataKey === 'deaths_at_level');
  const timeItem = payload.find(p => p.dataKey === 'time_from_previous_level_seconds');

  return (
    <div className="bg-stone-800 border border-stone-700 rounded-lg p-2.5 card-shadow z-20">
      <p className="text-molten-400 font-semibold text-xs mb-1">Level {label}</p>
      {xpItem?.value != null && (
        <p className="text-ember-400 text-xs">XP/hr: {formatXpAmount(xpItem.value)}</p>
      )}
      {timeItem?.value != null && (
        <p className="text-molten-300 text-xs">TTL: {formatDurationMinutes(timeItem.value)}</p>
      )}
      {deathsItem?.value != null && (
        <p className="text-blood-400 text-xs">Deaths: {deathsItem.value}</p>
      )}
    </div>
  );
}

function formatTimeAxis(seconds: number): string {
  if (seconds === 0) return '0';
  if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
  return `${(seconds / 3600).toFixed(1)}h`;
}

export const LevelHistoryChart = memo(function LevelHistoryChart({
  events,
  alwaysShowTime = false,
}: LevelHistoryChartProps) {
  const [showTime, setShowTime] = useState(alwaysShowTime);

  if (events.length < 2) return null;

  const hasDeaths = events.some(e => e.deaths_at_level > 0);
  const hasTime = events.some(e => e.time_from_previous_level_seconds != null);
  const timeVisible = showTime && hasTime;

  return (
    <div>
      {hasTime && !alwaysShowTime && (
        <div className={styles.header}>
          <div className={styles.toggleGroup}>
            <button
              className={`${styles.toggleButton} ${showTime ? styles.toggleActive : styles.toggleInactive}`}
              onClick={() => setShowTime(v => !v)}>
              Time / level
            </button>
          </div>
        </div>
      )}
      <div className={styles.container}>
        <ResponsiveContainer width="100%" height={280}>
          <ComposedChart data={events} margin={{ top: 4, right: 16, bottom: 0, left: 0 }}>
            <XAxis
              dataKey="level"
              tick={{ fill: 'var(--color-stone-500)', fontSize: 10 }}
              tickLine={false}
              axisLine={false}
            />
            {/* Left axis: XP/hr — log scale so early levels aren't crushed by late-game values */}
            <YAxis
              yAxisId="xp"
              orientation="left"
              scale="sqrt"
              domain={['auto', 'auto']}
              tickCount={8}
              tick={{ fill: 'var(--color-ember-400)', fontSize: 10 }}
              tickLine={false}
              axisLine={false}
              tickFormatter={v => formatXpAmount(v)}
              width={40}
            />
            {/* Right axis: time per level — log scale, only rendered when visible */}
            {timeVisible && (
              <YAxis
                yAxisId="time"
                orientation="right"
                scale="sqrt"
                domain={['auto', 'auto']}
                tickCount={8}
                tick={{ fill: 'var(--color-molten-500)', fontSize: 10 }}
                tickLine={false}
                axisLine={false}
                tickFormatter={formatTimeAxis}
                width={36}
              />
            )}
            {/* Hidden axis for deaths bars — keeps bar scaling independent of XP/hr scale */}
            {hasDeaths && <YAxis yAxisId="deaths" orientation="right" width={0} hide />}
            <Tooltip content={<CustomTooltip />} />
            {(timeVisible || hasDeaths) && (
              <Legend
                wrapperStyle={{ fontSize: 10, color: 'var(--color-stone-400)' }}
                iconSize={8}
              />
            )}
            <Line
              yAxisId="xp"
              type="monotone"
              dataKey="xp_per_hour"
              name="XP/hr"
              stroke="var(--color-ember-400)"
              strokeWidth={2}
              dot={false}
              connectNulls
            />
            {timeVisible && (
              <Line
                yAxisId="time"
                type="monotone"
                dataKey="time_from_previous_level_seconds"
                name="Time / level"
                stroke="var(--color-molten-400)"
                strokeWidth={1.5}
                strokeDasharray="4 2"
                dot={false}
                connectNulls
              />
            )}
            {hasDeaths && (
              <Bar
                yAxisId="deaths"
                dataKey="deaths_at_level"
                name="Deaths"
                fill="var(--color-blood-400)"
                opacity={0.7}
                maxBarSize={10}
                radius={[2, 2, 0, 0]}
              />
            )}
          </ComposedChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
});
