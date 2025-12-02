import { InformationCircleIcon } from '@heroicons/react/24/outline';
import { useState } from 'react';
import { tooltipStyles } from './tooltip.styles';

interface TooltipProps {
  content: string | React.ReactNode;
  children?: React.ReactNode;
  className?: string;
  showIcon?: boolean;
}

export function Tooltip({
  content,
  children,
  className = '',
  showIcon = false,
}: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false);

  return (
    <div className={`${tooltipStyles.container} ${className}`}>
      <div
        className={tooltipStyles.trigger}
        onMouseEnter={() => setIsVisible(true)}
        onMouseLeave={() => setIsVisible(false)}
      >
        {children}
        {showIcon && <InformationCircleIcon className={tooltipStyles.icon} />}
      </div>

      {isVisible && (
        <div className={tooltipStyles.tooltip}>
          <div className={tooltipStyles.content}>
            {content}
            {/* Arrow pointing down */}
            <div className={tooltipStyles.arrow}></div>
          </div>
        </div>
      )}
    </div>
  );
}
