import { act, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { HoverCard } from './hover-card';

describe('HoverCard', () => {
  it('renders children correctly', () => {
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    expect(screen.getByText('Hover me')).toBeInTheDocument();
  });

  it('shows card content on hover', () => {
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    fireEvent.mouseEnter(screen.getByText('Hover me'));

    expect(screen.getByText('Card text')).toBeInTheDocument();
  });

  it('hides card content on mouse leave', () => {
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    const trigger = screen.getByText('Hover me');
    fireEvent.mouseEnter(trigger);
    fireEvent.mouseLeave(trigger);

    expect(screen.queryByText('Card text')).not.toBeInTheDocument();
  });

  it('renders ReactNode content', () => {
    render(
      <HoverCard content={<span data-testid="custom-content">Custom</span>}>
        <span>Hover me</span>
      </HoverCard>,
    );

    fireEvent.mouseEnter(screen.getByText('Hover me'));

    expect(screen.getByTestId('custom-content')).toBeInTheDocument();
  });

  it('applies custom className to wrapper', () => {
    const { container } = render(
      <HoverCard content="Card text" className="custom-class">
        <span>Hover me</span>
      </HoverCard>,
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('shows info icon when showIcon is true', () => {
    const { container } = render(
      <HoverCard content="Card text" showIcon>
        <span>Hover me</span>
      </HoverCard>,
    );

    expect(container.querySelector('svg')).toBeInTheDocument();
  });

  it('does not show icon by default', () => {
    const { container } = render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    expect(container.querySelector('svg')).not.toBeInTheDocument();
  });

  it('shows on focus and hides on blur (keyboard accessible)', () => {
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    const trigger = screen.getByText('Hover me');
    fireEvent.focus(trigger);
    expect(screen.getByText('Card text')).toBeInTheDocument();

    fireEvent.blur(trigger);
    expect(screen.queryByText('Card text')).not.toBeInTheDocument();
  });

  it('renders nothing but children when content is falsy', () => {
    const { container } = render(
      <HoverCard content={null}>
        <span>Hover me</span>
      </HoverCard>,
    );

    // Should not render the wrapper div with hover machinery
    expect(screen.getByText('Hover me')).toBeInTheDocument();
    expect(container.querySelector('[onmouseenter]')).not.toBeInTheDocument();
  });

  describe('showDelay', () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('does not show content before delay elapses', () => {
      render(
        <HoverCard content="Card text" showDelay={200}>
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      // Content should not be visible yet
      expect(screen.queryByText('Card text')).not.toBeInTheDocument();

      // Advance timer past delay
      act(() => {
        vi.advanceTimersByTime(200);
      });

      expect(screen.getByText('Card text')).toBeInTheDocument();
    });

    it('cancels show when mouse leaves before delay elapses', () => {
      render(
        <HoverCard content="Card text" showDelay={200}>
          <span>Hover me</span>
        </HoverCard>,
      );

      const trigger = screen.getByText('Hover me');
      fireEvent.mouseEnter(trigger);
      fireEvent.mouseLeave(trigger);

      act(() => {
        vi.advanceTimersByTime(200);
      });

      expect(screen.queryByText('Card text')).not.toBeInTheDocument();
    });

    it('fires onOpenChange immediately on hover, before delay elapses', () => {
      const onOpenChange = vi.fn();

      render(
        <HoverCard content="Card text" showDelay={200} onOpenChange={onOpenChange}>
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      // Should have fired immediately, before the 200ms delay
      expect(onOpenChange).toHaveBeenCalledWith(true);
      expect(screen.queryByText('Card text')).not.toBeInTheDocument();
    });

    it('fires onOpenChange(false) on mouse leave', () => {
      const onOpenChange = vi.fn();

      render(
        <HoverCard content="Card text" showDelay={200} onOpenChange={onOpenChange}>
          <span>Hover me</span>
        </HoverCard>,
      );

      const trigger = screen.getByText('Hover me');
      fireEvent.mouseEnter(trigger);
      fireEvent.mouseLeave(trigger);

      expect(onOpenChange).toHaveBeenCalledWith(false);
    });
  });

  describe('Scroll Repositioning', () => {
    it('registers scroll and resize listeners when card is visible', () => {
      const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
      const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

      render(
        <HoverCard content="Card text">
          <span>Hover me</span>
        </HoverCard>,
      );

      const trigger = screen.getByText('Hover me');

      fireEvent.mouseEnter(trigger);

      expect(addEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
      expect(addEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

      fireEvent.mouseLeave(trigger);

      expect(removeEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
      expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

      addEventListenerSpy.mockRestore();
      removeEventListenerSpy.mockRestore();
    });

    it('uses fixed positioning for the card', () => {
      render(
        <HoverCard content="Card text">
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      const card = screen.getByRole('tooltip');
      expect(card).toHaveStyle({ position: 'fixed' });
    });

    it('renders card in portal to document.body', () => {
      render(
        <HoverCard content="Card text">
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      const card = screen.getByRole('tooltip');
      expect(card.parentElement).toBe(document.body);
    });

    it('cleans up event listeners on unmount', () => {
      const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

      const { unmount } = render(
        <HoverCard content="Card text">
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      unmount();

      expect(removeEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
      expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

      removeEventListenerSpy.mockRestore();
    });
  });

  describe('custom width', () => {
    it('applies default width class when not specified', () => {
      render(
        <HoverCard content="Card text">
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      const card = screen.getByRole('tooltip');
      expect(card).toHaveClass('w-80');
    });

    it('applies custom width class when specified', () => {
      render(
        <HoverCard content="Card text" width="w-56">
          <span>Hover me</span>
        </HoverCard>,
      );

      fireEvent.mouseEnter(screen.getByText('Hover me'));

      const card = screen.getByRole('tooltip');
      expect(card).toHaveClass('w-56');
    });
  });
});
