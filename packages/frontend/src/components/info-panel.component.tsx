import type { InfoPanelProps } from '@/types';
import React from 'react';

export const InfoPanel: React.FC<InfoPanelProps> = ({
  title,
  description,
  icon,
}) => {
  return (
    <div className='bg-gray-800 rounded-md p-3 border border-gray-600'>
      <div className='flex items-start gap-2'>
        <div className='text-blue-500 mt-0.5 flex-shrink-0'>{icon}</div>
        <div>
          <h3 className='text-white text-sm font-semibold mb-1 m-0'>{title}</h3>
          <p className='text-gray-400 text-xs m-0 leading-relaxed'>
            {description}
          </p>
        </div>
      </div>
    </div>
  );
};
