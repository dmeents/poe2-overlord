import { describe, expect, it } from 'vitest';
import { calculateItemsSoldPerHour } from './economy-utils';

describe('calculateItemsSoldPerHour', () => {
  it('calculates normal division correctly', () => {
    expect(calculateItemsSoldPerHour(1000, 10)).toBe('100.00');
  });

  it('formats with 2 decimal places', () => {
    expect(calculateItemsSoldPerHour(1000, 7)).toBe('142.86');
  });

  it('returns "0.00" when primary value is zero', () => {
    expect(calculateItemsSoldPerHour(1000, 0)).toBe('0.00');
  });

  it('returns "0.00" when primary value is Infinity', () => {
    expect(calculateItemsSoldPerHour(1000, Infinity)).toBe('0.00');
  });

  it('returns "0.00" when primary value is -Infinity', () => {
    expect(calculateItemsSoldPerHour(1000, -Infinity)).toBe('0.00');
  });

  it('returns "0.00" when primary value is NaN', () => {
    expect(calculateItemsSoldPerHour(1000, NaN)).toBe('0.00');
  });

  it('returns "0.00" when result would be Infinity', () => {
    expect(calculateItemsSoldPerHour(Infinity, 10)).toBe('0.00');
  });

  it('returns "0.00" when result would be NaN', () => {
    expect(calculateItemsSoldPerHour(NaN, 10)).toBe('0.00');
  });

  it('handles negative values correctly', () => {
    expect(calculateItemsSoldPerHour(1000, -10)).toBe('-100.00');
  });

  it('handles very small primary values', () => {
    expect(calculateItemsSoldPerHour(100, 0.01)).toBe('10,000.00');
  });

  it('handles very large results', () => {
    expect(calculateItemsSoldPerHour(1000000, 0.1)).toBe('10,000,000.00');
  });

  it('uses locale formatting for large numbers', () => {
    const result = calculateItemsSoldPerHour(1000000, 10);
    expect(result).toBe('100,000.00');
  });
});
