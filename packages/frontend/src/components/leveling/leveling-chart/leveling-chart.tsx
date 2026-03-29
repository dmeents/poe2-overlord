import { memo } from 'react';
import { Bar, ComposedChart, Line, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts';
import type { LevelEventResponse } from '@/types/leveling';
import { formatXpAmount } from '@/utils/format-xp';
import { levelingChartStyles as styles } from './leveling-chart.styles';

interface LevelingChartProps {
  data: LevelEventResponse[];
}

interface TooltipPayloadItem {
  dataKey: string;
  value: number | null;
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

  return (
    <div className="bg-stone-800 border border-stone-700 rounded-lg p-2.5 card-shadow z-20">
      <p className="text-molten-400 font-semibold text-xs mb-1">Level {label}</p>
      {xpItem?.value != null && (
        <p className="text-ember-400 text-xs">XP/hr: {formatXpAmount(xpItem.value)}</p>
      )}
      {deathsItem?.value != null && (
        <p className="text-blood-400 text-xs">Deaths: {deathsItem.value}</p>
      )}
    </div>
  );
}

export const LevelingChart = memo(function LevelingChart({ data }: LevelingChartProps) {
  if (data.length < 2) return null;

  const recent = data.slice(-20);
  const hasDeaths = recent.some(d => d.deaths_at_level > 0);

  return (
    <div className={styles.container}>
      <ResponsiveContainer width="100%" height={120}>
        <ComposedChart
          data={recent}
          margin={{ top: 4, right: hasDeaths ? 28 : 0, bottom: 0, left: 0 }}>
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
            width={36}
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
          <Line
            yAxisId="xp"
            type="monotone"
            dataKey="xp_per_hour"
            stroke="var(--color-ember-400)"
            strokeWidth={2}
            dot={false}
            connectNulls
          />
          {hasDeaths && (
            <Bar
              yAxisId="deaths"
              dataKey="deaths_at_level"
              fill="var(--color-blood-400)"
              opacity={0.7}
              maxBarSize={8}
              radius={[2, 2, 0, 0]}
            />
          )}
        </ComposedChart>
      </ResponsiveContainer>
    </div>
  );
});
