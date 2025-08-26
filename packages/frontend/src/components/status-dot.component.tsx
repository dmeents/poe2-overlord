interface StatusDotProps {
  isOnline: boolean;
  size?: 'sm' | 'md' | 'lg';
}

export function StatusDot({ isOnline, size = 'md' }: StatusDotProps) {
  return (
    <div>
      <div data-online={isOnline} data-size={size}></div>
    </div>
  );
}
