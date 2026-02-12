import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Select } from './form-select';

const mockOptions = [
  { value: 'option1', label: 'Option 1' },
  { value: 'option2', label: 'Option 2' },
  { value: 'option3', label: 'Option 3' },
];

describe('Select', () => {
  describe('basic variant', () => {
    it('renders native select element', () => {
      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="basic"
        />,
      );

      expect(screen.getByRole('combobox')).toBeInTheDocument();
    });

    it('renders options', () => {
      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="basic"
        />,
      );

      expect(screen.getByRole('option', { name: 'Option 1' })).toBeInTheDocument();
      expect(screen.getByRole('option', { name: 'Option 2' })).toBeInTheDocument();
    });

    it('calls onChange when option is selected', async () => {
      const user = userEvent.setup();
      const handleChange = vi.fn();

      render(
        <Select
          id="test-select"
          value=""
          onChange={handleChange}
          options={mockOptions}
          variant="basic"
        />,
      );

      await user.selectOptions(screen.getByRole('combobox'), 'option1');

      expect(handleChange).toHaveBeenCalledWith('option1');
    });

    it('renders label when provided', () => {
      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="basic"
          label="Select Option"
        />,
      );

      expect(screen.getByText('Select Option')).toBeInTheDocument();
    });

    it('disables select when disabled prop is true', () => {
      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="basic"
          disabled
        />,
      );

      expect(screen.getByRole('combobox')).toBeDisabled();
    });

    it('shows warning message when invalid', () => {
      render(
        <Select
          id="test-select"
          value="option1"
          onChange={vi.fn()}
          options={mockOptions}
          variant="basic"
          isValid={false}
          warningMessage="Please select a valid option"
        />,
      );

      expect(screen.getByText(/Please select a valid option/)).toBeInTheDocument();
    });
  });

  describe('dropdown variant', () => {
    it('renders dropdown trigger button', () => {
      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="dropdown"
        />,
      );

      expect(screen.getByRole('button')).toBeInTheDocument();
    });

    it('shows placeholder when no value selected', () => {
      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="dropdown"
          placeholder="Choose one"
        />,
      );

      expect(screen.getByText('Choose one')).toBeInTheDocument();
    });

    it('opens dropdown when clicked', async () => {
      const user = userEvent.setup();

      render(
        <Select
          id="test-select"
          value=""
          onChange={vi.fn()}
          options={mockOptions}
          variant="dropdown"
        />,
      );

      await user.click(screen.getByRole('button'));

      expect(screen.getByRole('listbox')).toBeInTheDocument();
    });

    it('calls onChange when option is clicked', async () => {
      const user = userEvent.setup();
      const handleChange = vi.fn();

      render(
        <Select
          id="test-select"
          value=""
          onChange={handleChange}
          options={mockOptions}
          variant="dropdown"
        />,
      );

      await user.click(screen.getByRole('button'));
      await user.click(screen.getByRole('option', { name: 'Option 1' }));

      expect(handleChange).toHaveBeenCalledWith('option1');
    });

    it('shows empty state when no options', async () => {
      const user = userEvent.setup();

      render(
        <Select id="test-select" value="" onChange={vi.fn()} options={[]} variant="dropdown" />,
      );

      await user.click(screen.getByRole('button'));

      expect(screen.getByText('No options available')).toBeInTheDocument();
    });
  });
});
