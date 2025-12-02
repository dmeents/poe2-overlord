import type { ReactNode } from 'react';

interface CardProps {
  children: ReactNode;
  title?: string;
  subtitle?: string;
  icon?: ReactNode;
  className?: string;
  variant?: 'default' | 'insight' | 'featured';
}

export function Card({
  children,
  title,
  subtitle,
  icon,
  className = '',
  variant = 'default',
}: CardProps) {
  const baseClasses = 'bg-zinc-800/50 border border-zinc-700/50 p-6 shadow-lg';

  const variantClasses = {
    default: '',
    insight: '',
    featured: 'bg-zinc-900/80 border border-zinc-700/50',
  };

  const titleClasses =
    'text-lg font-semibold text-white mb-6 flex items-center justify-between';

  return (
    <div className={`${baseClasses} ${variantClasses[variant]} ${className}`}>
      {title && (
        <h3 className={titleClasses}>
          <div className='flex items-center'>
            {icon && <span className='w-5 h-5 mr-2 text-zinc-400'>{icon}</span>}
            {title}
          </div>
          {subtitle && (
            <span className='text-xs text-zinc-400 font-normal'>
              {subtitle}
            </span>
          )}
        </h3>
      )}
      {children}
    </div>
  );
}
