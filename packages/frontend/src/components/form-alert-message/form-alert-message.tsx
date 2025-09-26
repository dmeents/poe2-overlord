import { formAlertMessageStyles } from './form-alert-message.styles';

interface AlertMessageProps {
  type: 'error' | 'success';
  message: string;
  className?: string;
}

export function AlertMessage({
  type,
  message,
  className = '',
}: AlertMessageProps) {
  if (!message) return null;

  const alertClasses = {
    error: formAlertMessageStyles.error,
    success: formAlertMessageStyles.success,
  };

  return (
    <div
      className={`${formAlertMessageStyles.container} ${alertClasses[type]} ${className}`}
    >
      {message}
    </div>
  );
}
