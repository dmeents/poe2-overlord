import { ChartPieIcon } from '@heroicons/react/24/outline';
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';
import type { CharacterData } from '@/types/character';
import { formatDuration } from '@/utils/format-duration';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';
import { EmptyState } from '../../ui/empty-state/empty-state';

interface ActDistributionChartProps {
  character: CharacterData;
  className?: string;
}

interface ActData {
  name: string;
  time: number;
  percentage: number;
  color: string;
  hexColor: string;
}

export function ActDistributionChart({ character, className = '' }: ActDistributionChartProps) {
  // Extract act data from character summary
  const actData: ActData[] = [
    {
      name: 'Act 1',
      time: character.summary.play_time_act1,
      percentage: 0, // Will be calculated
      color: 'bg-emerald-500',
      hexColor: '#10b981',
    },
    {
      name: 'Act 2',
      time: character.summary.play_time_act2,
      percentage: 0, // Will be calculated
      color: 'bg-blue-500',
      hexColor: '#3b82f6',
    },
    {
      name: 'Act 3',
      time: character.summary.play_time_act3,
      percentage: 0, // Will be calculated
      color: 'bg-purple-500',
      hexColor: '#8b5cf6',
    },
    {
      name: 'Act 4',
      time: character.summary.play_time_act4,
      percentage: 0, // Will be calculated
      color: 'bg-amber-500',
      hexColor: '#f59e0b',
    },
    {
      name: 'Act 5',
      time: character.summary.play_time_act5,
      percentage: 0, // Will be calculated
      color: 'bg-pink-500',
      hexColor: '#ec4899',
    },
    {
      name: 'Interlude',
      time: character.summary.play_time_interlude,
      percentage: 0, // Will be calculated
      color: 'bg-red-500',
      hexColor: '#ef4444',
    },
  ];

  // Calculate total time for main acts only
  const totalTime = actData.reduce((sum, act) => sum + act.time, 0);

  // Calculate percentages
  actData.forEach(act => {
    act.percentage = totalTime > 0 ? (act.time / totalTime) * 100 : 0;
  });

  // Filter out acts with no time
  const activeActs = actData.filter(act => act.time > 0);

  if (activeActs.length === 0) {
    return (
      <Card title="Act Distribution" icon={<ChartPieIcon />} className={className}>
        <EmptyState
          icon={<ChartPieIcon className="w-8 h-8" />}
          title="No Act Data"
          description="Start playing to see act distribution"
        />
      </Card>
    );
  }

  // Prepare data for Recharts
  const chartData = activeActs.map(act => ({
    name: act.name,
    value: act.time,
    percentage: act.percentage,
    color: act.hexColor,
  }));

  // Custom tooltip component
  // z-20: Tooltips (see patterns.md for z-index scale)
  const CustomTooltip = ({
    active,
    payload,
  }: {
    active?: boolean;
    payload?: Array<{
      payload: { name: string; value: number; percentage: number };
    }>;
  }) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className="bg-stone-800 border border-stone-700 rounded-lg p-3 shadow-lg relative z-20">
          <p className="text-white font-medium">{data.name}</p>
          <p className="text-stone-300 text-sm">
            {formatDuration(data.value)} ({data.percentage.toFixed(1)}%)
          </p>
        </div>
      );
    }
    return null;
  };

  return (
    <Card title="Act Distribution" icon={<ChartPieIcon />} className={className}>
      <div className="space-y-6">
        <div className="relative flex items-center justify-center h-48">
          <ResponsiveContainer width="100%" height={200}>
            <PieChart>
              <Pie
                data={chartData}
                cx="50%"
                cy="50%"
                innerRadius={60}
                outerRadius={80}
                paddingAngle={2}
                dataKey="value"
                stroke="none"
                className="transition-all duration-300">
                {chartData.map(entry => (
                  <Cell
                    key={`cell-${entry.name}`}
                    fill={entry.color}
                    className="transition-all duration-300 hover:opacity-80 cursor-pointer"
                  />
                ))}
              </Pie>
              <Tooltip content={<CustomTooltip />} />
            </PieChart>
          </ResponsiveContainer>
          <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
            <div className="text-2xl font-bold text-white">{activeActs.length}</div>
            <div className="text-xs text-stone-400 uppercase tracking-wide">Acts</div>
          </div>
        </div>
        <div>
          {activeActs.map(act => (
            <DataItem
              key={act.name}
              label={act.name}
              value={formatDuration(act.time)}
              subValue={`${act.percentage.toFixed(1)}%`}
              color={act.hexColor}
            />
          ))}
          <DataItem
            label="Total Campaign Time"
            value={formatDuration(totalTime)}
            className="font-medium"
          />
        </div>
      </div>
    </Card>
  );
}
