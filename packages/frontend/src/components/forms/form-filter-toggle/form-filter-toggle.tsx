import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/24/outline';
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

  return (
    <div className={`${formFilterToggleStyles.container} ${className}`}>
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
