import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { StatusIndicator } from './status-indicator';

describe('StatusIndicator', () => {
  it('renders icon correctly', () => {
    render(
      <StatusIndicator
        status='success'
        icon={<span data-testid='test-icon'>Icon</span>}
      />
    );

    expect(screen.getByTestId('test-icon')).toBeInTheDocument();
  });

  it('renders success status', () => {
    const { container } = render(
      <StatusIndicator status='success' icon={<span>Icon</span>} />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders warning status', () => {
    const { container } = render(
      <StatusIndicator status='warning' icon={<span>Icon</span>} />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders error status', () => {
    const { container } = render(
      <StatusIndicator status='error' icon={<span>Icon</span>} />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders info status', () => {
    const { container } = render(
      <StatusIndicator status='info' icon={<span>Icon</span>} />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <StatusIndicator
        status='success'
        icon={<span>Icon</span>}
        className='custom-class'
      />
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('renders with small size', () => {
    const { container } = render(
      <StatusIndicator status='success' icon={<span>Icon</span>} size='sm' />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders with medium size by default', () => {
    const { container } = render(
      <StatusIndicator status='success' icon={<span>Icon</span>} />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders with large size', () => {
    const { container } = render(
      <StatusIndicator status='success' icon={<span>Icon</span>} size='lg' />
    );

    expect(container.firstChild).toBeInTheDocument();
  });
});
