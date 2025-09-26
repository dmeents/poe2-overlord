import { timeStatCardStyles } from './time-stat-card.styles';

interface StatCardProps {
  value: string | number;
  label: string;
  className?: string;
}

export function StatCard({ value, label, className = '' }: StatCardProps) {
  return (
    <div className={`${timeStatCardStyles.container} ${className}`}>
      <div className={timeStatCardStyles.value}>{value}</div>
      <div className={timeStatCardStyles.label}>{label}</div>
    </div>
  );
}
