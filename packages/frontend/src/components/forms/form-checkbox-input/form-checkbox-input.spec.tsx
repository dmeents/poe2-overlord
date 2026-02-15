import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { CheckboxInput } from './form-checkbox-input';

describe('CheckboxInput', () => {
  describe('Static Rendering', () => {
    it('renders checkbox input correctly', () => {
      render(
        <CheckboxInput id="test-checkbox" checked={true} onChange={vi.fn()} label="Test Label" />,
      );

      // Label
      expect(screen.getByLabelText('Test Label')).toBeInTheDocument();

      // Checked state
      expect(screen.getByRole('checkbox')).toBeChecked();
    });
  });

  it('renders unchecked state correctly', () => {
    render(
      <CheckboxInput id="test-checkbox" checked={false} onChange={vi.fn()} label="Test Label" />,
    );

    expect(screen.getByRole('checkbox')).not.toBeChecked();
  });

  it('calls onChange when clicked', async () => {
    const user = userEvent.setup();
    const handleChange = vi.fn();

    render(
      <CheckboxInput
        id="test-checkbox"
        checked={false}
        onChange={handleChange}
        label="Test Label"
      />,
    );

    await user.click(screen.getByRole('checkbox'));

    expect(handleChange).toHaveBeenCalledWith(true);
  });

  it('renders description when provided', () => {
    render(
      <CheckboxInput
        id="test-checkbox"
        checked={false}
        onChange={vi.fn()}
        label="Test Label"
        description="This is a description"
      />,
    );

    expect(screen.getByText('This is a description')).toBeInTheDocument();
  });

  it('does not render description when not provided', () => {
    render(
      <CheckboxInput id="test-checkbox" checked={false} onChange={vi.fn()} label="Test Label" />,
    );

    expect(screen.queryByText('This is a description')).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <CheckboxInput
        id="test-checkbox"
        checked={false}
        onChange={vi.fn()}
        label="Test Label"
        className="custom-class"
      />,
    );

    expect(container.querySelector('.custom-class')).toBeInTheDocument();
  });

  it('renders React node as label', () => {
    render(
      <CheckboxInput
        id="test-checkbox"
        checked={false}
        onChange={vi.fn()}
        label={<span data-testid="custom-label">Custom Label</span>}
      />,
    );

    expect(screen.getByTestId('custom-label')).toBeInTheDocument();
  });
});
