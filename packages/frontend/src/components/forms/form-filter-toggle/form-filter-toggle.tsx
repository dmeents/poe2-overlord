import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/24/outline';
import { useEffect, useRef, useState } from 'react';
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
  const buttonRef = useRef<HTMLButtonElement>(null);
  const [dropdownPosition, setDropdownPosition] = useState({ top: 0, left: 0 });

  // Calculate dropdown position when opened
  useEffect(() => {
    if (isExpanded && buttonRef.current) {
      const rect = buttonRef.current.getBoundingClientRect();
      setDropdownPosition({
        top: rect.bottom + window.scrollY + 8,
        left: rect.left + window.scrollX,
      });
    }
  }, [isExpanded]);

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
        ref={buttonRef}
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
        <div
          className={formFilterToggleStyles.content}
          style={{
            top: `${dropdownPosition.top}px`,
            left: `${dropdownPosition.left}px`,
          }}
        >
          {children}
        </div>
      )}
    </div>
  );
}
