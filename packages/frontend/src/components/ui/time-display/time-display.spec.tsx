import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { TimeDisplay } from './time-display';

describe('TimeDisplay', () => {
  it('renders seconds only when under a minute', () => {
    render(<TimeDisplay seconds={45} />);

    expect(screen.getByText('45s')).toBeInTheDocument();
  });

  it('renders minutes and seconds when under an hour', () => {
    render(<TimeDisplay seconds={125} />);

    expect(screen.getByText('2m 5s')).toBeInTheDocument();
  });

  it('renders hours, minutes and seconds', () => {
    render(<TimeDisplay seconds={3725} />);

    expect(screen.getByText('1h 2m 5s')).toBeInTheDocument();
  });

  it('hides seconds when showSeconds is false for hours', () => {
    render(<TimeDisplay seconds={3725} showSeconds={false} />);

    expect(screen.getByText('1h 2m')).toBeInTheDocument();
  });

  it('hides seconds when showSeconds is false for minutes', () => {
    render(<TimeDisplay seconds={125} showSeconds={false} />);

    expect(screen.getByText('2m')).toBeInTheDocument();
  });

  it('always shows seconds when under a minute regardless of showSeconds', () => {
    render(<TimeDisplay seconds={45} showSeconds={false} />);

    expect(screen.getByText('45s')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    render(<TimeDisplay seconds={60} className="custom-class" />);

    expect(screen.getByText('1m 0s')).toHaveClass('custom-class');
  });

  it('handles zero seconds', () => {
    render(<TimeDisplay seconds={0} />);

    expect(screen.getByText('0s')).toBeInTheDocument();
  });

  it('handles exact hours', () => {
    render(<TimeDisplay seconds={3600} />);

    expect(screen.getByText('1h 0m 0s')).toBeInTheDocument();
  });

  it('handles exact minutes', () => {
    render(<TimeDisplay seconds={60} />);

    expect(screen.getByText('1m 0s')).toBeInTheDocument();
  });
});
