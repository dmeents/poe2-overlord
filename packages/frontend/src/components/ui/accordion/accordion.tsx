import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';
import { useId } from 'react';

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
}: AccordionProps): React.JSX.Element {
  // Generate unique IDs for ARIA attributes
  const buttonId = useId();
  const contentId = useId();
  const headingId = useId();

  return (
    <div className={`bg-zinc-900/80 border border-zinc-700/50 ${className}`}>
      {/* Accordion Header */}
      <div className="bg-zinc-700/50 border-b border-zinc-700/50">
        <button
          type="button"
          id={buttonId}
          onClick={onToggle}
          className="flex items-center justify-between w-full text-left hover:text-white transition-colors cursor-pointer p-3"
          aria-expanded={isExpanded}
          aria-controls={contentId}>
          <h3 id={headingId} className="text-base font-semibold text-white">
            {title}
          </h3>
          <div className="flex items-center gap-2">
            {subtitle && <span className="text-xs text-zinc-400">{subtitle}</span>}
            {isExpanded ? (
              <ChevronUpIcon className="w-4 h-4 text-zinc-400" aria-hidden="true" />
            ) : (
              <ChevronDownIcon className="w-4 h-4 text-zinc-400" aria-hidden="true" />
            )}
          </div>
        </button>
      </div>

      {/* Accordion Content */}
      {isExpanded && (
        <section id={contentId} aria-labelledby={headingId} className="p-3">
          {children}
        </section>
      )}
    </div>
  );
}
