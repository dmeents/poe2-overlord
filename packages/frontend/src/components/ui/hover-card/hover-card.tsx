import { InformationCircleIcon } from '@heroicons/react/24/outline';
import { memo, useCallback, useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { hoverCardStyles } from './hover-card.styles';

/** Maps Tailwind width classes to pixel values for viewport edge clamping. */
const TAILWIND_WIDTH_PX: Record<string, number> = {
  'w-48': 192,
  'w-56': 224,
  'w-64': 256,
  'w-72': 288,
  'w-80': 320,
  'w-96': 384,
};

/** Minimum gap (px) between the card edge and the viewport edge. */
const VIEWPORT_MARGIN = 8;

interface HoverCardProps {
  /** Content rendered inside the floating card */
  content: React.ReactNode;
  /** The trigger element(s) that activate the card on mouse enter / focus */
  children: React.ReactNode;
  /** Additional classes applied to the outer wrapper div */
  className?: string;
  /** Tailwind width class for the floating card (default: 'w-80') */
  width?: string;
  /** Milliseconds to wait before showing after hover begins (default: 0) */
  showDelay?: number;
  /** Show an info circle icon after children (default: false) */
  showIcon?: boolean;
  /**
   * Fires immediately when hover intent begins (true) or ends (false),
   * before showDelay elapses. Use this to kick off deferred data fetching
   * while the user waits for the card to appear.
   */
  onOpenChange?: (open: boolean) => void;
}

/**
 * Unified hover card component for rich hover-triggered content.
 *
 * Features:
 * - Configurable show delay (default 0ms)
 * - Automatically repositions on scroll/resize while visible
 * - Uses capture-phase scroll listening to catch scroll on any ancestor
 * - Renders in portal to avoid z-index and overflow issues
 * - Keyboard accessible (focus/blur)
 * - `onOpenChange` fires before the delay for deferred data loading
 * - When `content` is falsy, just renders children with no hover behavior
 *
 * @example
 * // Simple tooltip (zero delay)
 * <HoverCard content="Detailed information">
 *   <button>Hover me</button>
 * </HoverCard>
 *
 * @example
 * // Rich popover with delay and deferred loading
 * <HoverCard
 *   content={<RichContent data={data} />}
 *   showDelay={200}
 *   width="w-56"
 *   onOpenChange={(open) => { if (open) startFetching(); }}
 * >
 *   <img src={icon} alt="item" />
 * </HoverCard>
 */
export const HoverCard = memo(function HoverCard({
  content,
  children,
  className = '',
  width = 'w-80',
  showDelay = 0,
  showIcon = false,
  onOpenChange,
}: HoverCardProps) {
  const [position, setPosition] = useState<{
    top: number;
    /** Clamped center-X used for `left` in fixed positioning. */
    left: number;
    /** How far the arrow needs to shift from card-center to point at the trigger. */
    arrowOffset: number;
  } | null>(null);
  const [isVisible, setIsVisible] = useState(false);
  const triggerRef = useRef<HTMLDivElement>(null);
  const showTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearTimer = useCallback(() => {
    if (showTimerRef.current) {
      clearTimeout(showTimerRef.current);
      showTimerRef.current = null;
    }
  }, []);

  const calculatePosition = useCallback(() => {
    if (!triggerRef.current) return null;
    const rect = triggerRef.current.getBoundingClientRect();
    const triggerCenterX = rect.left + rect.width / 2;

    // Clamp the card's center-X so it stays within the viewport.
    // Uses translate(-50%) semantics: left is the horizontal center of the card.
    const cardWidth = TAILWIND_WIDTH_PX[width] ?? 320;
    const halfCard = cardWidth / 2;
    const clampedLeft = Math.max(
      halfCard + VIEWPORT_MARGIN,
      Math.min(window.innerWidth - halfCard - VIEWPORT_MARGIN, triggerCenterX),
    );

    // The arrow must point at the trigger's center, not the card's center.
    // arrowOffset shifts the arrow horizontally relative to card-center.
    const arrowOffset = triggerCenterX - clampedLeft;

    return { top: rect.top, left: clampedLeft, arrowOffset };
  }, [width]);

  const showCard = useCallback(() => {
    const pos = calculatePosition();
    if (!pos) return;
    // Set position immediately so content can start rendering / loading
    setPosition(pos);
    onOpenChange?.(true);

    if (showDelay > 0) {
      showTimerRef.current = setTimeout(() => setIsVisible(true), showDelay);
    } else {
      setIsVisible(true);
    }
  }, [calculatePosition, onOpenChange, showDelay]);

  const hideCard = useCallback(() => {
    clearTimer();
    setIsVisible(false);
    onOpenChange?.(false);
  }, [clearTimer, onOpenChange]);

  // Reposition on scroll/resize while visible
  useEffect(() => {
    if (!isVisible) return;

    const updatePosition = () => {
      const pos = calculatePosition();
      if (pos) setPosition(pos);
    };

    window.addEventListener('scroll', updatePosition, true);
    window.addEventListener('resize', updatePosition);

    return () => {
      window.removeEventListener('scroll', updatePosition, true);
      window.removeEventListener('resize', updatePosition);
    };
  }, [isVisible, calculatePosition]);

  // Clean up timer on unmount
  useEffect(() => {
    return () => clearTimer();
  }, [clearTimer]);

  // Don't attach any hover behaviour if there's nothing to show
  if (!content) {
    return <>{children}</>;
  }

  const cardNode = isVisible && position && (
    <div
      role="tooltip"
      className={`${hoverCardStyles.card} ${width}`}
      style={{
        position: 'fixed',
        top: `${position.top}px`,
        left: `${position.left}px`,
        transform: 'translate(-50%, calc(-100% - 8px))',
      }}>
      <div className={hoverCardStyles.content}>
        {content}
        {/* Arrow points at the trigger center, shifted when the card is clamped. */}
        <div
          className={hoverCardStyles.arrow}
          style={
            position.arrowOffset !== 0
              ? { marginLeft: `${position.arrowOffset}px` }
              : undefined
          }
        />
      </div>
    </div>
  );

  return (
    <div className={`${hoverCardStyles.container} ${className}`}>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: HoverCard trigger uses mouse/focus events for hover interaction */}
      <div
        ref={triggerRef}
        className={hoverCardStyles.trigger}
        onMouseEnter={showCard}
        onMouseLeave={hideCard}
        onFocus={showCard}
        onBlur={hideCard}>
        {children}
        {showIcon && <InformationCircleIcon className={hoverCardStyles.icon} aria-hidden="true" />}
      </div>

      {typeof document !== 'undefined' && createPortal(cardNode, document.body)}
    </div>
  );
});
