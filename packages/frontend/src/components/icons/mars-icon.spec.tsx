import { render } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { MarsIcon } from './mars-icon';

describe('MarsIcon', () => {
  it('renders SVG element', () => {
    const { container } = render(<MarsIcon />);

    expect(container.querySelector('svg')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<MarsIcon className="custom-class" />);

    expect(container.querySelector('svg')).toHaveClass('custom-class');
  });

  it('has aria-hidden attribute', () => {
    const { container } = render(<MarsIcon />);

    expect(container.querySelector('svg')).toHaveAttribute(
      'aria-hidden',
      'true'
    );
  });

  it('renders circle element', () => {
    const { container } = render(<MarsIcon />);

    expect(container.querySelector('circle')).toBeInTheDocument();
  });

  it('renders line elements for arrow', () => {
    const { container } = render(<MarsIcon />);

    const lines = container.querySelectorAll('line');
    expect(lines.length).toBe(3);
  });
});
