import type { ProcessStatusProps } from '@/types';
import { POE2_CONFIG } from '@/utils';
import { Activity } from 'lucide-react';
import React from 'react';
import { Button } from './button.component.tsx';

export const ProcessStatus: React.FC<ProcessStatusProps> = ({
  poe2Running,
  processInfo,
  onRefresh,
}) => {
  return (
    <div className='bg-[var(--color-bg-700)] rounded-md p-3 border border-[var(--color-border-600)]'>
      <div className='flex items-center justify-between'>
        <div className='flex items-center gap-2'>
          <Activity
            size={16}
            className={
              poe2Running ? 'text-[var(--color-primary-400)]' : 'text-red-500'
            }
          />
          <span className='text-[var(--color-text-100)] text-sm'>
            {POE2_CONFIG.PROCESS_NAME}
          </span>
        </div>
        <div className='flex items-center gap-2'>
          <span
            className={`
            text-xs px-2 py-1 rounded
            ${
              poe2Running
                ? 'bg-[var(--color-primary-500)]/20 text-[var(--color-primary-400)]'
                : 'bg-red-500/20 text-red-400'
            }
          `}
          >
            {poe2Running ? 'Running' : 'Not Found'}
          </span>
          <Button
            variant='secondary'
            size='sm'
            onClick={onRefresh}
            title='Refresh Process Status'
          >
            Refresh
          </Button>
        </div>
      </div>

      {processInfo && (
        <div className='mt-2 text-[var(--color-text-400)] text-xs'>
          Process: {processInfo.name}
        </div>
      )}
    </div>
  );
};
