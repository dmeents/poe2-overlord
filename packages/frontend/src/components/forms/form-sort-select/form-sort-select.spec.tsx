import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { SortSelect } from './form-sort-select';

const mockOptions = [
  { value: 'name', label: 'Name' },
  { value: 'date', label: 'Date' },
  { value: 'level', label: 'Level' },
];

describe('SortSelect', () => {
  it('renders with selected option label', () => {
    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
      />
    );

    expect(screen.getByText('Name')).toBeInTheDocument();
  });

  it('renders placeholder when no value selected', () => {
    render(
      <SortSelect
        id='sort'
        value=''
        direction='asc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
      />
    );

    expect(screen.getByText('Sort by...')).toBeInTheDocument();
  });

  it('renders label when provided', () => {
    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
        label='Sort By'
      />
    );

    expect(screen.getByText('Sort By')).toBeInTheDocument();
  });

  it('opens dropdown when clicked', async () => {
    const user = userEvent.setup();

    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
      />
    );

    await user.click(screen.getByRole('button', { name: /name/i }));

    expect(screen.getByText('Sort Options')).toBeInTheDocument();
    expect(screen.getByRole('listbox')).toBeInTheDocument();
  });

  it('calls onChange when option is selected', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();

    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={handleChange}
        onReset={vi.fn()}
        options={mockOptions}
      />
    );

    await user.click(screen.getByRole('button', { name: /name/i }));
    await user.click(screen.getByRole('option', { name: 'Date' }));

    expect(handleChange).toHaveBeenCalledWith('date');
  });

  it('calls onReset when reset button is clicked', async () => {
    const user = userEvent.setup();
    const handleReset = vi.fn();

    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={vi.fn()}
        onReset={handleReset}
        options={mockOptions}
      />
    );

    await user.click(screen.getByRole('button', { name: /name/i }));
    await user.click(screen.getByRole('button', { name: 'Reset' }));

    expect(handleReset).toHaveBeenCalledTimes(1);
  });

  it('shows ascending direction indicator', () => {
    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
      />
    );

    expect(screen.getByText('↑')).toBeInTheDocument();
  });

  it('shows descending direction indicator', () => {
    render(
      <SortSelect
        id='sort'
        value='name'
        direction='desc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
      />
    );

    expect(screen.getByText('↓')).toBeInTheDocument();
  });

  it('disables button when disabled prop is true', () => {
    render(
      <SortSelect
        id='sort'
        value='name'
        direction='asc'
        onChange={vi.fn()}
        onReset={vi.fn()}
        options={mockOptions}
        disabled
      />
    );

    expect(screen.getByRole('button', { name: /name/i })).toBeDisabled();
  });
});
