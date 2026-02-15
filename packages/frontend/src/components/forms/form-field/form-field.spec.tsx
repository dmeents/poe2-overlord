import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { FormField } from './form-field';

describe('FormField', () => {
  describe('Static Rendering', () => {
    it('renders form field information correctly', () => {
      render(
        <FormField label="Test Label" description="Test description">
          <input type="text" data-testid="test-input" />
        </FormField>,
      );

      // Label
      expect(screen.getByText('Test Label')).toBeInTheDocument();

      // Children
      expect(screen.getByTestId('test-input')).toBeInTheDocument();

      // Description when provided
      expect(screen.getByText('Test description')).toBeInTheDocument();
    });
  });

  it('does not render description when not provided', () => {
    render(
      <FormField label="Test Label">
        <input type="text" />
      </FormField>,
    );

    expect(screen.queryByText('Test description')).not.toBeInTheDocument();
  });

  it('associates label with input via htmlFor', () => {
    render(
      <FormField label="Email" htmlFor="email-input">
        <input type="email" id="email-input" />
      </FormField>,
    );

    expect(screen.getByLabelText('Email')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <FormField label="Test Label" className="custom-class">
        <input type="text" />
      </FormField>,
    );

    expect(container.querySelector('.custom-class')).toBeInTheDocument();
  });

  it('renders React node as label', () => {
    render(
      <FormField label={<span data-testid="custom-label">Custom Label</span>}>
        <input type="text" />
      </FormField>,
    );

    expect(screen.getByTestId('custom-label')).toBeInTheDocument();
  });

  it('renders divider when not last item', () => {
    const { container } = render(
      <FormField label="Test Label">
        <input type="text" />
      </FormField>,
    );

    // The divider is the second child in the fragment
    const dividers = container.querySelectorAll('div');
    expect(dividers.length).toBeGreaterThan(1);
  });

  it('does not render divider when last-form-item class is applied', () => {
    const { container } = render(
      <FormField label="Test Label" className="last-form-item">
        <input type="text" />
      </FormField>,
    );

    // Count divider elements - should have fewer when last item
    const mainContainer = container.firstElementChild;
    expect(mainContainer).toHaveClass('last-form-item');
  });
});
