import { CheckIcon, ChevronDownIcon } from '@heroicons/react/24/outline';
import { useEffect, useLayoutEffect, useRef, useState } from 'react';
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
  const dropdownRef = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLDivElement>(null);
  const [dropdownPosition, setDropdownPosition] = useState({
    top: 0,
    left: 0,
    width: 0,
  });

  // Calculate dropdown position when opened
  useLayoutEffect(() => {
    if (isOpen && triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      setDropdownPosition({
        top: rect.bottom + window.scrollY + 8,
        left: rect.left + window.scrollX,
        width: rect.width,
      });
    }
  }, [isOpen]);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen]);

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
    <div className={`${formSortSelectStyles.container} ${className}`} ref={dropdownRef}>
      {label && (
        <label htmlFor={id} className={formSortSelectStyles.label}>
          {label}
        </label>
      )}

      <div className={formSortSelectStyles.triggerContainer} ref={triggerRef}>
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

      {isOpen && (
        <div
          className={formSortSelectStyles.dropdown}
          style={{
            top: `${dropdownPosition.top}px`,
            left: `${dropdownPosition.left}px`,
            minWidth: `${dropdownPosition.width}px`,
          }}>
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
                  role="option"
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
        </div>
      )}
    </div>
  );
}
