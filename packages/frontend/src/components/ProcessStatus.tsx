import React from 'react';
import { Activity } from 'lucide-react';
import { Button } from './Button';
import { POE2_CONFIG } from '../utils';
import type { ProcessStatusProps } from '../types';

export const ProcessStatus: React.FC<ProcessStatusProps> = ({
  poe2Running,
  processInfo,
  onRefresh,
}) => {
  return (
    <div className='bg-gray-800 rounded-md p-3 border border-gray-600'>
      <div className='flex items-center justify-between'>
        <div className='flex items-center gap-2'>
          <Activity
            size={16}
            className={poe2Running ? 'text-green-500' : 'text-red-500'}
          />
          <span className='text-white text-sm'>{POE2_CONFIG.PROCESS_NAME}</span>
        </div>
        <div className='flex items-center gap-2'>
          <span
            className={`
            text-xs px-2 py-1 rounded
            ${
              poe2Running
                ? 'bg-green-500/20 text-green-500'
                : 'bg-red-500/20 text-red-500'
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
        <div className='mt-2 text-gray-400 text-xs'>
          Process: {processInfo.name}
        </div>
      )}
    </div>
  );
};
