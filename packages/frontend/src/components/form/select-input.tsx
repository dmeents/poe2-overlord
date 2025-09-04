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
    <div className='relative'>
      <select
        id={id}
        value={value}
        onChange={e => onChange(e.target.value)}
        className={`w-full px-3 py-2 pr-8 border border-zinc-700 bg-zinc-900 text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 appearance-none cursor-pointer ${className}`}
      >
        {placeholder && (
          <option value='' disabled className='bg-zinc-900 text-zinc-400'>
            {placeholder}
          </option>
        )}
        {options.map(option => (
          <option
            key={option.value}
            value={option.value}
            className='bg-zinc-900 text-white'
          >
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
}
