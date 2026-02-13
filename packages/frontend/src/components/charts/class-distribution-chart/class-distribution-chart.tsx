import { UsersIcon } from '@heroicons/react/24/outline';
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from 'recharts';
import { getClassHexColor } from '../../../utils/class-colors';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';
import { EmptyState } from '../../ui/empty-state/empty-state';

interface ClassDistributionChartProps {
  classDistribution: Record<string, number>;
  className?: string;
}

interface ClassData {
  name: string;
  value: number;
  percentage: number;
  hexColor: string;
  [key: string]: string | number;
}

export function ClassDistributionChart({
  classDistribution,
  className = '',
}: ClassDistributionChartProps) {
  const totalCharacters = Object.values(classDistribution).reduce((sum, count) => sum + count, 0);

  const chartData: ClassData[] = Object.entries(classDistribution)
    .map(([className, count]) => {
      const percentage = totalCharacters > 0 ? (count / totalCharacters) * 100 : 0;
      const hexColor = getClassHexColor(className);

      return {
        name: className,
        value: count,
        percentage,
        hexColor,
      };
    })
    .sort((a, b) => b.value - a.value); // Sort by count descending

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
            {data.value} character{data.value !== 1 ? 's' : ''} ({data.percentage.toFixed(1)}%)
          </p>
        </div>
      );
    }
    return null;
  };

  if (chartData.length === 0) {
    return (
      <Card title="Classes" icon={<UsersIcon />} className={className}>
        <EmptyState
          icon={<UsersIcon className="w-8 h-8" />}
          title="No Class Data"
          description="Create characters to see class distribution"
        />
      </Card>
    );
  }

  return (
    <Card title="Classes" icon={<UsersIcon />} className={className}>
      <div className="space-y-6">
        <div className="relative flex items-center justify-center h-40">
          <ResponsiveContainer width="100%" height={160}>
            <PieChart>
              <Pie
                data={chartData}
                cx="50%"
                cy="50%"
                innerRadius={40}
                outerRadius={60}
                paddingAngle={1}
                dataKey="value"
                stroke="none"
                className="transition-all duration-300">
                {chartData.map(entry => (
                  <Cell
                    key={`cell-${entry.name}`}
                    fill={entry.hexColor}
                    className="transition-all duration-300 hover:opacity-80 cursor-pointer"
                  />
                ))}
              </Pie>
              <Tooltip content={<CustomTooltip />} />
            </PieChart>
          </ResponsiveContainer>
          <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
            <div className="text-2xl font-bold text-white">{chartData.length}</div>
            <div className="text-xs text-stone-400 uppercase tracking-wide">Classes</div>
          </div>
        </div>
        <div>
          {chartData.map(classItem => (
            <DataItem
              key={classItem.name}
              label={classItem.name}
              value={classItem.value}
              subValue={`${classItem.percentage.toFixed(1)}%`}
              color={classItem.hexColor}
            />
          ))}
        </div>
      </div>
    </Card>
  );
}
