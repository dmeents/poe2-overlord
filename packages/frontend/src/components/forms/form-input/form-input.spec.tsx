import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Input } from './form-input';

describe('Input', () => {
  it('renders with default type text', () => {
    render(<Input id="test-input" value="" onChange={vi.fn()} />);

    expect(screen.getByRole('textbox')).toBeInTheDocument();
  });

  it('renders placeholder text', () => {
    render(<Input id="test-input" value="" onChange={vi.fn()} placeholder="Enter text" />);

    expect(screen.getByPlaceholderText('Enter text')).toBeInTheDocument();
  });

  it('calls onChange with string value when user types', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();

    render(<Input id="test-input" value="" onChange={handleChange} />);

    await user.type(screen.getByRole('textbox'), 'a');

    expect(handleChange).toHaveBeenCalledWith('a');
  });

  it('renders number type input', () => {
    render(<Input id="test-input" value="" onChange={vi.fn()} type="number" />);

    expect(screen.getByRole('spinbutton')).toBeInTheDocument();
  });

  it('shows error message when invalid', () => {
    render(
      <Input
        id="test-input"
        value="invalid"
        onChange={vi.fn()}
        isInvalid={true}
        errorMessage="This is invalid"
      />,
    );

    expect(screen.getByText('This is invalid')).toBeInTheDocument();
  });

  it('does not show error message when valid', () => {
    render(
      <Input
        id="test-input"
        value="valid"
        onChange={vi.fn()}
        isInvalid={false}
        errorMessage="This is invalid"
      />,
    );

    expect(screen.queryByText('This is invalid')).not.toBeInTheDocument();
  });

  it('passes min and max to number input', () => {
    render(<Input id="test-input" value={5} onChange={vi.fn()} type="number" min={0} max={10} />);

    const input = screen.getByRole('spinbutton');
    expect(input).toHaveAttribute('min', '0');
    expect(input).toHaveAttribute('max', '10');
  });
});
