import { cn } from '@/utils/tailwind';
import { useEffect, useRef } from 'react';
import { modalStyles } from './modal.styles';

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

  // Focus management
  useEffect(() => {
    if (isOpen && modalRef.current) {
      const focusableElements = modalRef.current.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      );
      const firstElement = focusableElements[0] as HTMLElement;
      if (firstElement) {
        firstElement.focus();
      }
    }
  }, [isOpen]);

  // Prevent body scroll when modal is open
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }

    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [isOpen]);

  if (!isOpen) return null;

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (closeOnBackdropClick && !disabled && e.target === e.currentTarget) {
      onClose();
    }
  };

  return (
    <div className={modalStyles.overlay}>
      <div className={modalStyles.container}>
        {/* Backdrop */}
        <div className={modalStyles.backdrop} onClick={handleBackdropClick} />

        {/* Modal */}
        <div
          ref={modalRef}
          className={cn(
            modalStyles.modal,
            modalStyles.sizeClasses[size],
            className
          )}
        >
          <div className={modalStyles.content}>
            {/* Header */}
            {(title || icon || showCloseButton) && (
              <div className={modalStyles.header}>
                <div className={modalStyles.headerContent}>
                  {icon && <div className={modalStyles.icon}>{icon}</div>}
                  {title && <h2 className={modalStyles.title}>{title}</h2>}
                </div>
                {showCloseButton && (
                  <button
                    onClick={onClose}
                    className={modalStyles.closeButton}
                    disabled={disabled}
                    aria-label='Close modal'
                  >
                    <svg
                      className={modalStyles.closeIcon}
                      fill='none'
                      viewBox='0 0 24 24'
                      stroke='currentColor'
                    >
                      <path
                        strokeLinecap='round'
                        strokeLinejoin='round'
                        strokeWidth={2}
                        d='M6 18L18 6M6 6l12 12'
                      />
                    </svg>
                  </button>
                )}
              </div>
            )}

            {/* Content */}
            {children}
          </div>
        </div>
      </div>
    </div>
  );
}
