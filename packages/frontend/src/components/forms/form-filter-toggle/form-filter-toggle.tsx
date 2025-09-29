import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/24/outline';
import { useEffect, useRef } from 'react';
import { formFilterToggleStyles } from './form-filter-toggle.styles';

export interface FilterToggleProps {
  isExpanded: boolean;
  onToggle: () => void;
  label: string;
  activeCount?: number;
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
}

export function FilterToggle({
  isExpanded,
  onToggle,
  label,
  activeCount = 0,
  disabled = false,
  className = '',
  children,
}: FilterToggleProps) {
  const hasActiveFilters = activeCount > 0;
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        if (isExpanded) {
          onToggle();
        }
      }
    };

    if (isExpanded) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isExpanded, onToggle]);

  return (
    <div
      className={`${formFilterToggleStyles.container} ${className}`}
      ref={dropdownRef}
    >
      <button
        type='button'
        onClick={onToggle}
        disabled={disabled}
        className={formFilterToggleStyles.toggleButton}
      >
        <span className={formFilterToggleStyles.toggleText}>
          {hasActiveFilters ? `${label} (${activeCount})` : label}
        </span>
        {isExpanded ? (
          <ChevronUpIcon className={formFilterToggleStyles.chevron} />
        ) : (
          <ChevronDownIcon className={formFilterToggleStyles.chevron} />
        )}
      </button>

      {isExpanded && children && (
        <div className={formFilterToggleStyles.content}>{children}</div>
      )}
    </div>
  );
}
