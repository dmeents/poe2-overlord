import { cn } from '@/utils';
import type { ButtonHTMLAttributes } from 'react';
import React from 'react';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
}

export const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  size = 'md',
  className,
  children,
  ...props
}) => {
  const baseClasses =
    'inline-flex items-center justify-center font-mono transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-[var(--color-bg-900)] disabled:opacity-50 disabled:cursor-not-allowed';

  const variantClasses = {
    primary:
      'bg-[var(--color-primary-600)] border border-[var(--color-primary-500)] text-[var(--color-text-inverted)] font-mono text-sm hover:bg-[var(--color-primary-500)] hover:border-[var(--color-primary-400)] focus:ring-[var(--color-primary-400)] active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-[var(--color-primary-600)] disabled:hover:border-[var(--color-primary-500)]',
    secondary:
      'bg-[var(--color-bg-700)] border border-[var(--color-border-600)] text-[var(--color-text-100)] hover:bg-[var(--color-bg-600)] hover:border-[var(--color-border-500)] hover:text-[var(--color-text-50)]',
    ghost:
      'bg-transparent border border-transparent text-[var(--color-text-300)] hover:text-[var(--color-text-100)] hover:bg-[var(--color-bg-700)]',
  };

  const sizeClasses = {
    sm: 'px-3 py-1.5 text-xs',
    md: 'px-4 py-2 text-sm',
    lg: 'px-6 py-3 text-base',
  };

  return (
    <button
      className={cn(
        baseClasses,
        variantClasses[variant],
        sizeClasses[size],
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
};
