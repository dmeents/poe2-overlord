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
    error: 'bg-red-950/20 border border-red-800 text-red-400',
    success: 'bg-green-950/20 border border-green-800 text-green-400',
  };

  return (
    <div className={`px-4 py-3 ${alertClasses[type]} ${className}`}>
      {message}
    </div>
  );
}
