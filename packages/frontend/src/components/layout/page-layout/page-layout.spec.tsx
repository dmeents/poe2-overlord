import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { PageLayout } from './page-layout';

describe('PageLayout', () => {
  it('renders left column content', () => {
    render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
      />
    );

    expect(screen.getByText('Left Content')).toBeInTheDocument();
  });

  it('renders right column content', () => {
    render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
      />
    );

    expect(screen.getByText('Right Content')).toBeInTheDocument();
  });

  it('renders children when provided', () => {
    render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
      >
        <div>Child Content</div>
      </PageLayout>
    );

    expect(screen.getByText('Child Content')).toBeInTheDocument();
  });

  it('does not require children', () => {
    const { container } = render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
      />
    );

    expect(container.firstChild).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
        className="custom-class"
      />
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('applies base styling classes', () => {
    const { container } = render(
      <PageLayout
        leftColumn={<div>Left Content</div>}
        rightColumn={<div>Right Content</div>}
      />
    );

    expect(container.firstChild).toHaveClass('min-h-screen');
    expect(container.firstChild).toHaveClass('bg-zinc-900');
    expect(container.firstChild).toHaveClass('text-white');
  });
});
