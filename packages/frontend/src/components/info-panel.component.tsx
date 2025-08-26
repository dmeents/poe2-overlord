import type { InfoPanelProps } from '@/types';
import React from 'react';

export const InfoPanel: React.FC<InfoPanelProps> = ({
  title,
  description,
  icon,
}) => {
  return (
    <div className='bg-[var(--color-bg-700)] rounded-md p-3 border border-[var(--color-border-600)]'>
      <div className='flex items-start gap-2'>
        <div className='text-[var(--color-secondary-400)] mt-0.5 flex-shrink-0'>
          {icon}
        </div>
        <div>
          <h3 className='text-[var(--color-text-100)] text-sm font-semibold mb-1 m-0'>
            {title}
          </h3>
          <p className='text-[var(--color-text-400)] text-xs m-0 leading-relaxed'>
            {description}
          </p>
        </div>
      </div>
    </div>
  );
};
