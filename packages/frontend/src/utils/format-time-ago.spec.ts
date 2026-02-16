import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { formatTimeAgo } from './format-time-ago';

describe('formatTimeAgo', () => {
  beforeEach(() => {
    // Mock Date.now() to return a fixed time
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2024-01-10T12:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('returns "just now" for dates in the future', () => {
    const futureDate = new Date('2024-01-10T13:00:00Z');
    expect(formatTimeAgo(futureDate)).toBe('just now');
  });

  it('returns "just now" for dates less than 60 seconds ago', () => {
    const date = new Date('2024-01-10T11:59:30Z');
    expect(formatTimeAgo(date)).toBe('just now');
  });

  it('formats minutes correctly (singular)', () => {
    const date = new Date('2024-01-10T11:59:00Z');
    expect(formatTimeAgo(date)).toBe('1 minute ago');
  });

  it('formats minutes correctly (plural)', () => {
    const date = new Date('2024-01-10T11:55:00Z');
    expect(formatTimeAgo(date)).toBe('5 minutes ago');
  });

  it('formats hours correctly (singular)', () => {
    const date = new Date('2024-01-10T11:00:00Z');
    expect(formatTimeAgo(date)).toBe('1 hour ago');
  });

  it('formats hours correctly (plural)', () => {
    const date = new Date('2024-01-10T09:00:00Z');
    expect(formatTimeAgo(date)).toBe('3 hours ago');
  });

  it('formats days correctly (singular)', () => {
    const date = new Date('2024-01-09T12:00:00Z');
    expect(formatTimeAgo(date)).toBe('1 day ago');
  });

  it('formats days correctly (plural)', () => {
    const date = new Date('2024-01-05T12:00:00Z');
    expect(formatTimeAgo(date)).toBe('5 days ago');
  });

  it('formats months correctly (singular)', () => {
    const date = new Date('2023-12-10T12:00:00Z');
    expect(formatTimeAgo(date)).toBe('1 month ago');
  });

  it('formats months correctly (plural)', () => {
    const date = new Date('2023-10-10T12:00:00Z');
    expect(formatTimeAgo(date)).toBe('3 months ago');
  });

  it('formats years correctly (singular)', () => {
    const date = new Date('2023-01-10T12:00:00Z');
    expect(formatTimeAgo(date)).toBe('1 year ago');
  });

  it('formats years correctly (plural)', () => {
    const date = new Date('2021-01-10T12:00:00Z');
    expect(formatTimeAgo(date)).toBe('3 years ago');
  });

  it('accepts Date objects', () => {
    const date = new Date('2024-01-10T11:00:00Z');
    expect(formatTimeAgo(date)).toBe('1 hour ago');
  });

  it('accepts ISO string dates', () => {
    expect(formatTimeAgo('2024-01-10T11:00:00Z')).toBe('1 hour ago');
  });

  it('accepts timestamps', () => {
    const timestamp = new Date('2024-01-10T11:00:00Z').getTime();
    expect(formatTimeAgo(timestamp)).toBe('1 hour ago');
  });
});
