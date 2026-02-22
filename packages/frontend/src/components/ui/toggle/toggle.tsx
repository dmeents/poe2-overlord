import { cn } from '@poe2-overlord/theme';
import { toggleStyles } from './toggle.styles';

interface ToggleProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  size?: 'sm' | 'md';
  id?: string;
  className?: string;
}

export function Toggle({
  checked,
  onChange,
  disabled = false,
  size = 'md',
  id,
  className,
}: ToggleProps) {
  const sizeStyles = toggleStyles.sizes[size];

  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      id={id}
      disabled={disabled}
      onClick={() => onChange(!checked)}
      className={cn(
        toggleStyles.base,
        sizeStyles.track,
        checked ? toggleStyles.track.on : toggleStyles.track.off,
        className,
      )}>
      <span
        className={cn(
          toggleStyles.thumb,
          sizeStyles.thumb,
          checked ? sizeStyles.translate : 'translate-x-0',
        )}
      />
    </button>
  );
}
