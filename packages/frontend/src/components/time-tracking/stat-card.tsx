interface StatCardProps {
  value: string | number;
  label: string;
  className?: string;
}

export function StatCard({ value, label, className = '' }: StatCardProps) {
  return (
    <div className={`bg-zinc-900/50 p-4 rounded-lg border border-zinc-800 ${className}`}>
      <div className='text-2xl font-bold text-white'>{value}</div>
      <div className='text-sm text-zinc-400'>{label}</div>
    </div>
  );
}
