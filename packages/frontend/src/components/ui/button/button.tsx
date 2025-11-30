import { cn } from '@/utils/tailwind';
import React from 'react';
import { buttonStyles } from './button.styles';

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
  // Special handling for icon variant sizes
  const getSizeClasses = () => {
    if (variant === 'icon') {
      return buttonStyles.iconSizes[size];
    }
    return buttonStyles.sizes[size];
  };

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      id={id}
      title={title}
      className={cn(
        buttonStyles.base,
        buttonStyles.variants[variant],
        getSizeClasses(),
        className
      )}
    >
      {children}
    </button>
  );
}
