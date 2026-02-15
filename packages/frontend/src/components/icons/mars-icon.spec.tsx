import { render } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { MarsIcon } from './mars-icon';

describe('MarsIcon', () => {
  describe('Static Rendering', () => {
    it('renders mars icon correctly', () => {
      const { container } = render(<MarsIcon />);

      const svg = container.querySelector('svg');

      // SVG element
      expect(svg).toBeInTheDocument();

      // aria-hidden attribute
      expect(svg).toHaveAttribute('aria-hidden', 'true');

      // Circle element
      expect(container.querySelector('circle')).toBeInTheDocument();

      // Line elements for arrow
      const lines = container.querySelectorAll('line');
      expect(lines.length).toBe(3);
    });
  });

  it('applies custom className', () => {
    const { container } = render(<MarsIcon className="custom-class" />);

    expect(container.querySelector('svg')).toHaveClass('custom-class');
  });
});
