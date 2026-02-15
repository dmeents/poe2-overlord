import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { LoadingSpinner } from './loading-spinner';

describe('LoadingSpinner', () => {
  describe('Static Rendering', () => {
    it('renders loading spinner correctly', () => {
      const { container } = render(<LoadingSpinner />);

      // Component renders without message
      expect(container.firstChild).toBeInTheDocument();

      // Does not render message when not provided
      expect(screen.queryByText('Loading')).not.toBeInTheDocument();
    });
  });

  it('renders with message when provided', () => {
    render(<LoadingSpinner message="Loading data..." />);

    expect(screen.getByText('Loading data...')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<LoadingSpinner className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });
});
