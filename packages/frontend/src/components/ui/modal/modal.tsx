import { cn } from '@poe2-overlord/theme';
import { useEffect, useId, useRef } from 'react';
import { modalStyles } from './modal.styles';

/**
 * Reference counter for scroll lock to handle nested/stacked modals.
 * Only restores scroll when the last modal is closed.
 */
let scrollLockCount = 0;

function lockScroll(): void {
  scrollLockCount++;
  if (scrollLockCount === 1) {
    document.body.style.overflow = 'hidden';
  }
}

function unlockScroll(): void {
  scrollLockCount = Math.max(0, scrollLockCount - 1);
  if (scrollLockCount === 0) {
    document.body.style.overflow = 'unset';
  }
}

export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  children: React.ReactNode;
  size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl';
  title?: string;
  icon?: React.ReactNode;
  showCloseButton?: boolean;
  closeOnBackdropClick?: boolean;
  closeOnEscape?: boolean;
  className?: string;
  disabled?: boolean;
}

export function Modal({
  isOpen,
  onClose,
  children,
  size = 'md',
  title,
  icon,
  showCloseButton = true,
  closeOnBackdropClick = true,
  closeOnEscape = true,
  className,
  disabled = false,
}: ModalProps) {
  const modalRef = useRef<HTMLDivElement>(null);
  const previouslyFocusedElementRef = useRef<HTMLElement | null>(null);
  const titleId = useId();
  const contentId = useId();

  // Handle escape key
  useEffect(() => {
    if (!isOpen || !closeOnEscape) return;

    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && !disabled) {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [isOpen, closeOnEscape, onClose, disabled]);

  // Focus management with focus trap and return focus
  useEffect(() => {
    if (!isOpen || !modalRef.current) return;

    const modal = modalRef.current;

    // Store the element that had focus before modal opened
    previouslyFocusedElementRef.current = document.activeElement as HTMLElement;

    const focusableElements = modal.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
    );
    const firstElement = focusableElements[0] as HTMLElement;
    const lastElement = focusableElements[focusableElements.length - 1] as HTMLElement;

    // Focus first element
    if (firstElement) {
      firstElement.focus();
    }

    // Handle Tab key to trap focus within modal
    const handleTabKey = (e: KeyboardEvent) => {
      if (e.key !== 'Tab') return;

      if (e.shiftKey) {
        // Shift + Tab: if on first element, go to last
        if (document.activeElement === firstElement) {
          e.preventDefault();
          lastElement?.focus();
        }
      } else {
        // Tab: if on last element, go to first
        if (document.activeElement === lastElement) {
          e.preventDefault();
          firstElement?.focus();
        }
      }
    };

    modal.addEventListener('keydown', handleTabKey);

    // Cleanup: restore focus to previously focused element
    return () => {
      modal.removeEventListener('keydown', handleTabKey);
      if (previouslyFocusedElementRef.current?.focus) {
        previouslyFocusedElementRef.current.focus();
      }
    };
  }, [isOpen]);

  // Prevent body scroll when modal is open
  // Uses reference counting to handle nested/stacked modals correctly
  useEffect(() => {
    if (isOpen) {
      lockScroll();
      return () => unlockScroll();
    }
    return undefined;
  }, [isOpen]);

  if (!isOpen) return null;

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (closeOnBackdropClick && !disabled && e.target === e.currentTarget) {
      onClose();
    }
  };

  return (
    <div className={modalStyles.overlay}>
      {/* biome-ignore lint/a11y/noStaticElementInteractions: Backdrop click to close modal; keyboard handled via Escape key */}
      {/* biome-ignore lint/a11y/useKeyWithClickEvents: Escape key handler exists for keyboard accessibility */}
      <div className={modalStyles.container} onClick={handleBackdropClick}>
        <div className={modalStyles.backdrop} aria-hidden="true" />
        <div
          ref={modalRef}
          role="dialog"
          aria-modal="true"
          aria-labelledby={title ? titleId : undefined}
          aria-describedby={contentId}
          className={cn(modalStyles.modal, modalStyles.sizeClasses[size], className)}>
          <div className={modalStyles.content}>
            {(title || icon || showCloseButton) && (
              <div className={modalStyles.header}>
                <div className={modalStyles.headerContent}>
                  {icon && (
                    <div className={modalStyles.icon} aria-hidden="true">
                      {icon}
                    </div>
                  )}
                  {title && (
                    <h2 id={titleId} className={modalStyles.title}>
                      {title}
                    </h2>
                  )}
                </div>
                {showCloseButton && (
                  <button
                    type="button"
                    onClick={onClose}
                    className={modalStyles.closeButton}
                    disabled={disabled}
                    aria-label="Close modal">
                    <svg
                      className={modalStyles.closeIcon}
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                      aria-hidden="true">
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M6 18L18 6M6 6l12 12"
                      />
                    </svg>
                  </button>
                )}
              </div>
            )}

            {/* Content */}
            <div id={contentId}>{children}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
