import { formInputStyles } from './form-input.styles';

export interface InputProps {
  id: string;
  value: string | number;
  onChange: (value: string) => void;
  type?: 'text' | 'number' | 'search' | 'email' | 'password' | 'url';
  placeholder?: string;
  isInvalid?: boolean;
  errorMessage?: string;
  min?: number;
  max?: number;
  disabled?: boolean;
}

export function Input({
  id,
  value,
  onChange,
  type = 'text',
  placeholder,
  isInvalid = false,
  errorMessage,
  min,
  max,
  disabled = false,
}: InputProps) {
  return (
    <div>
      <input
        id={id}
        type={type}
        value={value}
        onChange={e => onChange(e.target.value)}
        placeholder={placeholder}
        min={min}
        max={max}
        disabled={disabled}
        className={`${formInputStyles.input} ${isInvalid ? formInputStyles.invalidInput : ''}`}
      />
      {isInvalid && errorMessage && <p className={formInputStyles.errorMessage}>{errorMessage}</p>}
    </div>
  );
}
