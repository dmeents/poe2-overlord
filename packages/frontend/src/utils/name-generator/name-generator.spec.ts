import { describe, expect, it } from 'vitest';
import { generateFantasyName } from './name-generator';

describe('generateFantasyName', () => {
  it('generates a valid name', () => {
    const name = generateFantasyName();
    expect(name).toBeTruthy();
    expect(typeof name).toBe('string');
    expect(name.length).toBeGreaterThan(0);
  });

  it('capitalizes the first letter', () => {
    const name = generateFantasyName();
    expect(name[0]).toBe(name[0].toUpperCase());
  });

  it('respects min length constraint', () => {
    const name = generateFantasyName({ minLength: 5 });
    expect(name.length).toBeGreaterThanOrEqual(5);
  });

  it('respects max length constraint', () => {
    const name = generateFantasyName({ maxLength: 8 });
    expect(name.length).toBeLessThanOrEqual(8);
  });

  it('respects both min and max length constraints', () => {
    const name = generateFantasyName({ minLength: 5, maxLength: 10 });
    expect(name.length).toBeGreaterThanOrEqual(5);
    expect(name.length).toBeLessThanOrEqual(10);
  });

  it('generates names for each style', () => {
    const styles = ['nordic', 'arcane', 'shadow', 'ancient', 'exotic', 'dark', 'primal'] as const;

    for (const style of styles) {
      const name = generateFantasyName({ style });
      expect(name).toBeTruthy();
      expect(typeof name).toBe('string');
    }
  });

  it('generates male-specific names when gender is male', () => {
    const name = generateFantasyName({ gender: 'male' });
    expect(name).toBeTruthy();
  });

  it('generates female-specific names when gender is female', () => {
    const name = generateFantasyName({ gender: 'female' });
    expect(name).toBeTruthy();
  });

  it('respects allowBlending option', () => {
    const name1 = generateFantasyName({ allowBlending: true });
    const name2 = generateFantasyName({ allowBlending: false });
    expect(name1).toBeTruthy();
    expect(name2).toBeTruthy();
  });

  it('generates multiple different names', () => {
    const names = new Set<string>();
    for (let i = 0; i < 100; i++) {
      names.add(generateFantasyName());
    }
    // Should generate at least 50 unique names out of 100
    expect(names.size).toBeGreaterThan(50);
  });

  it('handles edge case of very strict length constraints', () => {
    const name = generateFantasyName({ minLength: 3, maxLength: 4 });
    expect(name.length).toBeGreaterThanOrEqual(3);
    expect(name.length).toBeLessThanOrEqual(4);
  });
});
