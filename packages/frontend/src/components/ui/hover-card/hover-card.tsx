import {
  arrow,
  autoUpdate,
  FloatingArrow,
  FloatingPortal,
  flip,
  offset,
  safePolygon,
  shift,
  size,
  useFloating,
  useFocus,
  useHover,
  useInteractions,
  useRole,
  useTransitionStyles,
} from '@floating-ui/react';
import { InformationCircleIcon } from '@heroicons/react/24/outline';
import { memo, useCallback, useRef, useState } from 'react';
import { hoverCardStyles } from './hover-card.styles';

/** Pixel height of the arrow tip — keeps the gap consistent with the offset. */
const ARROW_HEIGHT = 6;

interface HoverCardProps {
  /** Content rendered inside the floating card */
  content: React.ReactNode;
  /** The trigger element(s) that activate the card on mouse enter / focus */
  children: React.ReactNode;
  /** Additional classes applied to the outer wrapper div */
  className?: string;
  /** Tailwind width class applied to the floating card (default: 'w-80') */
  width?: string;
  /** Milliseconds to wait before showing after hover begins (default: 0) */
  showDelay?: number;
  /** Show an info circle icon after children (default: false) */
  showIcon?: boolean;
  /**
   * Fires immediately when hover intent begins (true) or ends (false),
   * BEFORE showDelay elapses. Use this to kick off deferred data fetching
   * while the user waits for the card to appear.
   */
  onOpenChange?: (open: boolean) => void;
}

/**
 * Unified hover card component powered by @floating-ui/react.
 *
 * Features:
 * - Auto-flip: switches to bottom placement when there is no space above
 * - Auto-shift: slides along the viewport edge to stay fully in view
 * - autoUpdate: repositions on scroll, resize, and ResizeObserver — no manual listeners
 * - safePolygon: cursor can move from trigger to card without it closing
 * - Keyboard accessible: focus/blur open/close
 * - Configurable show delay (default 0ms)
 * - onOpenChange fires immediately on hover start (before showDelay) for deferred loading
 * - When content is falsy, renders children as-is with no hover machinery attached
 *
 * @example
 * // Simple tooltip (zero delay)
 * <HoverCard content="Detailed information">
 *   <button>Hover me</button>
 * </HoverCard>
 *
 * @example
 * // Rich card with delay and deferred data loading
 * <HoverCard
 *   content={<RichContent data={data} />}
 *   showDelay={200}
 *   width="w-72"
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
  const [isOpen, setIsOpen] = useState(false);
  const [maxHeight, setMaxHeight] = useState<number | undefined>(undefined);
  const arrowRef = useRef<SVGSVGElement>(null);

  const { refs, floatingStyles, context } = useFloating({
    open: isOpen,
    onOpenChange: setIsOpen,
    placement: 'top',
    strategy: 'fixed',
    // transform: false — use left/top instead of translate() for positioning.
    // The default transform:true produces floatingStyles.transform='translate(X,Y)'.
    // Spreading transitionStyles on top overwrites that with scale(…), collapsing
    // the position to (0,0). With transform:false, left/top carry the position and
    // transitionStyles.transform is free to animate scale independently.
    transform: false,
    middleware: [
      offset(ARROW_HEIGHT + 2),
      flip({ padding: 8 }),
      shift({ padding: 8 }),
      size({
        padding: 8,
        apply({ availableHeight }) {
          setMaxHeight(availableHeight);
        },
      }),
      arrow({ element: arrowRef, padding: 6 }),
    ],
    whileElementsMounted: autoUpdate,
  });

  const hover = useHover(context, {
    delay: { open: showDelay, close: 0 },
    handleClose: safePolygon(),
  });
  const focus = useFocus(context, { visibleOnly: false });
  const role = useRole(context, { role: 'tooltip' });

  const { getReferenceProps, getFloatingProps } = useInteractions([hover, focus, role]);

  // Fire consumer's onOpenChange immediately on pointer enter/leave — before
  // showDelay elapses — so deferred data fetching can start right away.
  // useHover uses pointer events, so we mirror that here.
  const handlePointerEnter = useCallback(() => onOpenChange?.(true), [onOpenChange]);
  const handlePointerLeave = useCallback(() => onOpenChange?.(false), [onOpenChange]);

  const { isMounted, styles: transitionStyles } = useTransitionStyles(context, {
    initial: { opacity: 0, transform: 'scale(0.95)' },
    open: { opacity: 1, transform: 'scale(1)' },
    // Open has a subtle animation; close is immediate so the element unmounts
    // right away (avoids keeping stale portal content in the DOM).
    duration: { open: 120, close: 0 },
  });

  // No hover machinery when there is nothing to show
  if (!content) {
    return <>{children}</>;
  }

  return (
    <div className={`${hoverCardStyles.container} ${className}`}>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: HoverCard trigger uses mouse/focus events for hover interaction */}
      <div
        ref={refs.setReference}
        className={hoverCardStyles.trigger}
        {...getReferenceProps({
          onPointerEnter: handlePointerEnter,
          onPointerLeave: handlePointerLeave,
        })}>
        {children}
        {showIcon && <InformationCircleIcon className={hoverCardStyles.icon} aria-hidden="true" />}
      </div>

      {isMounted && (
        <FloatingPortal>
          <div
            ref={refs.setFloating}
            className={`${hoverCardStyles.card} ${width}`}
            style={{
              ...floatingStyles,
              ...transitionStyles,
              maxHeight: maxHeight !== undefined ? `${maxHeight}px` : undefined,
              overflowY: 'auto',
            }}
            {...getFloatingProps()}>
            {content}
            <FloatingArrow
              ref={arrowRef}
              context={context}
              className={hoverCardStyles.arrow}
              height={ARROW_HEIGHT}
              width={12}
              tipRadius={1}
            />
          </div>
        </FloatingPortal>
      )}
    </div>
  );
});
