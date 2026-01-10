import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { LoadingSpinner } from './loading-spinner';

describe('LoadingSpinner', () => {
  it('renders without message', () => {
    const { container } = render(<LoadingSpinner />);

    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders with message when provided', () => {
    render(<LoadingSpinner message='Loading data...' />);

    expect(screen.getByText('Loading data...')).toBeInTheDocument();
  });

  it('does not render message when not provided', () => {
    render(<LoadingSpinner />);

    expect(screen.queryByText('Loading')).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<LoadingSpinner className='custom-class' />);

    expect(container.firstChild).toHaveClass('custom-class');
  });
});
