import type React from 'react';
import { formFieldStyles } from './form-field.styles';

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
      <div className={`${formFieldStyles.container} ${className}`}>
        <div className={formFieldStyles.fieldContainer}>
          <label htmlFor={htmlFor} className={formFieldStyles.label}>
            {label}
          </label>
          <div className={formFieldStyles.childrenContainer}>{children}</div>
        </div>
        {description && <p className={formFieldStyles.description}>{description}</p>}
      </div>
      {!isLastItem && <div className={formFieldStyles.divider}></div>}
    </>
  );
}
