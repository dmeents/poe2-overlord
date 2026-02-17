import { CheckIcon, ChevronDownIcon } from '@heroicons/react/24/outline';
import { useState } from 'react';
import { useDropdownPosition } from '../../../hooks/useDropdownPosition';
import { DropdownPortal } from '../../ui/dropdown-portal/dropdown-portal';
import { formSortSelectStyles } from './form-sort-select.styles';

export interface SortOption {
  value: string;
  label: string;
}

export interface SortSelectProps {
  id: string;
  value: string;
  direction: 'asc' | 'desc';
  onChange: (field: string, direction?: 'asc' | 'desc') => void;
  onReset: () => void;
  options: SortOption[];
  label?: string;
  disabled?: boolean;
  className?: string;
}

export function SortSelect({
  id,
  value,
  direction,
  onChange,
  onReset,
  options,
  label,
  disabled = false,
  className = '',
}: SortSelectProps) {
  const [isOpen, setIsOpen] = useState(false);

  const { dropdownRef, triggerRef, dropdownPosition } = useDropdownPosition({
    isOpen,
    onClose: () => setIsOpen(false),
    includeWidth: true,
  });

  const selectedOption = options.find(option => option.value === value);

  const handleOptionSelect = (optionValue: string) => {
    onChange(optionValue);
    setIsOpen(false);
  };

  const handleDirectionToggle = () => {
    onChange(value, direction === 'asc' ? 'desc' : 'asc');
  };

  const handleReset = () => {
    onReset();
    setIsOpen(false);
  };

  const getCurrentSortLabel = () => {
    return selectedOption ? selectedOption.label : 'Sort by...';
  };

  const getDirectionIcon = () => {
    return direction === 'desc' ? '↓' : '↑';
  };

  return (
    <div className={`${formSortSelectStyles.container} ${className}`}>
      {label && (
        <label htmlFor={id} className={formSortSelectStyles.label}>
          {label}
        </label>
      )}

      <div
        className={formSortSelectStyles.triggerContainer}
        ref={triggerRef as React.RefObject<HTMLDivElement>}>
        <button
          type="button"
          className={formSortSelectStyles.trigger}
          onClick={() => !disabled && setIsOpen(!isOpen)}
          disabled={disabled}
          aria-haspopup="listbox"
          aria-expanded={isOpen}>
          <span className={formSortSelectStyles.triggerText}>{getCurrentSortLabel()}</span>
          <div className={formSortSelectStyles.triggerIcons}>
            <span className={formSortSelectStyles.directionIcon}>{getDirectionIcon()}</span>
            <ChevronDownIcon
              className={`${formSortSelectStyles.chevron} ${isOpen ? formSortSelectStyles.chevronOpen : ''}`}
            />
          </div>
        </button>
      </div>

      <DropdownPortal
        isOpen={isOpen}
        dropdownRef={dropdownRef}
        position={dropdownPosition}
        className={formSortSelectStyles.dropdown}>
        <div className={formSortSelectStyles.header}>
          <h4 className={formSortSelectStyles.headerTitle}>Sort Options</h4>
          <button
            type="button"
            onClick={handleReset}
            className={formSortSelectStyles.resetButton}>
            Reset
          </button>
        </div>

        <div className={formSortSelectStyles.optionsList} role="listbox">
          {options.map(option => {
            const isSelected = option.value === value;
            return (
              <div
                key={option.value}
                className={`${formSortSelectStyles.option} ${
                  isSelected ? formSortSelectStyles.optionSelected : ''
                }`}
                onClick={() => handleOptionSelect(option.value)}
                onKeyDown={e => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    handleOptionSelect(option.value);
                  }
                }}
                role="option"
                tabIndex={0}
                aria-selected={isSelected}>
                <span className={formSortSelectStyles.optionLabel}>{option.label}</span>
                {isSelected && <CheckIcon className={formSortSelectStyles.optionIcon} />}
              </div>
            );
          })}
        </div>

        {/* Direction Toggle */}
        <div className={formSortSelectStyles.directionToggle}>
          <button
            type="button"
            onClick={handleDirectionToggle}
            className={formSortSelectStyles.directionButton}>
            <span className={formSortSelectStyles.directionText}>
              {direction === 'desc' ? 'Descending' : 'Ascending'}
            </span>
            <span className={formSortSelectStyles.directionIconLarge}>{getDirectionIcon()}</span>
          </button>
        </div>
      </DropdownPortal>
    </div>
  );
}
