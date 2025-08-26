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
}

export function SelectInput({
  id,
  value,
  onChange,
  options,
  className = '',
}: SelectInputProps) {
  return (
    <select
      id={id}
      value={value}
      onChange={e => onChange(e.target.value)}
      className={`w-full px-3 py-2 border border-zinc-700 bg-zinc-900 text-zinc-100 shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${className}`}
    >
      {options.map(option => (
        <option
          key={option.value}
          value={option.value}
          className='bg-zinc-900 text-zinc-100'
        >
          {option.label}
        </option>
      ))}
    </select>
  );
}
