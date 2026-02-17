import { CheckIcon, ChevronDownIcon } from '@heroicons/react/24/outline';
import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { DropdownPortal } from '../../ui/dropdown-portal/dropdown-portal';
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
  // Uses viewport-relative coordinates (no scroll offset) for fixed positioning in portal
  useLayoutEffect(() => {
    if (variant === 'dropdown' && isOpen && triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      setDropdownPosition({
        top: rect.bottom + 8,
        left: rect.left,
        width: rect.width,
      });
    }
  }, [isOpen, variant]);

  // Close dropdown when clicking outside (only for dropdown variant)
  // Must check both trigger and dropdown refs since dropdown is in a portal
  useEffect(() => {
    if (variant !== 'dropdown') return;

    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as Node;
      const isOutsideDropdown = dropdownRef.current && !dropdownRef.current.contains(target);
      const isOutsideTrigger = triggerRef.current && !triggerRef.current.contains(target);

      if (isOutsideDropdown && isOutsideTrigger) {
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

  // Basic select variant
  if (variant === 'basic') {
    const validationClasses =
      value && !isValid ? formSelectStyles.invalidSelect : formSelectStyles.validSelect;

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
          className={`${formSelectStyles.basicSelect} ${validationClasses} ${className}`}>
          {placeholder && (
            <option value="" disabled className={formSelectStyles.placeholderOption}>
              {placeholder}
            </option>
          )}
          {options.map(option => (
            <option
              key={option.value}
              value={option.value}
              disabled={option.disabled}
              className={formSelectStyles.option}>
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
  const defaultRenderTrigger = (isOpen: boolean, selectedOption?: SelectOption) => (
    <button
      type="button"
      className={`${formSelectStyles.trigger} ${triggerClassName}`}
      onClick={() => !disabled && setIsOpen(!isOpen)}
      disabled={disabled}
      aria-haspopup="listbox"
      aria-expanded={isOpen}>
      <span className={formSelectStyles.triggerText}>
        {selectedOption ? selectedOption.label : placeholder}
      </span>
      <div className={formSelectStyles.triggerIcons}>
        <ChevronDownIcon
          className={`${formSelectStyles.chevron} ${isOpen ? formSelectStyles.chevronOpen : ''}`}
        />
      </div>
    </button>
  );

  const defaultRenderOption = (option: SelectOption, isSelected: boolean) => (
    <div
      key={option.value}
      className={`px-4 py-2 hover:bg-stone-700/50 cursor-pointer transition-colors flex items-center justify-between ${
        isSelected ? formSelectStyles.optionSelected : ''
      } ${option.disabled ? formSelectStyles.optionDisabled : ''}`}
      onClick={() => !option.disabled && handleOptionSelect(option.value)}
      onKeyDown={e => {
        if (!option.disabled && (e.key === 'Enter' || e.key === ' ')) {
          e.preventDefault();
          handleOptionSelect(option.value);
        }
      }}
      role="option"
      tabIndex={option.disabled ? -1 : 0}
      aria-selected={isSelected}>
      <span className={formSelectStyles.optionLabel}>{option.label}</span>
      {isSelected && <CheckIcon className="w-4 h-4 text-verdant-400" />}
    </div>
  );

  return (
    <div className={`${formSelectStyles.container} ${className}`}>
      {label && (
        <label htmlFor={id} className={formSelectStyles.label}>
          {label}
        </label>
      )}

      <div className={formSelectStyles.triggerContainer} ref={triggerRef}>
        {defaultRenderTrigger(isOpen, selectedOption)}
      </div>

      <DropdownPortal
        isOpen={isOpen}
        dropdownRef={dropdownRef}
        position={dropdownPosition}
        className={`${formSelectStyles.dropdown} ${dropdownClassName}`}>
        <div className={formSelectStyles.optionsList} role="listbox">
          {options.length === 0 ? (
            <div className={formSelectStyles.emptyState}>No options available</div>
          ) : (
            options.map(option => {
              const isSelected = option.value === value;
              return defaultRenderOption(option, isSelected);
            })
          )}
        </div>
      </DropdownPortal>
    </div>
  );
}
