export interface StatItem {
  value: string | number;
  label: string;
  subtext?: string;
}

export interface StatGridProps {
  stats: StatItem[];
  columns?: 2 | 3 | 4;
  className?: string;
}
