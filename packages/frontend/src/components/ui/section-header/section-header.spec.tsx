import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { SectionHeader } from './section-header';

describe('SectionHeader', () => {
  it('renders title correctly', () => {
    render(<SectionHeader title="Test Section" />);

    expect(screen.getByText('Test Section')).toBeInTheDocument();
  });

  it('renders icon when provided', () => {
    render(<SectionHeader title="Test Section" icon={<span data-testid="test-icon">Icon</span>} />);

    expect(screen.getByTestId('test-icon')).toBeInTheDocument();
  });

  it('does not render icon when not provided', () => {
    render(<SectionHeader title="Test Section" />);

    expect(screen.queryByTestId('test-icon')).not.toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(<SectionHeader title="Test Section" className="custom-class" />);

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('renders title as heading', () => {
    render(<SectionHeader title="Test Section" />);

    expect(screen.getByRole('heading', { level: 3 })).toBeInTheDocument();
  });
});
