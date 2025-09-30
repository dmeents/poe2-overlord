import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';

export interface AccordionProps {
  title: string;
  subtitle?: string;
  isExpanded: boolean;
  onToggle: () => void;
  children: ReactNode;
  className?: string;
}

export function Accordion({
  title,
  subtitle,
  isExpanded,
  onToggle,
  children,
  className = '',
}: AccordionProps) {
  return (
    <div className={`bg-zinc-900/80 border border-zinc-700/50 ${className}`}>
      {/* Accordion Header */}
      <div className='bg-zinc-700/50 border-b border-zinc-700/50'>
        <button
          type='button'
          onClick={onToggle}
          className='flex items-center justify-between w-full text-left hover:text-white transition-colors cursor-pointer p-3'
        >
          <h3 className='text-base font-semibold text-white'>{title}</h3>
          <div className='flex items-center gap-2'>
            {subtitle && (
              <span className='text-xs text-zinc-400'>{subtitle}</span>
            )}
            {isExpanded ? (
              <ChevronUpIcon className='w-4 h-4 text-zinc-400' />
            ) : (
              <ChevronDownIcon className='w-4 h-4 text-zinc-400' />
            )}
          </div>
        </button>
      </div>

      {/* Accordion Content */}
      {isExpanded && <div className='p-3'>{children}</div>}
    </div>
  );
}
