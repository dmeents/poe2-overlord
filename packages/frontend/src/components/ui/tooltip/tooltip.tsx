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

/**
 * Tooltip component with automatic scroll repositioning.
 *
 * Features:
 * - Automatically repositions on scroll/resize while visible
 * - Uses capture phase event listening to catch scroll on any ancestor
 * - Renders in portal to avoid z-index and overflow issues
 * - Uses fixed positioning with viewport coordinates
 *
 * @example
 * <Tooltip content="Detailed information">
 *   <button>Hover me</button>
 * </Tooltip>
 */
export function Tooltip({
  content,
  children,
  className = '',
  showIcon = false,
}: TooltipProps) {
  // Use null to indicate tooltip is hidden; position is calculated before showing
  const [position, setPosition] = useState<{ top: number; left: number } | null>(
    null
  );
  const triggerRef = useRef<HTMLDivElement>(null);

  // Show tooltip: calculate position first, then display
  const showTooltip = () => {
    if (triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      // For fixed positioning, use viewport coordinates directly (no scroll offset needed)
      setPosition({
        top: rect.top,
        left: rect.left + rect.width / 2,
      });
    }
  };

  // Hide tooltip
  const hideTooltip = () => {
    setPosition(null);
  };

  // Reposition on scroll/resize while visible
  useEffect(() => {
    if (!position || !triggerRef.current) return;

    const updatePosition = () => {
      if (triggerRef.current) {
        const rect = triggerRef.current.getBoundingClientRect();
        setPosition({
          top: rect.top,
          left: rect.left + rect.width / 2,
        });
      }
    };

    // Use capture phase to catch scroll events on any element
    window.addEventListener('scroll', updatePosition, true);
    window.addEventListener('resize', updatePosition);

    return () => {
      window.removeEventListener('scroll', updatePosition, true);
      window.removeEventListener('resize', updatePosition);
    };
  }, [position]);

  // Only render when position is calculated (prevents flash at 0,0)
  const tooltipContent = position && (
    <div
      role="tooltip"
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
        onMouseEnter={showTooltip}
        onMouseLeave={hideTooltip}
      >
        {children}
        {showIcon && (
          <InformationCircleIcon
            className={tooltipStyles.icon}
            aria-hidden="true"
          />
        )}
      </div>

      {typeof document !== 'undefined' &&
        createPortal(tooltipContent, document.body)}
    </div>
  );
}
