import { render } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { VenusIcon } from './venus-icon';

describe('VenusIcon', () => {
  it('renders SVG element', () => {
    const { container } = render(<VenusIcon />);

    expect(container.querySelector('svg')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<VenusIcon className='custom-class' />);

    expect(container.querySelector('svg')).toHaveClass('custom-class');
  });

  it('has aria-hidden attribute', () => {
    const { container } = render(<VenusIcon />);

    expect(container.querySelector('svg')).toHaveAttribute(
      'aria-hidden',
      'true'
    );
  });

  it('renders circle element', () => {
    const { container } = render(<VenusIcon />);

    expect(container.querySelector('circle')).toBeInTheDocument();
  });

  it('renders line elements for cross', () => {
    const { container } = render(<VenusIcon />);

    const lines = container.querySelectorAll('line');
    expect(lines.length).toBe(2);
  });
});
