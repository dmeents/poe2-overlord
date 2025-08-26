import { InformationCircleIcon } from '@heroicons/react/24/outline';
import { useState } from 'react';

interface TooltipProps {
  content: string | React.ReactNode;
  children?: React.ReactNode;
  className?: string;
}

export function Tooltip({ content, children, className = '' }: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false);

  return (
    <div className={`relative inline-block ${className}`}>
      <div
        className='inline-flex items-center gap-1 cursor-help'
        onMouseEnter={() => setIsVisible(true)}
        onMouseLeave={() => setIsVisible(false)}
      >
        {children}
        <InformationCircleIcon className='w-4 h-4 text-zinc-400 hover:text-zinc-300 transition-colors' />
      </div>

      {isVisible && (
        <div className='absolute z-10 w-80 p-3 bg-zinc-800 border border-zinc-700 text-zinc-200 text-sm shadow-lg -top-2 left-1/2 transform -translate-x-1/2 -translate-y-full'>
          <div className='relative'>
            {content}
            {/* Arrow pointing down */}
            <div className='absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-zinc-800'></div>
          </div>
        </div>
      )}
    </div>
  );
}
