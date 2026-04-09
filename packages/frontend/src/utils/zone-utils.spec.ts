import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { ZoneMetadata } from '../queries/zones';
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
    expect(zone.act).toBeUndefined();
    expect(zone.area_level).toBeUndefined();
    expect(zone.description).toBeUndefined();
    expect(zone.image_url).toBeUndefined();
    expect(zone.wiki_url).toBeUndefined();
    expect(zone.last_updated).toBeUndefined();
  });

  it('merges wiki metadata when provided', () => {
    const metadata: ZoneMetadata = {
      zone_name: 'Test Zone',
      zone_type: 'area',
      act: 1,
      area_level: 5,
      is_town: false,
      has_waypoint: true,
      bosses: ['Boss A'],
      npcs: ['NPC C'],
      connected_zones: ['Other Zone'],
      description: 'A test zone description.',
      points_of_interest: ['POI 1'],
      image_url: 'https://example.com/zone.jpg',
      first_discovered: '2024-01-10T12:00:00Z',
      last_updated: '2024-01-10T12:00:00Z',
      wiki_url: 'https://wiki.example.com/zone',
    };

    const zone = createPlaceholderZone('Test Zone', metadata);

    // Player stats stay at zero
    expect(zone.visits).toBe(0);
    expect(zone.duration).toBe(0);
    expect(zone.deaths).toBe(0);

    // Wiki fields populated from metadata
    expect(zone.act).toBe(1);
    expect(zone.area_level).toBe(5);
    expect(zone.has_waypoint).toBe(true);
    expect(zone.bosses).toEqual(['Boss A']);
    expect(zone.npcs).toEqual(['NPC C']);
    expect(zone.connected_zones).toEqual(['Other Zone']);
    expect(zone.description).toBe('A test zone description.');
    expect(zone.points_of_interest).toEqual(['POI 1']);
    expect(zone.image_url).toBe('https://example.com/zone.jpg');
    expect(zone.wiki_url).toBe('https://wiki.example.com/zone');
    expect(zone.last_updated).toBe('2024-01-10T12:00:00Z');
  });
});
