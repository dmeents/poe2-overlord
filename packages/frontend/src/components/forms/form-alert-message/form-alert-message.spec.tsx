import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { AlertMessage } from './form-alert-message';

describe('AlertMessage', () => {
  it('renders error message correctly', () => {
    render(<AlertMessage type='error' message='Something went wrong' />);

    expect(screen.getByText('Something went wrong')).toBeInTheDocument();
  });

  it('renders success message correctly', () => {
    render(<AlertMessage type='success' message='Operation completed' />);

    expect(screen.getByText('Operation completed')).toBeInTheDocument();
  });

  it('returns null when message is empty', () => {
    const { container } = render(<AlertMessage type='error' message='' />);

    expect(container.firstChild).toBeNull();
  });

  it('applies custom className', () => {
    render(
      <AlertMessage
        type='error'
        message='Test message'
        className='custom-class'
      />
    );

    expect(screen.getByText('Test message')).toHaveClass('custom-class');
  });
});
