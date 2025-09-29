interface TimeDisplayProps {
  seconds: number;
  showSeconds?: boolean;
  className?: string;
}

export function TimeDisplay({
  seconds,
  showSeconds = true,
  className = '',
}: TimeDisplayProps) {
  const formatTime = (totalSeconds: number): string => {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const secs = totalSeconds % 60;

    if (hours > 0) {
      if (showSeconds) {
        return `${hours}h ${minutes}m ${secs}s`;
      }
      return `${hours}h ${minutes}m`;
    } else if (minutes > 0) {
      if (showSeconds) {
        return `${minutes}m ${secs}s`;
      }
      return `${minutes}m`;
    } else {
      return `${secs}s`;
    }
  };

  return <span className={className}>{formatTime(seconds)}</span>;
}
