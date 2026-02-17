import type { CSSProperties, ReactNode, RefObject } from 'react';
import { createPortal } from 'react-dom';

interface DropdownPortalProps {
  isOpen: boolean;
  dropdownRef: RefObject<HTMLDivElement | null>;
  position: { top: number; left: number; width?: number };
  className?: string;
  style?: CSSProperties;
  children: ReactNode;
}

/**
 * Shared dropdown portal component for rendering dropdowns outside the DOM hierarchy.
 *
 * Renders dropdowns using React portals to document.body to avoid positioning issues
 * with CSS transforms on parent elements (see ADR-006).
 *
 * @example
 * <DropdownPortal
 *   isOpen={isOpen}
 *   dropdownRef={dropdownRef}
 *   position={dropdownPosition}
 *   className="bg-stone-800 border border-stone-700"
 *   style={{ minWidth: '300px' }}>
 *   <div>Dropdown content</div>
 * </DropdownPortal>
 */
export function DropdownPortal({
  isOpen,
  dropdownRef,
  position,
  className = '',
  style = {},
  children,
}: DropdownPortalProps) {
  if (!isOpen || typeof document === 'undefined') {
    return null;
  }

  return createPortal(
    <div
      ref={dropdownRef}
      className={className}
      style={{
        position: 'fixed',
        top: `${position.top}px`,
        left: `${position.left}px`,
        ...(position.width !== undefined ? { minWidth: `${position.width}px` } : {}),
        ...style,
      }}>
      {children}
    </div>,
    document.body,
  );
}
