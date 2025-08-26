import { cn } from '@/utils';
import React from 'react';

interface ButtonProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'icon';
  size?: 'xs' | 'sm' | 'md' | 'lg';
  onClick?: () => void;
  disabled?: boolean;
  type?: 'button' | 'submit' | 'reset';
  className?: string;
  id?: string;
  title?: string;
}

export function Button({
  children,
  variant = 'primary',
  size = 'md',
  onClick,
  disabled = false,
  type = 'button',
  className,
  id,
  title,
}: ButtonProps) {
  const baseClasses =
    'cursor-pointer inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50';

  const variantClasses = {
    primary:
      'bg-emerald-800 text-white hover:bg-emerald-900 border border-emerald-900',
    secondary:
      'bg-zinc-800 text-zinc-200 hover:bg-zinc-700 border border-zinc-700',
    outline:
      'border border-zinc-700 bg-zinc-900 text-zinc-200 hover:bg-zinc-800',
    ghost: 'hover:bg-zinc-800 hover:text-zinc-200 cursor-default',
    icon: 'flex items-center justify-center bg-transparent text-zinc-400 hover:text-zinc-200',
  };

  const sizeClasses = {
    xs: 'h-6 px-2 text-xs',
    sm: 'h-8 px-3 text-sm',
    md: 'h-10 px-4 py-2',
    lg: 'h-11 px-8',
  };

  // Special handling for icon variant sizes
  const getSizeClasses = () => {
    if (variant === 'icon') {
      return {
        xs: 'w-4 h-4',
        sm: 'w-5 h-5',
        md: 'w-6 h-6',
        lg: 'h-8 w-8 p-0',
      }[size];
    }
    return sizeClasses[size];
  };

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      id={id}
      title={title}
      className={cn(
        baseClasses,
        variantClasses[variant],
        getSizeClasses(),
        className
      )}
    >
      {children}
    </button>
  );
}
