import { type RefObject, useEffect, useLayoutEffect, useRef, useState } from 'react';

interface DropdownPosition {
  top: number;
  left: number;
  width?: number;
}

interface UseDropdownPositionOptions {
  isOpen: boolean;
  onClose: () => void;
  includeWidth?: boolean;
  enabled?: boolean;
}

interface UseDropdownPositionReturn {
  dropdownRef: RefObject<HTMLDivElement | null>;
  triggerRef: RefObject<HTMLElement | null>;
  dropdownPosition: DropdownPosition;
}

/**
 * Hook for managing dropdown positioning and click-outside behavior.
 * Calculates absolute position based on trigger element and closes dropdown when clicking outside.
 *
 * @param options - Configuration options
 * @param options.isOpen - Whether the dropdown is currently open
 * @param options.onClose - Callback to close the dropdown
 * @param options.includeWidth - Whether to include width in the position calculation (default: true)
 * @param options.enabled - Whether positioning logic is enabled (default: true)
 * @returns Refs for dropdown and trigger elements, plus calculated position
 */
export function useDropdownPosition({
  isOpen,
  onClose,
  includeWidth = true,
  enabled = true,
}: UseDropdownPositionOptions): UseDropdownPositionReturn {
  const [dropdownPosition, setDropdownPosition] = useState<DropdownPosition>({
    top: 0,
    left: 0,
    ...(includeWidth ? { width: 0 } : {}),
  });

  const dropdownRef = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLElement>(null);

  // Calculate dropdown position when opened
  useLayoutEffect(() => {
    if (!enabled || !isOpen || !triggerRef.current) return;

    const rect = triggerRef.current.getBoundingClientRect();
    setDropdownPosition({
      top: rect.bottom + window.scrollY + 8,
      left: rect.left + window.scrollX,
      ...(includeWidth ? { width: rect.width } : {}),
    });
  }, [isOpen, enabled, includeWidth]);

  // Close dropdown when clicking outside
  useEffect(() => {
    if (!enabled || !isOpen) return;

    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen, enabled, onClose]);

  return {
    dropdownRef,
    triggerRef,
    dropdownPosition,
  };
}
