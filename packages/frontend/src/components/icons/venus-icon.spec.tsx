import { render } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { VenusIcon } from './venus-icon';

describe('VenusIcon', () => {
  describe('Static Rendering', () => {
    it('renders venus icon correctly', () => {
      const { container } = render(<VenusIcon />);

      const svg = container.querySelector('svg');

      // SVG element
      expect(svg).toBeInTheDocument();

      // aria-hidden attribute
      expect(svg).toHaveAttribute('aria-hidden', 'true');

      // Circle element
      expect(container.querySelector('circle')).toBeInTheDocument();

      // Line elements for cross
      const lines = container.querySelectorAll('line');
      expect(lines.length).toBe(2);
    });
  });

  it('applies custom className', () => {
    const { container } = render(<VenusIcon className="custom-class" />);

    expect(container.querySelector('svg')).toHaveClass('custom-class');
  });
});
