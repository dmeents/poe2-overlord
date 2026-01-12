import { render, screen, fireEvent } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { Tooltip } from './tooltip';

describe('Tooltip', () => {
  it('renders children correctly', () => {
    render(
      <Tooltip content="Tooltip text">
        <span>Hover me</span>
      </Tooltip>
    );

    expect(screen.getByText('Hover me')).toBeInTheDocument();
  });

  it('shows tooltip content on hover', () => {
    render(
      <Tooltip content="Tooltip text">
        <span>Hover me</span>
      </Tooltip>
    );

    fireEvent.mouseEnter(screen.getByText('Hover me'));

    expect(screen.getByText('Tooltip text')).toBeInTheDocument();
  });

  it('hides tooltip content on mouse leave', () => {
    render(
      <Tooltip content="Tooltip text">
        <span>Hover me</span>
      </Tooltip>
    );

    const trigger = screen.getByText('Hover me');
    fireEvent.mouseEnter(trigger);
    fireEvent.mouseLeave(trigger);

    expect(screen.queryByText('Tooltip text')).not.toBeInTheDocument();
  });

  it('renders ReactNode content', () => {
    render(
      <Tooltip content={<span data-testid="custom-content">Custom</span>}>
        <span>Hover me</span>
      </Tooltip>
    );

    fireEvent.mouseEnter(screen.getByText('Hover me'));

    expect(screen.getByTestId('custom-content')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <Tooltip content="Tooltip text" className="custom-class">
        <span>Hover me</span>
      </Tooltip>
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('shows info icon when showIcon is true', () => {
    const { container } = render(
      <Tooltip content="Tooltip text" showIcon>
        <span>Hover me</span>
      </Tooltip>
    );

    // Check for SVG icon presence
    expect(container.querySelector('svg')).toBeInTheDocument();
  });

  it('does not show icon by default', () => {
    const { container } = render(
      <Tooltip content="Tooltip text">
        <span>Hover me</span>
      </Tooltip>
    );

    // No SVG icon by default
    expect(container.querySelector('svg')).not.toBeInTheDocument();
  });

  describe('Scroll Repositioning', () => {
    it('registers scroll and resize listeners when tooltip is visible', () => {
      const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
      const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

      render(
        <Tooltip content="Tooltip text">
          <span>Hover me</span>
        </Tooltip>
      );

      const trigger = screen.getByText('Hover me');

      // Show tooltip
      fireEvent.mouseEnter(trigger);

      // Verify scroll listener added with capture phase
      expect(addEventListenerSpy).toHaveBeenCalledWith(
        'scroll',
        expect.any(Function),
        true
      );
      // Verify resize listener added
      expect(addEventListenerSpy).toHaveBeenCalledWith(
        'resize',
        expect.any(Function)
      );

      // Hide tooltip
      fireEvent.mouseLeave(trigger);

      // Verify listeners removed
      expect(removeEventListenerSpy).toHaveBeenCalledWith(
        'scroll',
        expect.any(Function),
        true
      );
      expect(removeEventListenerSpy).toHaveBeenCalledWith(
        'resize',
        expect.any(Function)
      );

      addEventListenerSpy.mockRestore();
      removeEventListenerSpy.mockRestore();
    });

    it('uses fixed positioning for tooltip', () => {
      render(
        <Tooltip content="Tooltip text">
          <span>Hover me</span>
        </Tooltip>
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      const tooltip = screen.getByRole('tooltip');
      expect(tooltip).toHaveStyle({ position: 'fixed' });
    });

    it('renders tooltip in portal to document.body', () => {
      render(
        <Tooltip content="Tooltip text">
          <span>Hover me</span>
        </Tooltip>
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      const tooltip = screen.getByRole('tooltip');
      // Tooltip should be a direct child of document.body (rendered in portal)
      expect(tooltip.parentElement).toBe(document.body);
    });

    it('cleans up event listeners on unmount', () => {
      const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

      const { unmount } = render(
        <Tooltip content="Tooltip text">
          <span>Hover me</span>
        </Tooltip>
      );

      // Show tooltip
      fireEvent.mouseEnter(screen.getByText('Hover me'));

      // Unmount while tooltip is visible
      unmount();

      // Verify listeners were cleaned up
      expect(removeEventListenerSpy).toHaveBeenCalledWith(
        'scroll',
        expect.any(Function),
        true
      );
      expect(removeEventListenerSpy).toHaveBeenCalledWith(
        'resize',
        expect.any(Function)
      );

      removeEventListenerSpy.mockRestore();
    });
  });
});
