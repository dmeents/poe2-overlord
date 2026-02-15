import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { DataItem } from './data-item';

describe('DataItem', () => {
  describe('Static Rendering', () => {
    it('renders data item information correctly', () => {
      render(<DataItem label="Test Label" value="Test Value" />);

      // Label
      expect(screen.getByText('Test Label')).toBeInTheDocument();

      // String value
      expect(screen.getByText('Test Value')).toBeInTheDocument();
    });
  });

  it('renders subValue when provided', () => {
    render(<DataItem label="Label" value="Value" subValue="Sub Value" />);

    expect(screen.getByText('Sub Value')).toBeInTheDocument();
  });

  it('does not render subValue when not provided', () => {
    render(<DataItem label="Label" value="Value" />);

    expect(screen.queryByText('Sub Value')).not.toBeInTheDocument();
  });

  it('renders icon when provided', () => {
    render(
      <DataItem label="Label" value="Value" icon={<span data-testid="test-icon">Icon</span>} />,
    );

    expect(screen.getByTestId('test-icon')).toBeInTheDocument();
  });

  it('does not render icon when not provided', () => {
    render(<DataItem label="Label" value="Value" />);

    expect(screen.queryByTestId('test-icon')).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<DataItem label="Label" value="Value" className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('applies border color when color is provided', () => {
    const { container } = render(<DataItem label="Label" value="Value" color="#ff0000" />);

    expect(container.firstChild).toHaveStyle({ borderLeftColor: '#ff0000' });
  });

  it('renders ReactNode as label', () => {
    render(<DataItem label={<span data-testid="custom-label">Custom Label</span>} value="Value" />);

    expect(screen.getByTestId('custom-label')).toBeInTheDocument();
  });
});
