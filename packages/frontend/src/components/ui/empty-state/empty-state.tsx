import type { ReactNode } from 'react';

interface EmptyStateProps {
  icon: ReactNode;
  title: string;
  description: string;
  action?: ReactNode;
  className?: string;
}

export function EmptyState({ icon, title, description, action, className = '' }: EmptyStateProps) {
  return (
    <div className={`text-center py-12 ${className}`}>
      <div className="text-zinc-500 mb-4">
        <div className="mx-auto h-12 w-12">{icon}</div>
      </div>
      <h3 className="text-lg font-medium text-zinc-300 mb-2">{title}</h3>
      <p className="text-zinc-500">{description}</p>
      {action && <div className="mt-4">{action}</div>}
    </div>
  );
}
