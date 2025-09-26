import { formTextInputStyles } from './form-text-input.styles';

interface TextInputProps {
  id: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  isValid?: boolean;
  warningMessage?: string;
  className?: string;
}

export function TextInput({
  id,
  value,
  onChange,
  placeholder,
  isValid = true,
  warningMessage,
  className = '',
}: TextInputProps) {
  const validationClasses =
    value && !isValid
      ? formTextInputStyles.invalidInput
      : formTextInputStyles.validInput;

  return (
    <div className={formTextInputStyles.container}>
      <input
        id={id}
        type='text'
        value={value}
        onChange={e => onChange(e.target.value)}
        placeholder={placeholder}
        className={`${formTextInputStyles.baseInput} ${validationClasses} ${className}`}
      />
      {value && !isValid && warningMessage && (
        <p className={formTextInputStyles.warningMessage}>
          ⚠️ {warningMessage}
        </p>
      )}
    </div>
  );
}
