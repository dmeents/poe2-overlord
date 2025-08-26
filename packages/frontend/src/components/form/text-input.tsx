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
  const baseClasses =
    'w-full px-3 py-2 border bg-zinc-900 text-zinc-100 placeholder-zinc-600 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500';

  const validationClasses =
    value && !isValid ? 'border-amber-500 bg-amber-950/20' : 'border-zinc-700';

  return (
    <div className='space-y-2'>
      <input
        id={id}
        type='text'
        value={value}
        onChange={e => onChange(e.target.value)}
        placeholder={placeholder}
        className={`${baseClasses} ${validationClasses} ${className}`}
      />
      {value && !isValid && warningMessage && (
        <p className='text-sm text-amber-500'>⚠️ {warningMessage}</p>
      )}
    </div>
  );
}
