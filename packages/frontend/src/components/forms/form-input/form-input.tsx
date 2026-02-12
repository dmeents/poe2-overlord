import { MagnifyingGlassIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { formInputStyles } from './form-input.styles';

export type InputType = 'text' | 'number' | 'search' | 'email' | 'password' | 'url';

export interface InputProps {
  id: string;
  value: string | number | null;
  onChange: (value: string | number | null) => void;
  type?: InputType;
  placeholder?: string;
  label?: string;
  disabled?: boolean;
  isValid?: boolean;
  warningMessage?: string;
  className?: string;
  min?: number;
  max?: number;
  step?: number;
  showClearButton?: boolean;
  onClear?: () => void;
}

export function Input({
  id,
  value,
  onChange,
  type = 'text',
  placeholder,
  label,
  disabled = false,
  isValid = true,
  warningMessage,
  className = '',
  min,
  max,
  step = 1,
  showClearButton = type === 'search',
  onClear,
}: InputProps) {
  const validationClasses =
    value && !isValid ? formInputStyles.invalidInput : formInputStyles.validInput;

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const inputValue = e.target.value;

    if (type === 'number') {
      if (inputValue === '') {
        onChange(null);
      } else {
        const numValue = parseFloat(inputValue);
        if (!Number.isNaN(numValue)) {
          onChange(numValue);
        }
      }
    } else {
      onChange(inputValue);
    }
  };

  const handleClear = () => {
    onChange('');
    onClear?.();
  };

  const displayValue = value ?? '';

  return (
    <div className={formInputStyles.container}>
      {label && (
        <label htmlFor={id} className={formInputStyles.label}>
          {label}
        </label>
      )}

      <div className={formInputStyles.inputContainer}>
        {type === 'search' && (
          <div className={formInputStyles.iconContainer}>
            <MagnifyingGlassIcon className={formInputStyles.searchIcon} />
          </div>
        )}

        <input
          id={id}
          type={type === 'search' ? 'text' : type}
          value={displayValue}
          onChange={handleChange}
          placeholder={placeholder}
          disabled={disabled}
          min={min}
          max={max}
          step={step}
          className={`${formInputStyles.input} ${
            type === 'search' ? formInputStyles.searchInput : ''
          } ${validationClasses} ${className}`}
        />

        {showClearButton && value && (
          <button
            type="button"
            onClick={handleClear}
            className={formInputStyles.clearButton}
            aria-label="Clear input">
            <XMarkIcon className={formInputStyles.clearIcon} />
          </button>
        )}
      </div>

      {value && !isValid && warningMessage && (
        <p className={formInputStyles.warningMessage}>⚠️ {warningMessage}</p>
      )}
    </div>
  );
}
