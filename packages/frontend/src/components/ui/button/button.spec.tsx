import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Button } from './button';

describe('Button', () => {
  describe('Static Rendering', () => {
    it('renders button information correctly', () => {
      render(
        <Button id="test-id" title="test title">
          Click me
        </Button>,
      );

      const button = screen.getByRole('button', { name: 'Click me' });

      // Children
      expect(button).toBeInTheDocument();

      // Defaults to type="button"
      expect(button).toHaveAttribute('type', 'button');

      // id and title attributes
      expect(button).toHaveAttribute('id', 'test-id');
      expect(button).toHaveAttribute('title', 'test title');
    });
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

  it('accepts custom type prop', () => {
    render(<Button type="submit">Submit</Button>);

    expect(screen.getByRole('button')).toHaveAttribute('type', 'submit');
  });

  it('applies custom className', () => {
    render(<Button className="custom-class">Click me</Button>);

    expect(screen.getByRole('button')).toHaveClass('custom-class');
  });

  describe('loading state', () => {
    it('renders loading state correctly', () => {
      render(<Button loading>Click me</Button>);

      const button = screen.getByRole('button');

      // Loading spinner visible (svg with animate-spin class)
      const spinner = button.querySelector('svg.animate-spin');
      expect(spinner).toBeInTheDocument();

      // Disabled when loading
      expect(button).toBeDisabled();

      // aria-busy attribute set
      expect(button).toHaveAttribute('aria-busy', 'true');

      // cursor-wait class applied
      expect(button).toHaveClass('cursor-wait');
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

    it('hides loading spinner when loading is false', () => {
      render(<Button loading={false}>Click me</Button>);

      const button = screen.getByRole('button');
      const spinner = button.querySelector('svg.animate-spin');
      expect(spinner).not.toBeInTheDocument();
    });

    it('does not set aria-busy when not loading', () => {
      render(<Button>Click me</Button>);

      expect(screen.getByRole('button')).toHaveAttribute('aria-busy', 'false');
    });
  });
});
