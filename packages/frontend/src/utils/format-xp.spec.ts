import { describe, expect, it } from 'vitest';
import { formatXpAmount, formatXpRate } from './format-xp';

describe('formatXpAmount', () => {
  it('formats millions with one decimal place', () => {
    expect(formatXpAmount(1_200_000)).toBe('1.2M');
  });

  it('formats whole millions without decimal', () => {
    expect(formatXpAmount(2_000_000)).toBe('2M');
  });

  it('formats thousands as K', () => {
    expect(formatXpAmount(524_000)).toBe('524K');
  });

  it('formats fractional thousands without decimal', () => {
    expect(formatXpAmount(1_500)).toBe('2K'); // rounds to nearest K
  });

  it('formats sub-thousand values as integers', () => {
    expect(formatXpAmount(900)).toBe('900');
    expect(formatXpAmount(0)).toBe('0');
    expect(formatXpAmount(999)).toBe('999');
  });

  it('formats exactly 1000 as 1K', () => {
    expect(formatXpAmount(1_000)).toBe('1K');
  });

  it('formats exactly 1_000_000 as 1M', () => {
    expect(formatXpAmount(1_000_000)).toBe('1M');
  });
});

describe('formatXpRate', () => {
  it('appends XP/hr suffix', () => {
    expect(formatXpRate(1_200_000)).toBe('1.2M XP/hr');
  });

  it('formats thousands correctly with suffix', () => {
    expect(formatXpRate(524_000)).toBe('524K XP/hr');
  });

  it('formats small values correctly with suffix', () => {
    expect(formatXpRate(500)).toBe('500 XP/hr');
  });
});
