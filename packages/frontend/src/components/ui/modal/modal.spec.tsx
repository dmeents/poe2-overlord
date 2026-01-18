import { render, screen, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Modal } from './modal';

describe('Modal', () => {
  it('renders children when open', () => {
    render(
      <Modal isOpen={true} onClose={vi.fn()}>
        <div>Modal Content</div>
      </Modal>
    );

    expect(screen.getByText('Modal Content')).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    render(
      <Modal isOpen={false} onClose={vi.fn()}>
        <div>Modal Content</div>
      </Modal>
    );

    expect(screen.queryByText('Modal Content')).not.toBeInTheDocument();
  });

  it('renders title when provided', () => {
    render(
      <Modal isOpen={true} onClose={vi.fn()} title="Test Title">
        <div>Content</div>
      </Modal>
    );

    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });

  it('renders icon when provided', () => {
    render(
      <Modal
        isOpen={true}
        onClose={vi.fn()}
        icon={<span data-testid="test-icon">Icon</span>}
      >
        <div>Content</div>
      </Modal>
    );

    expect(screen.getByTestId('test-icon')).toBeInTheDocument();
  });

  it('renders close button by default', () => {
    render(
      <Modal isOpen={true} onClose={vi.fn()}>
        <div>Content</div>
      </Modal>
    );

    expect(
      screen.getByRole('button', { name: 'Close modal' })
    ).toBeInTheDocument();
  });

  it('hides close button when showCloseButton is false', () => {
    render(
      <Modal isOpen={true} onClose={vi.fn()} showCloseButton={false}>
        <div>Content</div>
      </Modal>
    );

    expect(
      screen.queryByRole('button', { name: 'Close modal' })
    ).not.toBeInTheDocument();
  });

  it('calls onClose when close button is clicked', async () => {
    const user = userEvent.setup();
    const handleClose = vi.fn();

    render(
      <Modal isOpen={true} onClose={handleClose}>
        <div>Content</div>
      </Modal>
    );

    await user.click(screen.getByRole('button', { name: 'Close modal' }));

    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when Escape is pressed', () => {
    const handleClose = vi.fn();

    render(
      <Modal isOpen={true} onClose={handleClose}>
        <div>Content</div>
      </Modal>
    );

    fireEvent.keyDown(document, { key: 'Escape' });

    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it('does not call onClose when Escape is pressed and closeOnEscape is false', () => {
    const handleClose = vi.fn();

    render(
      <Modal isOpen={true} onClose={handleClose} closeOnEscape={false}>
        <div>Content</div>
      </Modal>
    );

    fireEvent.keyDown(document, { key: 'Escape' });

    expect(handleClose).not.toHaveBeenCalled();
  });

  it('does not call onClose when disabled', async () => {
    const user = userEvent.setup();
    const handleClose = vi.fn();

    render(
      <Modal isOpen={true} onClose={handleClose} disabled>
        <div>Content</div>
      </Modal>
    );

    await user.click(screen.getByRole('button', { name: 'Close modal' }));

    expect(handleClose).not.toHaveBeenCalled();
  });

  it('applies custom className', () => {
    const { container } = render(
      <Modal isOpen={true} onClose={vi.fn()} className="custom-class">
        <div>Content</div>
      </Modal>
    );

    expect(container.querySelector('.custom-class')).toBeInTheDocument();
  });

  describe('scroll lock', () => {
    beforeEach(() => {
      // Reset body overflow before each test
      document.body.style.overflow = '';
    });

    it('locks scroll when modal opens', () => {
      render(
        <Modal isOpen={true} onClose={vi.fn()}>
          <div>Content</div>
        </Modal>
      );

      expect(document.body.style.overflow).toBe('hidden');
    });

    it('unlocks scroll when modal closes', () => {
      const { rerender } = render(
        <Modal isOpen={true} onClose={vi.fn()}>
          <div>Content</div>
        </Modal>
      );

      expect(document.body.style.overflow).toBe('hidden');

      rerender(
        <Modal isOpen={false} onClose={vi.fn()}>
          <div>Content</div>
        </Modal>
      );

      expect(document.body.style.overflow).toBe('unset');
    });

    it('handles nested modals correctly - only unlocks on last close', () => {
      const { rerender } = render(
        <>
          <Modal isOpen={true} onClose={vi.fn()}>
            <div>First Modal</div>
          </Modal>
          <Modal isOpen={true} onClose={vi.fn()}>
            <div>Second Modal</div>
          </Modal>
        </>
      );

      // Both modals open - scroll should be locked
      expect(document.body.style.overflow).toBe('hidden');

      // Close one modal
      rerender(
        <>
          <Modal isOpen={true} onClose={vi.fn()}>
            <div>First Modal</div>
          </Modal>
          <Modal isOpen={false} onClose={vi.fn()}>
            <div>Second Modal</div>
          </Modal>
        </>
      );

      // One modal still open - scroll should still be locked
      expect(document.body.style.overflow).toBe('hidden');

      // Close last modal
      rerender(
        <>
          <Modal isOpen={false} onClose={vi.fn()}>
            <div>First Modal</div>
          </Modal>
          <Modal isOpen={false} onClose={vi.fn()}>
            <div>Second Modal</div>
          </Modal>
        </>
      );

      // All modals closed - scroll should be unlocked
      expect(document.body.style.overflow).toBe('unset');
    });

    it('cleans up scroll lock on unmount', () => {
      const { unmount } = render(
        <Modal isOpen={true} onClose={vi.fn()}>
          <div>Content</div>
        </Modal>
      );

      expect(document.body.style.overflow).toBe('hidden');

      unmount();

      expect(document.body.style.overflow).toBe('unset');
    });
  });
});
