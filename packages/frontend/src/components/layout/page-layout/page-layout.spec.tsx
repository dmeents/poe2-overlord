import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { PageLayout } from './page-layout';

describe('PageLayout', () => {
  describe('Static Rendering', () => {
    it('renders page layout correctly', () => {
      const { container } = render(
        <PageLayout leftColumn={<div>Left Content</div>} rightColumn={<div>Right Content</div>} />,
      );

      // Left column content
      expect(screen.getByText('Left Content')).toBeInTheDocument();

      // Right column content
      expect(screen.getByText('Right Content')).toBeInTheDocument();

      // Base styling classes
      expect(container.firstChild).toHaveClass('min-h-screen');
      expect(container.firstChild).toHaveClass('text-stone-50');
    });
  });

  it('renders children when provided', () => {
    render(
      <PageLayout leftColumn={<div>Left Content</div>} rightColumn={<div>Right Content</div>}>
        <div>Child Content</div>
      </PageLayout>,
    );

    expect(screen.getByText('Child Content')).toBeInTheDocument();
  });

  it('does not require children', () => {
    const { container } = render(
      <PageLayout leftColumn={<div>Left Content</div>} rightColumn={<div>Right Content</div>} />,
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
        className="custom-class"
      />,
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });
});
