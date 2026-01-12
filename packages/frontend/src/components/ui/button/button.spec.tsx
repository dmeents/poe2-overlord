import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Button } from './button';

describe('Button', () => {
  it('renders children correctly', () => {
    render(<Button>Click me</Button>);

    expect(
      screen.getByRole('button', { name: 'Click me' })
    ).toBeInTheDocument();
  });

  it('calls onClick when clicked', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(<Button onClick={handleClick}>Click me</Button>);

    await user.click(screen.getByRole('button'));

    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('does not call onClick when disabled', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(
      <Button onClick={handleClick} disabled>
        Click me
      </Button>
    );

    await user.click(screen.getByRole('button'));

    expect(handleClick).not.toHaveBeenCalled();
  });

  it('applies disabled attribute correctly', () => {
    render(<Button disabled>Click me</Button>);

    expect(screen.getByRole('button')).toBeDisabled();
  });

  it('defaults to type="button"', () => {
    render(<Button>Click me</Button>);

    expect(screen.getByRole('button')).toHaveAttribute('type', 'button');
  });

  it('accepts custom type prop', () => {
    render(<Button type="submit">Submit</Button>);

    expect(screen.getByRole('button')).toHaveAttribute('type', 'submit');
  });

  it('applies custom className', () => {
    render(<Button className="custom-class">Click me</Button>);

    expect(screen.getByRole('button')).toHaveClass('custom-class');
  });

  it('applies id and title attributes', () => {
    render(
      <Button id="test-id" title="test title">
        Click me
      </Button>
    );

    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('id', 'test-id');
    expect(button).toHaveAttribute('title', 'test title');
  });
});
