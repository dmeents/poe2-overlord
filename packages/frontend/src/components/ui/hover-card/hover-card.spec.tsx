import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { HoverCard } from './hover-card';

// @floating-ui/react's useHover uses setTimeout(fn, 0) internally even for
// zero-delay — flush pending state updates with `await act(async () => {})`.

describe('HoverCard', () => {
  it('renders children correctly', () => {
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    expect(screen.getByText('Hover me')).toBeInTheDocument();
  });

  it('shows card content on hover', async () => {
    const user = userEvent.setup();
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    await user.hover(screen.getByText('Hover me'));

    expect(screen.getByText('Card text')).toBeInTheDocument();
  });

  it('hides card content on mouse leave', async () => {
    const user = userEvent.setup();
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    await user.hover(screen.getByText('Hover me'));
    expect(screen.getByText('Card text')).toBeInTheDocument();

    await user.unhover(screen.getByText('Hover me'));
    expect(screen.queryByText('Card text')).not.toBeInTheDocument();
  });

  it('renders ReactNode content', async () => {
    const user = userEvent.setup();
    render(
      <HoverCard content={<span data-testid="custom-content">Custom</span>}>
        <span>Hover me</span>
      </HoverCard>,
    );

    await user.hover(screen.getByText('Hover me'));

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

  it('shows on focus and hides on blur (keyboard accessible)', async () => {
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    // useFocus handlers are attached to the reference div via getReferenceProps,
    // not to the inner span. Fire events on the parent element.
    // waitFor is required because useTransitionStyles advances state via
    // requestAnimationFrame (polyfilled as setTimeout in jsdom), which act() alone
    // does not reliably flush.
    const trigger = screen.getByText('Hover me');
    const referenceEl = trigger.parentElement!;

    fireEvent.focus(referenceEl);
    await waitFor(() => expect(screen.getByText('Card text')).toBeInTheDocument());

    fireEvent.blur(referenceEl);
    await waitFor(() => expect(screen.queryByText('Card text')).not.toBeInTheDocument());
  });

  it('renders only children when content is falsy', () => {
    render(
      <HoverCard content={null}>
        <span>Hover me</span>
      </HoverCard>,
    );

    expect(screen.getByText('Hover me')).toBeInTheDocument();
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();
  });

  it('renders card with tooltip role', async () => {
    const user = userEvent.setup();
    render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    await user.hover(screen.getByText('Hover me'));

    expect(screen.getByRole('tooltip')).toBeInTheDocument();
  });

  it('renders card in a portal (outside component tree)', async () => {
    const user = userEvent.setup();
    const { container } = render(
      <HoverCard content="Card text">
        <span>Hover me</span>
      </HoverCard>,
    );

    await user.hover(screen.getByText('Hover me'));

    // The card should NOT be inside the component's own container
    expect(container.querySelector('[role="tooltip"]')).not.toBeInTheDocument();
    // But it should be findable in the document
    expect(screen.getByRole('tooltip')).toBeInTheDocument();
  });

  describe('custom width', () => {
    it('applies default width class when not specified', async () => {
      const user = userEvent.setup();
      render(
        <HoverCard content="Card text">
          <span>Hover me</span>
        </HoverCard>,
      );

      await user.hover(screen.getByText('Hover me'));

      expect(screen.getByRole('tooltip')).toHaveClass('w-80');
    });

    it('applies custom width class when specified', async () => {
      const user = userEvent.setup();
      render(
        <HoverCard content="Card text" width="w-72">
          <span>Hover me</span>
        </HoverCard>,
      );

      await user.hover(screen.getByText('Hover me'));

      expect(screen.getByRole('tooltip')).toHaveClass('w-72');
    });
  });

  describe('showDelay', () => {
    // These tests use real timers. vi.useFakeTimers() replaces setTimeout globally,
    // which breaks React's internal scheduler (also uses setTimeout for jsdom). The
    // delay is kept small (50ms) so tests stay fast while still being meaningful.

    it('does not show content before delay elapses', async () => {
      // user.hover() fires pointerover (bubbling) which is required to reach
      // @floating-ui/react's native event listener and start the delay timer.
      // fireEvent.pointerEnter dispatches pointerenter (non-bubbling) which only
      // hits React's synthetic layer — triggering our onOpenChange callback but
      // not the library's internal hover timer.
      const user = userEvent.setup();
      render(
        <HoverCard content="Card text" showDelay={200}>
          <span>Hover me</span>
        </HoverCard>,
      );

      // Start hover — events are dispatched asynchronously by userEvent
      const hoverPromise = user.hover(screen.getByText('Hover me'));

      // Before any events fire: card must not be shown
      expect(screen.queryByText('Card text')).not.toBeInTheDocument();

      // Events fire and React settles — 200ms timer has started but not elapsed
      await hoverPromise;
      expect(screen.queryByText('Card text')).not.toBeInTheDocument();

      // Card must appear after the 200ms delay elapses
      await waitFor(() => expect(screen.getByText('Card text')).toBeInTheDocument(), {
        timeout: 1000,
      });
    });

    it('cancels show when mouse leaves before delay elapses', async () => {
      // 500ms delay gives a wide margin: hover+unhover finishes in <50ms so the
      // timer is always cancelled before it fires.
      const user = userEvent.setup();
      render(
        <HoverCard content="Card text" showDelay={500}>
          <span>Hover me</span>
        </HoverCard>,
      );

      const trigger = screen.getByText('Hover me');
      await user.hover(trigger);
      await user.unhover(trigger);

      expect(screen.queryByText('Card text')).not.toBeInTheDocument();
    });

    it('fires onOpenChange immediately on hover, before delay elapses', () => {
      const onOpenChange = vi.fn();
      render(
        <HoverCard content="Card text" showDelay={200} onOpenChange={onOpenChange}>
          <span>Hover me</span>
        </HoverCard>,
      );

      const referenceEl = screen.getByText('Hover me').parentElement!;

      // onPointerEnter fires our handlePointerEnter synchronously — before any delay
      fireEvent.pointerEnter(referenceEl);

      expect(onOpenChange).toHaveBeenCalledWith(true);
      // Card must not appear yet (delay hasn't elapsed)
      expect(screen.queryByText('Card text')).not.toBeInTheDocument();
    });

    it('fires onOpenChange(false) on mouse leave', async () => {
      const onOpenChange = vi.fn();
      render(
        <HoverCard content="Card text" showDelay={200} onOpenChange={onOpenChange}>
          <span>Hover me</span>
        </HoverCard>,
      );

      const referenceEl = screen.getByText('Hover me').parentElement!;

      fireEvent.pointerEnter(referenceEl);
      await act(async () => {});

      fireEvent.pointerLeave(referenceEl);
      await act(async () => {});

      expect(onOpenChange).toHaveBeenCalledWith(false);
    });
  });
});
