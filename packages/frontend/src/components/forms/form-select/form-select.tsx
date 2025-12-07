import { CheckIcon, ChevronDownIcon } from '@heroicons/react/24/outline';
import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { formSelectStyles } from './form-select.styles';

export interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export type SelectVariant = 'basic' | 'dropdown';

export interface SelectProps {
  id: string;
  value: string;
  onChange: (value: string) => void;
  options: SelectOption[];
  variant?: SelectVariant;
  placeholder?: string;
  label?: string;
  disabled?: boolean;
  isValid?: boolean;
  warningMessage?: string;
  className?: string;
  triggerClassName?: string;
  dropdownClassName?: string;
  showClearButton?: boolean;
  onClear?: () => void;
  renderTrigger?: (
    isOpen: boolean,
    selectedOption?: SelectOption
  ) => React.ReactNode;
  renderOption?: (option: SelectOption, isSelected: boolean) => React.ReactNode;
}

export function Select({
  id,
  value,
  onChange,
  options,
  variant = 'basic',
  placeholder = 'Select an option...',
  label,
  disabled = false,
  isValid = true,
  warningMessage,
  className = '',
  triggerClassName = '',
  dropdownClassName = '',
  showClearButton = false,
  onClear,
  renderTrigger,
  renderOption,
}: SelectProps) {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLDivElement>(null);
  const [dropdownPosition, setDropdownPosition] = useState({
    top: 0,
    left: 0,
    width: 0,
  });

  // Calculate dropdown position when opened (only for dropdown variant)
  useLayoutEffect(() => {
    if (variant === 'dropdown' && isOpen && triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      setDropdownPosition({
        top: rect.bottom + window.scrollY + 8,
        left: rect.left + window.scrollX,
        width: rect.width,
      });
    }
  }, [isOpen, variant]);

  // Close dropdown when clicking outside (only for dropdown variant)
  useEffect(() => {
    if (variant !== 'dropdown') return;

    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen, variant]);

  const selectedOption = options.find(option => option.value === value);

  const handleOptionSelect = (optionValue: string) => {
    onChange(optionValue);
    setIsOpen(false);
  };

  const handleClear = (e: React.MouseEvent) => {
    e.stopPropagation();
    onClear?.();
    setIsOpen(false);
  };

  // Basic select variant
  if (variant === 'basic') {
    const validationClasses =
      value && !isValid
        ? formSelectStyles.invalidSelect
        : formSelectStyles.validSelect;

    return (
      <div className={formSelectStyles.container}>
        {label && (
          <label htmlFor={id} className={formSelectStyles.label}>
            {label}
          </label>
        )}
        <select
          id={id}
          value={value}
          onChange={e => onChange(e.target.value)}
          disabled={disabled}
          className={`${formSelectStyles.basicSelect} ${validationClasses} ${className}`}
        >
          {placeholder && (
            <option
              value=''
              disabled
              className={formSelectStyles.placeholderOption}
            >
              {placeholder}
            </option>
          )}
          {options.map(option => (
            <option
              key={option.value}
              value={option.value}
              disabled={option.disabled}
              className={formSelectStyles.option}
            >
              {option.label}
            </option>
          ))}
        </select>
        {value && !isValid && warningMessage && (
          <p className={formSelectStyles.warningMessage}>⚠️ {warningMessage}</p>
        )}
      </div>
    );
  }

  // Dropdown select variant
  const defaultRenderTrigger = (
    isOpen: boolean,
    selectedOption?: SelectOption
  ) => (
    <button
      type='button'
      className={`${formSelectStyles.trigger} ${triggerClassName}`}
      onClick={() => !disabled && setIsOpen(!isOpen)}
      disabled={disabled}
      aria-haspopup='listbox'
      aria-expanded={isOpen}
    >
      <span className={formSelectStyles.triggerText}>
        {selectedOption ? selectedOption.label : placeholder}
      </span>
      <div className={formSelectStyles.triggerIcons}>
        {showClearButton && selectedOption && (
          <button
            type='button'
            onClick={handleClear}
            className={formSelectStyles.clearButton}
            aria-label='Clear selection'
          >
            ×
          </button>
        )}
        <ChevronDownIcon
          className={`${formSelectStyles.chevron} ${isOpen ? formSelectStyles.chevronOpen : ''}`}
        />
      </div>
    </button>
  );

  const defaultRenderOption = (option: SelectOption, isSelected: boolean) => (
    <div
      key={option.value}
      className={`px-4 py-2 hover:bg-zinc-700/50 cursor-pointer transition-colors flex items-center justify-between ${
        isSelected ? formSelectStyles.optionSelected : ''
      } ${option.disabled ? formSelectStyles.optionDisabled : ''}`}
      onClick={() => !option.disabled && handleOptionSelect(option.value)}
      role='option'
      aria-selected={isSelected}
    >
      <span className={formSelectStyles.optionLabel}>{option.label}</span>
      {isSelected && <CheckIcon className='w-4 h-4 text-emerald-400' />}
    </div>
  );

  return (
    <div
      className={`${formSelectStyles.container} ${className}`}
      ref={dropdownRef}
    >
      {label && (
        <label htmlFor={id} className={formSelectStyles.label}>
          {label}
        </label>
      )}

      <div className={formSelectStyles.triggerContainer} ref={triggerRef}>
        {renderTrigger
          ? renderTrigger(isOpen, selectedOption)
          : defaultRenderTrigger(isOpen, selectedOption)}
      </div>

      {isOpen && (
        <div
          className={`${formSelectStyles.dropdown} ${dropdownClassName}`}
          style={{
            top: `${dropdownPosition.top}px`,
            left: `${dropdownPosition.left}px`,
            minWidth: `${dropdownPosition.width}px`,
          }}
        >
          <div className={formSelectStyles.optionsList} role='listbox'>
            {options.length === 0 ? (
              <div className={formSelectStyles.emptyState}>
                No options available
              </div>
            ) : (
              options.map(option => {
                const isSelected = option.value === value;
                return renderOption
                  ? renderOption(option, isSelected)
                  : defaultRenderOption(option, isSelected);
              })
            )}
          </div>
        </div>
      )}
    </div>
  );
}
