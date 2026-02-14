import { ChartPieIcon } from '@heroicons/react/24/outline';
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';
import type { CharacterData } from '@/types/character';
import { getActHexColor } from '@/utils/act-colors';
import { formatDuration } from '@/utils/format-duration';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { ChartTooltip, formatDurationTooltip } from '../chart-tooltip/chart-tooltip';
import { actChartStyles } from './act-distribution-chart.styles';

interface ActDistributionChartProps {
  character: CharacterData;
  className?: string;
}

interface ActData {
  name: string;
  time: number;
  percentage: number;
  hexColor: string;
}

export function ActDistributionChart({ character, className = '' }: ActDistributionChartProps) {
  // Extract act data from character summary
  const actData: ActData[] = [
    {
      name: 'Act 1',
      time: character.summary.play_time_act1,
      percentage: 0, // Will be calculated
      hexColor: getActHexColor('Act 1'),
    },
    {
      name: 'Act 2',
      time: character.summary.play_time_act2,
      percentage: 0, // Will be calculated
      hexColor: getActHexColor('Act 2'),
    },
    {
      name: 'Act 3',
      time: character.summary.play_time_act3,
      percentage: 0, // Will be calculated
      hexColor: getActHexColor('Act 3'),
    },
    {
      name: 'Act 4',
      time: character.summary.play_time_act4,
      percentage: 0, // Will be calculated
      hexColor: getActHexColor('Act 4'),
    },
    {
      name: 'Act 5',
      time: character.summary.play_time_act5,
      percentage: 0, // Will be calculated
      hexColor: getActHexColor('Act 5'),
    },
    {
      name: 'Interlude',
      time: character.summary.play_time_interlude,
      percentage: 0, // Will be calculated
      hexColor: getActHexColor('Interlude'),
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
    hexColor: act.hexColor,
  }));

  return (
    <Card title="Act Distribution" icon={<ChartPieIcon />} className={className}>
      <div className={actChartStyles.container}>
        <div className={actChartStyles.chartWrapper}>
          <div className={actChartStyles.chartBackdrop} />
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
                stroke="var(--color-stone-900)"
                strokeWidth={2}
                className={actChartStyles.pie}>
                {chartData.map(entry => (
                  <Cell
                    key={`cell-${entry.name}`}
                    fill={entry.hexColor}
                    className={actChartStyles.cell}
                  />
                ))}
              </Pie>
              <Tooltip
                content={
                  <ChartTooltip
                    formatContent={data => formatDurationTooltip(data, formatDuration)}
                  />
                }
              />
            </PieChart>
          </ResponsiveContainer>
          <div className={actChartStyles.centerStats}>
            <div className={actChartStyles.centerValue}>{activeActs.length}</div>
            <div className={actChartStyles.centerLabel}>Acts</div>
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
            className={actChartStyles.totalRow}
          />
        </div>
      </div>
    </Card>
  );
}
