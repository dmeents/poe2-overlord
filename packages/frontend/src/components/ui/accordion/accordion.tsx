import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';
import { useId } from 'react';
import { accordionStyles } from './accordion.styles';

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
    <div className={`${accordionStyles.container} ${className}`}>
      {/* Accordion Header */}
      <div className={accordionStyles.header}>
        <button
          type="button"
          id={buttonId}
          onClick={onToggle}
          className={accordionStyles.button}
          aria-expanded={isExpanded}
          aria-controls={contentId}>
          <h3 id={headingId} className={accordionStyles.title}>
            {title}
          </h3>
          <div className="flex items-center gap-2">
            {subtitle && <span className={accordionStyles.subtitle}>{subtitle}</span>}
            {isExpanded ? (
              <ChevronUpIcon className={accordionStyles.icon} aria-hidden="true" />
            ) : (
              <ChevronDownIcon className={accordionStyles.icon} aria-hidden="true" />
            )}
          </div>
        </button>
      </div>

      {/* Accordion Content */}
      {isExpanded && (
        <section id={contentId} aria-labelledby={headingId} className={accordionStyles.content}>
          {children}
        </section>
      )}
    </div>
  );
}
