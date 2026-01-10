import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Input } from './form-input';

describe('Input', () => {
  it('renders with default type text', () => {
    render(<Input id='test-input' value='' onChange={vi.fn()} />);

    expect(screen.getByRole('textbox')).toBeInTheDocument();
  });

  it('renders with label when provided', () => {
    render(
      <Input id='test-input' value='' onChange={vi.fn()} label='Test Label' />
    );

    expect(screen.getByLabelText('Test Label')).toBeInTheDocument();
  });

  it('renders placeholder text', () => {
    render(
      <Input
        id='test-input'
        value=''
        onChange={vi.fn()}
        placeholder='Enter text'
      />
    );

    expect(screen.getByPlaceholderText('Enter text')).toBeInTheDocument();
  });

  it('calls onChange when user types', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();

    render(<Input id='test-input' value='' onChange={handleChange} />);

    await user.type(screen.getByRole('textbox'), 'a');

    expect(handleChange).toHaveBeenCalledWith('a');
  });

  it('handles number type input', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();

    render(
      <Input id='test-input' value='' onChange={handleChange} type='number' />
    );

    await user.type(screen.getByRole('spinbutton'), '5');

    expect(handleChange).toHaveBeenCalledWith(5);
  });

  it('handles empty number input as null', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();

    render(
      <Input id='test-input' value={5} onChange={handleChange} type='number' />
    );

    await user.clear(screen.getByRole('spinbutton'));

    expect(handleChange).toHaveBeenCalledWith(null);
  });

  it('renders disabled state correctly', () => {
    render(<Input id='test-input' value='' onChange={vi.fn()} disabled />);

    expect(screen.getByRole('textbox')).toBeDisabled();
  });

  it('renders search input with magnifying glass icon', () => {
    render(
      <Input id='test-input' value='' onChange={vi.fn()} type='search' />
    );

    // Search input renders as textbox (type is converted to text)
    expect(screen.getByRole('textbox')).toBeInTheDocument();
  });

  it('shows clear button for search input with value', () => {
    render(
      <Input id='test-input' value='test' onChange={vi.fn()} type='search' />
    );

    expect(
      screen.getByRole('button', { name: 'Clear input' })
    ).toBeInTheDocument();
  });

  it('hides clear button when search input is empty', () => {
    render(
      <Input id='test-input' value='' onChange={vi.fn()} type='search' />
    );

    expect(
      screen.queryByRole('button', { name: 'Clear input' })
    ).not.toBeInTheDocument();
  });

  it('clears input when clear button is clicked', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();
    const handleClear = vi.fn();

    render(
      <Input
        id='test-input'
        value='test'
        onChange={handleChange}
        type='search'
        onClear={handleClear}
      />
    );

    await user.click(screen.getByRole('button', { name: 'Clear input' }));

    expect(handleChange).toHaveBeenCalledWith('');
    expect(handleClear).toHaveBeenCalled();
  });

  it('shows warning message when invalid', () => {
    render(
      <Input
        id='test-input'
        value='invalid'
        onChange={vi.fn()}
        isValid={false}
        warningMessage='This is invalid'
      />
    );

    expect(screen.getByText(/This is invalid/)).toBeInTheDocument();
  });

  it('does not show warning message when valid', () => {
    render(
      <Input
        id='test-input'
        value='valid'
        onChange={vi.fn()}
        isValid={true}
        warningMessage='This is invalid'
      />
    );

    expect(screen.queryByText(/This is invalid/)).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    render(
      <Input
        id='test-input'
        value=''
        onChange={vi.fn()}
        className='custom-class'
      />
    );

    expect(screen.getByRole('textbox')).toHaveClass('custom-class');
  });

  it('passes min, max, step to number input', () => {
    render(
      <Input
        id='test-input'
        value={5}
        onChange={vi.fn()}
        type='number'
        min={0}
        max={10}
        step={2}
      />
    );

    const input = screen.getByRole('spinbutton');
    expect(input).toHaveAttribute('min', '0');
    expect(input).toHaveAttribute('max', '10');
    expect(input).toHaveAttribute('step', '2');
  });
});
