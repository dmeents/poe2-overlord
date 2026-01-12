import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { EmptyState } from './empty-state';

describe('EmptyState', () => {
  it('renders icon correctly', () => {
    render(
      <EmptyState
        icon={<span data-testid="test-icon">Icon</span>}
        title="Test Title"
        description="Test description"
      />
    );

    expect(screen.getByTestId('test-icon')).toBeInTheDocument();
  });

  it('renders title correctly', () => {
    render(
      <EmptyState
        icon={<span>Icon</span>}
        title="No Items Found"
        description="There are no items to display"
      />
    );

    expect(screen.getByText('No Items Found')).toBeInTheDocument();
  });

  it('renders description correctly', () => {
    render(
      <EmptyState
        icon={<span>Icon</span>}
        title="No Items Found"
        description="There are no items to display"
      />
    );

    expect(
      screen.getByText('There are no items to display')
    ).toBeInTheDocument();
  });

  it('renders action when provided', () => {
    render(
      <EmptyState
        icon={<span>Icon</span>}
        title="No Items Found"
        description="There are no items to display"
        action={<button>Add Item</button>}
      />
    );

    expect(
      screen.getByRole('button', { name: 'Add Item' })
    ).toBeInTheDocument();
  });

  it('does not render action when not provided', () => {
    render(
      <EmptyState
        icon={<span>Icon</span>}
        title="No Items Found"
        description="There are no items to display"
      />
    );

    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <EmptyState
        icon={<span>Icon</span>}
        title="No Items Found"
        description="There are no items to display"
        className="custom-class"
      />
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });
});
