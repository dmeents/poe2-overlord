import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Button } from './button';

describe('Button', () => {
  it('renders children correctly', () => {
    render(<Button>Click me</Button>);

    expect(screen.getByRole('button', { name: 'Click me' })).toBeInTheDocument();
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
      </Button>,
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
      </Button>,
    );

    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('id', 'test-id');
    expect(button).toHaveAttribute('title', 'test title');
  });

  describe('loading state', () => {
    it('shows loading spinner when loading is true', () => {
      render(<Button loading>Click me</Button>);

      // The spinner should be visible (svg with animate-spin class)
      const button = screen.getByRole('button');
      const spinner = button.querySelector('svg.animate-spin');
      expect(spinner).toBeInTheDocument();
    });

    it('hides loading spinner when loading is false', () => {
      render(<Button loading={false}>Click me</Button>);

      const button = screen.getByRole('button');
      const spinner = button.querySelector('svg.animate-spin');
      expect(spinner).not.toBeInTheDocument();
    });

    it('is disabled when loading', () => {
      render(<Button loading>Click me</Button>);

      expect(screen.getByRole('button')).toBeDisabled();
    });

    it('does not call onClick when loading', async () => {
      const user = userEvent.setup();
      const handleClick = vi.fn();

      render(
        <Button onClick={handleClick} loading>
          Click me
        </Button>,
      );

      await user.click(screen.getByRole('button'));

      expect(handleClick).not.toHaveBeenCalled();
    });

    it('sets aria-busy when loading', () => {
      render(<Button loading>Click me</Button>);

      expect(screen.getByRole('button')).toHaveAttribute('aria-busy', 'true');
    });

    it('does not set aria-busy when not loading', () => {
      render(<Button>Click me</Button>);

      expect(screen.getByRole('button')).toHaveAttribute('aria-busy', 'false');
    });

    it('applies cursor-wait class when loading', () => {
      render(<Button loading>Click me</Button>);

      expect(screen.getByRole('button')).toHaveClass('cursor-wait');
    });
  });
});
