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
    'inline-flex items-center justify-center font-mono transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-black disabled:opacity-50 disabled:cursor-not-allowed';

  const variantClasses = {
    primary:
      'bg-gray-700 border border-gray-600 text-white font-mono text-sm hover:bg-amber-500 hover:text-black hover:border-amber-500 focus:outline-none focus:ring-2 focus:ring-amber-500/50 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-gray-700 disabled:hover:text-white disabled:hover:border-gray-600',
    secondary:
      'bg-gray-700 border border-gray-500 text-white hover:bg-gray-800 hover:border-gray-600',
    ghost:
      'bg-transparent border border-transparent text-gray-300 hover:text-white hover:bg-gray-800',
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
