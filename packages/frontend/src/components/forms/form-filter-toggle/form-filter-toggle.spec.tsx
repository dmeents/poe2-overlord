import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { FilterToggle } from './form-filter-toggle';

describe('FilterToggle', () => {
  it('renders label correctly', () => {
    render(<FilterToggle isExpanded={false} onToggle={vi.fn()} label="Filters" />);

    expect(screen.getByText('Filters')).toBeInTheDocument();
  });

  it('shows active count when provided', () => {
    render(<FilterToggle isExpanded={false} onToggle={vi.fn()} label="Filters" activeCount={3} />);

    expect(screen.getByText('Filters (3)')).toBeInTheDocument();
  });

  it('does not show count when zero', () => {
    render(<FilterToggle isExpanded={false} onToggle={vi.fn()} label="Filters" activeCount={0} />);

    expect(screen.getByText('Filters')).toBeInTheDocument();
    expect(screen.queryByText('Filters (0)')).not.toBeInTheDocument();
  });

  it('calls onToggle when clicked', async () => {
    const user = userEvent.setup();
    const handleToggle = vi.fn();

    render(<FilterToggle isExpanded={false} onToggle={handleToggle} label="Filters" />);

    await user.click(screen.getByRole('button'));

    expect(handleToggle).toHaveBeenCalledTimes(1);
  });

  it('shows children when expanded', () => {
    render(
      <FilterToggle isExpanded={true} onToggle={vi.fn()} label="Filters">
        <div>Filter Content</div>
      </FilterToggle>,
    );

    expect(screen.getByText('Filter Content')).toBeInTheDocument();
  });

  it('hides children when collapsed', () => {
    render(
      <FilterToggle isExpanded={false} onToggle={vi.fn()} label="Filters">
        <div>Filter Content</div>
      </FilterToggle>,
    );

    expect(screen.queryByText('Filter Content')).not.toBeInTheDocument();
  });

  it('disables button when disabled prop is true', () => {
    render(<FilterToggle isExpanded={false} onToggle={vi.fn()} label="Filters" disabled />);

    expect(screen.getByRole('button')).toBeDisabled();
  });

  it('applies custom className', () => {
    const { container } = render(
      <FilterToggle
        isExpanded={false}
        onToggle={vi.fn()}
        label="Filters"
        className="custom-class"
      />,
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });
});
