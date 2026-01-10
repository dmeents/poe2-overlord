import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Accordion } from './accordion';

describe('Accordion', () => {
  it('renders title correctly', () => {
    render(
      <Accordion title='Test Title' isExpanded={false} onToggle={vi.fn()}>
        <div>Content</div>
      </Accordion>
    );

    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });

  it('renders subtitle when provided', () => {
    render(
      <Accordion
        title='Test Title'
        subtitle='Test Subtitle'
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
      <Accordion title='Test Title' isExpanded={false} onToggle={vi.fn()}>
        <div>Content</div>
      </Accordion>
    );

    expect(screen.queryByText('Test Subtitle')).not.toBeInTheDocument();
  });

  it('shows content when expanded', () => {
    render(
      <Accordion title='Test Title' isExpanded={true} onToggle={vi.fn()}>
        <div>Expanded Content</div>
      </Accordion>
    );

    expect(screen.getByText('Expanded Content')).toBeInTheDocument();
  });

  it('hides content when collapsed', () => {
    render(
      <Accordion title='Test Title' isExpanded={false} onToggle={vi.fn()}>
        <div>Hidden Content</div>
      </Accordion>
    );

    expect(screen.queryByText('Hidden Content')).not.toBeInTheDocument();
  });

  it('calls onToggle when header is clicked', async () => {
    const user = userEvent.setup();
    const handleToggle = vi.fn();

    render(
      <Accordion title='Test Title' isExpanded={false} onToggle={handleToggle}>
        <div>Content</div>
      </Accordion>
    );

    await user.click(screen.getByRole('button'));

    expect(handleToggle).toHaveBeenCalledTimes(1);
  });

  it('applies custom className', () => {
    const { container } = render(
      <Accordion
        title='Test Title'
        isExpanded={false}
        onToggle={vi.fn()}
        className='custom-class'
      >
        <div>Content</div>
      </Accordion>
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('renders complex children when expanded', () => {
    render(
      <Accordion title='Test Title' isExpanded={true} onToggle={vi.fn()}>
        <div data-testid='child-1'>Child 1</div>
        <div data-testid='child-2'>Child 2</div>
      </Accordion>
    );

    expect(screen.getByTestId('child-1')).toBeInTheDocument();
    expect(screen.getByTestId('child-2')).toBeInTheDocument();
  });
});
