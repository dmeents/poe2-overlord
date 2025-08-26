interface LoadingSpinnerProps {
  message?: string;
  className?: string;
}

export function LoadingSpinner({
  message = 'Loading...',
  className = '',
}: LoadingSpinnerProps) {
  return (
    <div className={`flex items-center justify-center p-8 ${className}`}>
      <div className='animate-spin h-8 w-8 border-b-2 border-blue-500'></div>
      <span className='ml-2 text-zinc-400'>{message}</span>
    </div>
  );
}
