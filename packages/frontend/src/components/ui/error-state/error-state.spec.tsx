import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { ErrorState } from './error-state';

describe('ErrorState', () => {
  it('renders default title when not provided', () => {
    render(<ErrorState />);

    expect(screen.getByText('Error Loading Data')).toBeInTheDocument();
  });

  it('renders custom title when provided', () => {
    render(<ErrorState title="Custom Error Title" />);

    expect(screen.getByText('Custom Error Title')).toBeInTheDocument();
  });

  it('renders message when provided', () => {
    render(<ErrorState message="Something went wrong" />);

    expect(screen.getByText('Something went wrong')).toBeInTheDocument();
  });

  it('renders Error object message', () => {
    render(<ErrorState error={new Error('Network error')} />);

    expect(screen.getByText('Network error')).toBeInTheDocument();
  });

  it('renders string error', () => {
    render(<ErrorState error="Connection failed" />);

    expect(screen.getByText('Connection failed')).toBeInTheDocument();
  });

  it('renders default error message for unknown error', () => {
    render(<ErrorState error={null} />);

    expect(screen.getByText('An unknown error occurred')).toBeInTheDocument();
  });

  it('renders message from object with message property', () => {
    render(<ErrorState error={{ message: 'API error message' }} />);

    expect(screen.getByText('API error message')).toBeInTheDocument();
  });

  it('renders default error for object without message property', () => {
    render(<ErrorState error={{ code: 500 }} />);

    expect(screen.getByText('An unknown error occurred')).toBeInTheDocument();
  });

  it('renders default error for object with non-string message', () => {
    render(<ErrorState error={{ message: 123 }} />);

    expect(screen.getByText('An unknown error occurred')).toBeInTheDocument();
  });

  it('renders default error for undefined', () => {
    render(<ErrorState error={undefined} />);

    expect(screen.getByText('An unknown error occurred')).toBeInTheDocument();
  });

  it('prefers message over error prop', () => {
    render(<ErrorState message="Custom message" error={new Error('Error message')} />);

    expect(screen.getByText('Custom message')).toBeInTheDocument();
  });

  it('renders action when provided', () => {
    render(<ErrorState action={<button type="button">Retry</button>} />);

    expect(screen.getByRole('button', { name: 'Retry' })).toBeInTheDocument();
  });

  it('does not render action when not provided', () => {
    render(<ErrorState />);

    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });

  it('renders custom icon when provided', () => {
    render(<ErrorState icon={<span data-testid="custom-icon">X</span>} />);

    expect(screen.getByTestId('custom-icon')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<ErrorState className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });
});
