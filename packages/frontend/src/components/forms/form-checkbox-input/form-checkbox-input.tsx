import { formCheckboxInputStyles } from './form-checkbox-input.styles';

interface CheckboxInputProps {
  id: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  label: string | React.ReactNode;
  description?: string;
  className?: string;
}

export function CheckboxInput({
  id,
  checked,
  onChange,
  label,
  description,
  className = '',
}: CheckboxInputProps) {
  return (
    <>
      <div className={`${formCheckboxInputStyles.container} ${className}`}>
        <div className={formCheckboxInputStyles.fieldContainer}>
          <label htmlFor={id} className={formCheckboxInputStyles.label}>
            {label}
          </label>
          <div className={formCheckboxInputStyles.inputContainer}>
            <input
              id={id}
              type="checkbox"
              checked={checked}
              onChange={e => onChange(e.target.checked)}
              className={formCheckboxInputStyles.checkbox}
            />
          </div>
        </div>
        {description && (
          <p className={formCheckboxInputStyles.description}>{description}</p>
        )}
      </div>
      <div className={formCheckboxInputStyles.divider}></div>
    </>
  );
}
