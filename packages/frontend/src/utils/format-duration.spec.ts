import { describe, expect, it } from 'vitest';
import { formatDuration, formatDurationMinutes } from './format-duration';

describe('formatDuration', () => {
  it('returns "0s" for zero seconds', () => {
    expect(formatDuration(0)).toBe('0s');
  });

  it('formats seconds only', () => {
    expect(formatDuration(45)).toBe('45s');
  });

  it('formats minutes only', () => {
    expect(formatDuration(120)).toBe('2m');
  });

  it('formats hours only', () => {
    expect(formatDuration(7200)).toBe('2h');
  });

  it('formats minutes and seconds', () => {
    expect(formatDuration(135)).toBe('2m 15s');
  });

  it('formats hours and minutes', () => {
    expect(formatDuration(7320)).toBe('2h 2m');
  });

  it('formats hours and seconds', () => {
    expect(formatDuration(7215)).toBe('2h 15s');
  });

  it('formats hours, minutes, and seconds', () => {
    expect(formatDuration(7335)).toBe('2h 2m 15s');
  });

  it('handles large values', () => {
    expect(formatDuration(86400)).toBe('24h');
  });
});

describe('formatDurationMinutes', () => {
  it('returns "0m" for zero seconds', () => {
    expect(formatDurationMinutes(0)).toBe('0m');
  });

  it('formats minutes only', () => {
    expect(formatDurationMinutes(120)).toBe('2m');
  });

  it('rounds to nearest minute', () => {
    expect(formatDurationMinutes(90)).toBe('2m');
    expect(formatDurationMinutes(29)).toBe('0m');
  });

  it('formats hours only when minutes are 0', () => {
    expect(formatDurationMinutes(7200)).toBe('2h');
  });

  it('formats hours and minutes', () => {
    expect(formatDurationMinutes(7380)).toBe('2h 3m');
  });

  it('handles large values', () => {
    expect(formatDurationMinutes(86400)).toBe('24h');
  });
});
