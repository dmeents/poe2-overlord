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
import { formatXpAmount, formatXpRate } from '@/utils/format-xp';
import { levelHistoryChartStyles as styles } from './level-history-chart.styles';

interface LevelHistoryChartProps {
  events: LevelEventResponse[];
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
        <p className="text-ember-400 text-xs">{formatXpRate(xpItem.value)}</p>
      )}
      {timeItem?.value != null && (
        <p className="text-molten-300 text-xs">{formatDurationMinutes(timeItem.value)}</p>
      )}
      {deathsItem?.value != null && deathsItem.value > 0 && (
        <p className="text-blood-400 text-xs">
          ☠ {deathsItem.value} death{deathsItem.value !== 1 ? 's' : ''}
        </p>
      )}
    </div>
  );
}

export const LevelHistoryChart = memo(function LevelHistoryChart({
  events,
}: LevelHistoryChartProps) {
  const [showTime, setShowTime] = useState(false);

  if (events.length < 2) return null;

  const hasDeaths = events.some(e => e.deaths_at_level > 0);
  const hasTime = events.some(e => e.time_from_previous_level_seconds != null);

  const rightMargin = hasDeaths ? 28 : 0;

  return (
    <div>
      <div className={styles.header}>
        <span className={styles.title}>XP / hr history</span>
        {hasTime && (
          <div className={styles.toggleGroup}>
            <button
              className={`${styles.toggleButton} ${showTime ? styles.toggleActive : styles.toggleInactive}`}
              onClick={() => setShowTime(v => !v)}>
              Time / level
            </button>
          </div>
        )}
      </div>
      <div className={styles.container}>
        <ResponsiveContainer width="100%" height={280}>
          <ComposedChart data={events} margin={{ top: 4, right: rightMargin, bottom: 0, left: 0 }}>
            <XAxis
              dataKey="level"
              tick={{ fill: 'var(--color-stone-500)', fontSize: 10 }}
              tickLine={false}
              axisLine={false}
            />
            <YAxis
              yAxisId="xp"
              orientation="left"
              tick={{ fill: 'var(--color-stone-500)', fontSize: 10 }}
              tickLine={false}
              axisLine={false}
              tickFormatter={v => formatXpAmount(v)}
              width={40}
            />
            {hasDeaths && (
              <YAxis
                yAxisId="deaths"
                orientation="right"
                tick={{ fill: 'var(--color-stone-500)', fontSize: 10 }}
                tickLine={false}
                axisLine={false}
                allowDecimals={false}
                width={20}
              />
            )}
            <Tooltip content={<CustomTooltip />} />
            {(showTime || hasDeaths) && (
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
            {showTime && (
              <Line
                yAxisId="xp"
                type="monotone"
                dataKey="time_from_previous_level_seconds"
                name="Time (s)"
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
