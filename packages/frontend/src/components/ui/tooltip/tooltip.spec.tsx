import { render, screen, fireEvent } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Tooltip } from './tooltip';

describe('Tooltip', () => {
  it('renders children correctly', () => {
    render(
      <Tooltip content='Tooltip text'>
        <span>Hover me</span>
      </Tooltip>
    );

    expect(screen.getByText('Hover me')).toBeInTheDocument();
  });

  it('shows tooltip content on hover', () => {
    render(
      <Tooltip content='Tooltip text'>
        <span>Hover me</span>
      </Tooltip>
    );

    fireEvent.mouseEnter(screen.getByText('Hover me'));

    expect(screen.getByText('Tooltip text')).toBeInTheDocument();
  });

  it('hides tooltip content on mouse leave', () => {
    render(
      <Tooltip content='Tooltip text'>
        <span>Hover me</span>
      </Tooltip>
    );

    const trigger = screen.getByText('Hover me');
    fireEvent.mouseEnter(trigger);
    fireEvent.mouseLeave(trigger);

    expect(screen.queryByText('Tooltip text')).not.toBeInTheDocument();
  });

  it('renders ReactNode content', () => {
    render(
      <Tooltip content={<span data-testid='custom-content'>Custom</span>}>
        <span>Hover me</span>
      </Tooltip>
    );

    fireEvent.mouseEnter(screen.getByText('Hover me'));

    expect(screen.getByTestId('custom-content')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const { container } = render(
      <Tooltip content='Tooltip text' className='custom-class'>
        <span>Hover me</span>
      </Tooltip>
    );

    expect(container.firstChild).toHaveClass('custom-class');
  });

  it('shows info icon when showIcon is true', () => {
    const { container } = render(
      <Tooltip content='Tooltip text' showIcon>
        <span>Hover me</span>
      </Tooltip>
    );

    // Check for SVG icon presence
    expect(container.querySelector('svg')).toBeInTheDocument();
  });

  it('does not show icon by default', () => {
    const { container } = render(
      <Tooltip content='Tooltip text'>
        <span>Hover me</span>
      </Tooltip>
    );

    // No SVG icon by default
    expect(container.querySelector('svg')).not.toBeInTheDocument();
  });
});
