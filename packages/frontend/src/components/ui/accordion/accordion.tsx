import { ChevronDownIcon } from '@heroicons/react/24/outline';
import type { ReactNode } from 'react';
import { useId } from 'react';
import { cn } from '@/utils/tailwind';
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
    <div className={cn(accordionStyles.container, className)}>
      {/* Collapsible Section Divider */}
      <button
        type="button"
        id={buttonId}
        onClick={onToggle}
        className={accordionStyles.button}
        aria-expanded={isExpanded}
        aria-controls={contentId}>
        {/* Chevron Icon */}
        <ChevronDownIcon
          className={cn(
            accordionStyles.icon,
            isExpanded ? accordionStyles.iconExpanded : accordionStyles.iconCollapsed,
          )}
          aria-hidden="true"
        />

        {/* Title */}
        <h3
          id={headingId}
          className={cn(
            accordionStyles.title,
            isExpanded ? accordionStyles.titleExpanded : accordionStyles.titleCollapsed,
          )}>
          {title}
        </h3>

        {/* Subtitle with dot separator */}
        {subtitle && (
          <>
            <span aria-hidden="true" className="text-stone-500">
              ·
            </span>
            <span
              className={cn(
                accordionStyles.subtitle,
                isExpanded ? accordionStyles.subtitleExpanded : accordionStyles.subtitleCollapsed,
              )}>
              {subtitle}
            </span>
          </>
        )}

        {/* Divider line */}
        <div
          aria-hidden="true"
          className={cn(
            accordionStyles.divider,
            isExpanded ? accordionStyles.dividerExpanded : accordionStyles.dividerCollapsed,
          )}
        />
      </button>

      {/* Content with grid animation */}
      <div
        className={cn(
          accordionStyles.contentWrapper,
          isExpanded
            ? accordionStyles.contentWrapperExpanded
            : accordionStyles.contentWrapperCollapsed,
        )}>
        <section
          id={contentId}
          aria-labelledby={headingId}
          aria-hidden={!isExpanded || undefined}
          className={accordionStyles.content}>
          {children}
        </section>
      </div>
    </div>
  );
}
