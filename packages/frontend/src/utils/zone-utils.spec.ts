import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createMockLocation, createMockZone } from '../test/mock-data';
import { createPlaceholderZone, getDisplayAct } from './zone-utils';

describe('getDisplayAct', () => {
  it('returns "Endgame" for hideout zones', () => {
    const zone = createMockZone({ zone_name: 'My Hideout', act: undefined });
    expect(getDisplayAct(zone)).toBe('Endgame');
  });

  it('returns "Endgame" for act 10', () => {
    const zone = createMockZone({ act: 10 });
    expect(getDisplayAct(zone)).toBe('Endgame');
  });

  it('returns "Interlude" for act 6', () => {
    const zone = createMockZone({ act: 6 });
    expect(getDisplayAct(zone)).toBe('Interlude');
  });

  it('returns "Act {n}" for acts 1-5', () => {
    expect(getDisplayAct(createMockZone({ act: 1 }))).toBe('Act 1');
    expect(getDisplayAct(createMockZone({ act: 2 }))).toBe('Act 2');
    expect(getDisplayAct(createMockZone({ act: 3 }))).toBe('Act 3');
    expect(getDisplayAct(createMockZone({ act: 4 }))).toBe('Act 4');
    expect(getDisplayAct(createMockZone({ act: 5 }))).toBe('Act 5');
  });

  it('returns undefined when act is not defined', () => {
    const zone = createMockZone({ zone_name: 'Unknown Zone', act: undefined });
    expect(getDisplayAct(zone)).toBeUndefined();
  });

  it('works with EnrichedLocationState', () => {
    const location = createMockLocation({ act: 3 });
    expect(getDisplayAct(location)).toBe('Act 3');
  });

  it('case-insensitively matches hideout', () => {
    expect(getDisplayAct(createMockZone({ zone_name: 'HIDEOUT' }))).toBe('Endgame');
    expect(getDisplayAct(createMockZone({ zone_name: 'hideout' }))).toBe('Endgame');
    expect(getDisplayAct(createMockZone({ zone_name: 'My hideout' }))).toBe('Endgame');
  });
});

describe('createPlaceholderZone', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2024-01-10T12:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('creates a zone with the given name', () => {
    const zone = createPlaceholderZone('Test Zone');
    expect(zone.zone_name).toBe('Test Zone');
  });

  it('creates a zone with all default values', () => {
    const zone = createPlaceholderZone('Test Zone');
    expect(zone.duration).toBe(0);
    expect(zone.deaths).toBe(0);
    expect(zone.visits).toBe(0);
    expect(zone.is_active).toBe(false);
    expect(zone.is_town).toBe(false);
    expect(zone.has_waypoint).toBe(false);
  });

  it('creates a zone with empty arrays', () => {
    const zone = createPlaceholderZone('Test Zone');
    expect(zone.bosses).toEqual([]);
    expect(zone.monsters).toEqual([]);
    expect(zone.npcs).toEqual([]);
    expect(zone.connected_zones).toEqual([]);
    expect(zone.points_of_interest).toEqual([]);
  });

  it('sets timestamps to current time', () => {
    const zone = createPlaceholderZone('Test Zone');
    expect(zone.first_visited).toBe('2024-01-10T12:00:00.000Z');
    expect(zone.last_visited).toBe('2024-01-10T12:00:00.000Z');
  });

  it('sets optional fields to undefined', () => {
    const zone = createPlaceholderZone('Test Zone');
    expect(zone.entry_timestamp).toBeUndefined();
    expect(zone.area_id).toBeUndefined();
    expect(zone.act).toBeUndefined();
    expect(zone.area_level).toBeUndefined();
    expect(zone.description).toBeUndefined();
    expect(zone.image_url).toBeUndefined();
    expect(zone.wiki_url).toBeUndefined();
    expect(zone.last_updated).toBeUndefined();
  });
});
