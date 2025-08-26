import type { FooterProps } from '@/types';
import React from 'react';

export const Footer: React.FC<FooterProps> = ({ version, technology }) => {
  return (
    <div className='p-2 border-t border-[var(--color-border-600)]'>
      <div className='flex items-center justify-between text-xs text-[var(--color-text-400)]'>
        <span>{version}</span>
        <span>{technology}</span>
      </div>
    </div>
  );
};
