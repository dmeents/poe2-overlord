import type React from 'react';
import { cn } from '@/utils/tailwind';
import { buttonStyles } from './button.styles';

interface ButtonProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'text' | 'icon' | 'danger' | 'active';
  size?: 'xs' | 'sm' | 'md' | 'lg';
  onClick?: () => void;
  disabled?: boolean;
  loading?: boolean;
  type?: 'button' | 'submit' | 'reset';
  className?: string;
  id?: string;
  title?: string;
}

/**
 * Loading spinner for button loading state
 */
function LoadingSpinner({ size }: { size: 'xs' | 'sm' | 'md' | 'lg' }) {
  const spinnerSizes = {
    xs: 'w-3 h-3',
    sm: 'w-3.5 h-3.5',
    md: 'w-4 h-4',
    lg: 'w-5 h-5',
  };

  return (
    <svg
      className={cn('animate-spin', spinnerSizes[size])}
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      aria-hidden="true">
      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
      <path
        className="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
  );
}

export function Button({
  children,
  variant = 'primary',
  size = 'md',
  onClick,
  disabled = false,
  loading = false,
  type = 'button',
  className,
  id,
  title,
}: ButtonProps) {
  const getSizeClasses = () => {
    if (variant === 'icon') {
      return buttonStyles.iconSizes[size];
    }
    return buttonStyles.sizes[size];
  };

  const isDisabled = disabled || loading;

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={isDisabled}
      id={id}
      title={title}
      aria-busy={loading}
      className={cn(
        buttonStyles.base,
        buttonStyles.variants[variant],
        getSizeClasses(),
        loading && 'cursor-wait',
        className,
      )}>
      {loading && (
        <span className="mr-2">
          <LoadingSpinner size={size} />
        </span>
      )}
      {children}
    </button>
  );
}
