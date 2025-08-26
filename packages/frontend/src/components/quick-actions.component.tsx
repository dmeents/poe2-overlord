import type { QuickActionProps } from '@/types';

interface QuickActionsComponentProps {
  actions: QuickActionProps[];
}

export function QuickActionsComponent({ actions }: QuickActionsComponentProps) {
  return (
    <div>
      <h3>Quick Actions</h3>
      <div>
        {actions.map((action, index) => (
          <button key={index} onClick={action.onClick} title={action.label}>
            {action.icon}
            <span>{action.label}</span>
          </button>
        ))}
      </div>
    </div>
  );
}
