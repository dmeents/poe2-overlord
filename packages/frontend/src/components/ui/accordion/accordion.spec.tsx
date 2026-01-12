import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Accordion } from './accordion';

describe('Accordion', () => {
  it('renders title correctly', () => {
    render(
      <Accordion title="Test Title" isExpanded={false} onToggle={vi.fn()}>
        <div>Content</div>
      </Accordion>
    );

    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });

  it('renders subtitle when provided', () => {
    render(
      <Accordion
        title="Test Title"
        subtitle="Test Subtitle"
        isExpanded={false}
        onToggle={vi.fn()}
      >
        <div>Content</div>
      </Accordion>
    );

    expect(screen.getByText('Test Subtitle')).toBeInTheDocument();
  });

  it('does not render subtitle when not provided', () => {
    render(
      <Accordion title="Test Title" isExpanded={false} onToggle={vi.fn()}>
        <div>Content</div>
      </Accordion>
    );

    expect(screen.queryByText('Test Subtitle')).not.toBeInTheDocument();
  });

  it('shows content when expanded', () => {
    render(
      <Accordion title="Test Title" isExpanded={true} onToggle={vi.fn()}>
        <div>Expanded Content</div>
      </Accordion>
    );

    expect(screen.getByText('Expanded Content')).toBeInTheDocument();
  });

  it('hides content when collapsed', () => {
    render(
      <Accordion title="Test Title" isExpanded={false} onToggle={vi.fn()}>
        <div>Hidden Content</div>
      </Accordion>
    );

    expect(screen.queryByText('Hidden Content')).not.toBeInTheDocument();
  });

  it('calls onToggle when header is clicked', async () => {
    const user = userEvent.setup();
    const handleToggle = vi.fn();

    render(
      <Accordion title="Test Title" isExpanded={false} onToggle={handleToggle}>
        <div>Content</div>
      </Accordion>
    );

    await user.click(screen.getByRole('button'));

    expect(handleToggle).toHaveBeenCalledTimes(1);
  });

  it('applies custom className', () => {
    const { container } = render(
      <Accordion
        title="Test Title"
        isExpanded={false}
        onToggle={vi.fn()}
        className="custom-class"
      >
        <div>Content</div>
      </Accordion>
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('renders complex children when expanded', () => {
    render(
      <Accordion title="Test Title" isExpanded={true} onToggle={vi.fn()}>
        <div data-testid="child-1">Child 1</div>
        <div data-testid="child-2">Child 2</div>
      </Accordion>
    );

    expect(screen.getByTestId('child-1')).toBeInTheDocument();
    expect(screen.getByTestId('child-2')).toBeInTheDocument();
  });

  describe('Accessibility', () => {
    it('has proper ARIA attributes when collapsed', () => {
      render(
        <Accordion title="Test Title" isExpanded={false} onToggle={vi.fn()}>
          <div>Content</div>
        </Accordion>
      );

      const button = screen.getByRole('button', { name: /test title/i });
      expect(button).toHaveAttribute('aria-expanded', 'false');
      expect(button).toHaveAttribute('aria-controls');
    });

    it('has proper ARIA attributes when expanded', () => {
      render(
        <Accordion title="Test Title" isExpanded={true} onToggle={vi.fn()}>
          <div>Content</div>
        </Accordion>
      );

      const button = screen.getByRole('button', { name: /test title/i });
      expect(button).toHaveAttribute('aria-expanded', 'true');
      expect(button).toHaveAttribute('aria-controls');

      // Verify the content region exists and is properly labeled
      const region = screen.getByRole('region', { name: /test title/i });
      expect(region).toBeInTheDocument();
    });

    it('links button to content panel via aria-controls', () => {
      render(
        <Accordion title="Test Title" isExpanded={true} onToggle={vi.fn()}>
          <div>Expanded Content</div>
        </Accordion>
      );

      const button = screen.getByRole('button', { name: /test title/i });
      const controlsId = button.getAttribute('aria-controls');

      expect(controlsId).toBeTruthy();

      const region = screen.getByRole('region');
      expect(region).toHaveAttribute('id', controlsId);
    });

    it('hides decorative icons from screen readers', () => {
      const { container } = render(
        <Accordion title="Test Title" isExpanded={false} onToggle={vi.fn()}>
          <div>Content</div>
        </Accordion>
      );

      const icons = container.querySelectorAll('svg');
      icons.forEach(icon => {
        expect(icon).toHaveAttribute('aria-hidden', 'true');
      });
    });

    it('uses unique IDs for multiple accordions', () => {
      render(
        <>
          <Accordion title="Accordion 1" isExpanded={false} onToggle={vi.fn()}>
            <div>Content 1</div>
          </Accordion>
          <Accordion title="Accordion 2" isExpanded={true} onToggle={vi.fn()}>
            <div>Content 2</div>
          </Accordion>
        </>
      );

      const buttons = screen.getAllByRole('button');
      const button1Controls = buttons[0].getAttribute('aria-controls');
      const button2Controls = buttons[1].getAttribute('aria-controls');

      // IDs should be unique
      expect(button1Controls).not.toBe(button2Controls);

      // Each button should have unique ID
      const button1Id = buttons[0].getAttribute('id');
      const button2Id = buttons[1].getAttribute('id');
      expect(button1Id).not.toBe(button2Id);
    });
  });
});
