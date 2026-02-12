import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Card } from './card';

describe('Card', () => {
  it('renders children correctly', () => {
    render(
      <Card>
        <div>Card Content</div>
      </Card>,
    );

    expect(screen.getByText('Card Content')).toBeInTheDocument();
  });

  it('renders title when provided', () => {
    render(
      <Card title="Test Title">
        <div>Content</div>
      </Card>,
    );

    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });

  it('renders subtitle when provided', () => {
    render(
      <Card title="Test Title" subtitle="Test Subtitle">
        <div>Content</div>
      </Card>,
    );

    expect(screen.getByText('Test Subtitle')).toBeInTheDocument();
  });

  it('renders icon when provided', () => {
    render(
      <Card title="Title" icon={<span data-testid="test-icon">Icon</span>}>
        <div>Content</div>
      </Card>,
    );

    expect(screen.getByTestId('test-icon')).toBeInTheDocument();
  });

  it('renders right action button when provided', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(
      <Card title="Title" rightAction={{ label: 'Action', onClick: handleClick }}>
        <div>Content</div>
      </Card>,
    );

    const actionButton = screen.getByRole('button', { name: 'Action' });
    expect(actionButton).toBeInTheDocument();

    await user.click(actionButton);
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('hides subtitle when rightAction is provided', () => {
    render(
      <Card title="Title" subtitle="Subtitle" rightAction={{ label: 'Action', onClick: vi.fn() }}>
        <div>Content</div>
      </Card>,
    );

    expect(screen.queryByText('Subtitle')).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <Card className="custom-class">
        <div>Content</div>
      </Card>,
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('does not render header when no title is provided', () => {
    render(
      <Card>
        <div>Content Only</div>
      </Card>,
    );

    // Header should not be present when there's no title
    expect(screen.queryByText('Title')).not.toBeInTheDocument();
  });

  it('renders status indicator when showStatusIndicator is true', () => {
    const { container } = render(
      <Card title="Title" showStatusIndicator>
        <div>Content</div>
      </Card>,
    );

    // Check for the animate-pulse class which is on the status indicator
    expect(container.querySelector('.animate-pulse')).toBeInTheDocument();
  });
});
