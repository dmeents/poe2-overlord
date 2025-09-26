import { loadingSpinnerStyles } from './loading-spinner.styles';

interface LoadingSpinnerProps {
  message?: string;
  className?: string;
}

export function LoadingSpinner({
  message = 'Loading...',
  className = '',
}: LoadingSpinnerProps) {
  return (
    <div className={`${loadingSpinnerStyles.container} ${className}`}>
      <div className={loadingSpinnerStyles.spinner}></div>
      <span className={loadingSpinnerStyles.message}>{message}</span>
    </div>
  );
}
