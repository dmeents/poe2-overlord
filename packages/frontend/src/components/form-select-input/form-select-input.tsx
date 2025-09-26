import { formSelectInputStyles } from './form-select-input.styles';

interface SelectOption {
  value: string;
  label: string;
}

interface SelectInputProps {
  id: string;
  value: string;
  onChange: (value: string) => void;
  options: SelectOption[];
  className?: string;
  placeholder?: string;
}

export function SelectInput({
  id,
  value,
  onChange,
  options,
  className = '',
  placeholder,
}: SelectInputProps) {
  return (
    <div className={formSelectInputStyles.container}>
      <select
        id={id}
        value={value}
        onChange={e => onChange(e.target.value)}
        className={`${formSelectInputStyles.select} ${className}`}
      >
        {placeholder && (
          <option
            value=''
            disabled
            className={formSelectInputStyles.placeholderOption}
          >
            {placeholder}
          </option>
        )}
        {options.map(option => (
          <option
            key={option.value}
            value={option.value}
            className={formSelectInputStyles.option}
          >
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
}
