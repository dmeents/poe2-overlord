import React from 'react';

interface FormFieldProps {
  label: string | React.ReactNode;
  description?: string;
  children: React.ReactNode;
  htmlFor?: string;
  className?: string;
}

export function FormField({
  label,
  description,
  children,
  htmlFor,
  className = '',
}: FormFieldProps) {
  const isLastItem = className.includes('last-form-item');

  return (
    <>
      <div className={`min-h-[60px] flex flex-col justify-center ${className}`}>
        <div className='flex items-center justify-between gap-8'>
          <label
            htmlFor={htmlFor}
            className='text-sm font-medium text-zinc-300 flex-shrink-0 min-w-[220px] flex items-center'
          >
            {label}
          </label>
          <div className='flex-1 min-w-0 max-w-sm'>{children}</div>
        </div>
        {description && (
          <p className='text-sm text-zinc-500 ml-[calc(220px+2rem)] mt-1'>
            {description}
          </p>
        )}
      </div>
      {!isLastItem && <div className='border-b border-zinc-800/50 my-2'></div>}
    </>
  );
}
