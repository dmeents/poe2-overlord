import type { ReactNode } from 'react';
import { Button } from '../button/button';

interface CardProps {
  children: ReactNode;
  title?: string;
  subtitle?: string;
  icon?: ReactNode;
  className?: string;
  variant?: 'default' | 'insight' | 'featured';
  accentColor?: 'emerald' | 'blue' | 'purple' | 'amber' | 'zinc';
  showStatusIndicator?: boolean;
  rightAction?: {
    label: string;
    onClick: () => void;
  };
}

export function Card({
  children,
  title,
  subtitle,
  icon,
  className = '',
  variant = 'default',
  accentColor = 'zinc',
  showStatusIndicator = false,
  rightAction,
}: CardProps) {
  const baseClasses = 'bg-zinc-800/25 border border-zinc-700/50 overflow-hidden';

  const variantClasses = {
    default: '',
    insight: '',
    featured: '',
  };

  const accentGradients = {
    emerald: 'bg-gradient-to-r from-emerald-500/10 to-transparent',
    blue: 'bg-gradient-to-r from-blue-500/10 to-transparent',
    purple: 'bg-gradient-to-r from-purple-500/10 to-transparent',
    amber: 'bg-gradient-to-r from-amber-500/10 to-transparent',
    zinc: 'bg-gradient-to-r from-zinc-700/20 to-transparent',
  };

  const accentTextColors = {
    emerald: 'text-emerald-400',
    blue: 'text-blue-400',
    purple: 'text-purple-400',
    amber: 'text-amber-400',
    zinc: 'text-white',
  };

  const accentDotColors = {
    emerald: 'bg-emerald-400',
    blue: 'bg-blue-400',
    purple: 'bg-purple-400',
    amber: 'bg-amber-400',
    zinc: 'bg-zinc-400',
  };

  return (
    <div className={`${baseClasses} ${variantClasses[variant]} ${className}`}>
      {title && (
        <div className={`${accentGradients[accentColor]} px-5 py-3 border-b border-zinc-800`}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              {showStatusIndicator && (
                <div
                  className={`w-2 h-2 ${accentDotColors[accentColor]} rounded-full animate-pulse`}
                />
              )}
              {icon && <span className={`${accentTextColors[accentColor]} w-4 h-4`}>{icon}</span>}
              <span
                className={`text-xs ${accentTextColors[accentColor]} font-medium uppercase tracking-wider`}>
                {title}
              </span>
            </div>
            {subtitle && !rightAction && (
              <span className="text-xs text-zinc-400 font-normal">{subtitle}</span>
            )}
            {rightAction && (
              <Button
                onClick={rightAction.onClick}
                variant="text"
                size="xs"
                className="text-zinc-400 hover:text-zinc-200 h-auto py-0">
                {rightAction.label}
              </Button>
            )}
          </div>
        </div>
      )}
      <div>{children}</div>
    </div>
  );
}
