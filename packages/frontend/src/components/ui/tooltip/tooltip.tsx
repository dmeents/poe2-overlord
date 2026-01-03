import { InformationCircleIcon } from '@heroicons/react/24/outline';
import { useState, useRef, useEffect } from 'react';
import { createPortal } from 'react-dom';
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
  const [position, setPosition] = useState({ top: 0, left: 0 });
  const triggerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (isVisible && triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      setPosition({
        top: rect.top + window.scrollY,
        left: rect.left + rect.width / 2 + window.scrollX,
      });
    }
  }, [isVisible]);

  const tooltipContent = isVisible && (
    <div
      className={tooltipStyles.tooltip}
      style={{
        position: 'fixed',
        top: `${position.top}px`,
        left: `${position.left}px`,
        transform: 'translate(-50%, calc(-100% - 8px))',
      }}
    >
      <div className={tooltipStyles.content}>
        {content}
        {/* Arrow pointing down */}
        <div className={tooltipStyles.arrow}></div>
      </div>
    </div>
  );

  return (
    <div className={`${tooltipStyles.container} ${className}`}>
      <div
        ref={triggerRef}
        className={tooltipStyles.trigger}
        onMouseEnter={() => setIsVisible(true)}
        onMouseLeave={() => setIsVisible(false)}
      >
        {children}
        {showIcon && <InformationCircleIcon className={tooltipStyles.icon} />}
      </div>

      {typeof document !== 'undefined' &&
        createPortal(tooltipContent, document.body)}
    </div>
  );
}
