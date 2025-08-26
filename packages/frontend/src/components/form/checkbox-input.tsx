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
      <div className={`min-h-[60px] flex flex-col justify-center ${className}`}>
        <div className='flex items-center justify-between gap-8'>
          <label
            htmlFor={id}
            className='text-sm font-medium text-zinc-300 flex-shrink-0 min-w-[220px] flex items-center'
          >
            {label}
          </label>
          <div className='flex-1 min-w-0 max-w-sm flex justify-end items-center'>
            <input
              id={id}
              type='checkbox'
              checked={checked}
              onChange={e => onChange(e.target.checked)}
              className='h-4 w-4 text-blue-500 focus:ring-blue-500 border-zinc-600 bg-zinc-900 rounded'
            />
          </div>
        </div>
        {description && (
          <p className='text-sm text-zinc-500 ml-[calc(220px+2rem)] mt-1'>
            {description}
          </p>
        )}
      </div>
      <div className='border-b border-zinc-800/50 my-2'></div>
    </>
  );
}
