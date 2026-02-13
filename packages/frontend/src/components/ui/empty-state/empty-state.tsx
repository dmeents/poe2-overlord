import type { ReactNode } from 'react';
import { emptyStateStyles } from './empty-state.styles';

interface EmptyStateProps {
  icon: ReactNode;
  title: string;
  description: string;
  action?: ReactNode;
  className?: string;
}

export function EmptyState({ icon, title, description, action, className = '' }: EmptyStateProps) {
  return (
    <div className={`${emptyStateStyles.container} ${className}`}>
      <div className={emptyStateStyles.iconContainer}>
        <div className={emptyStateStyles.icon}>{icon}</div>
      </div>
      <h3 className={emptyStateStyles.title}>{title}</h3>
      <p className={emptyStateStyles.description}>{description}</p>
      {action && <div className={emptyStateStyles.actionContainer}>{action}</div>}
    </div>
  );
}
