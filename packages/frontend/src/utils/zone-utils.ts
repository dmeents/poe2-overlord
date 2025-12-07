import type { ZoneStats } from '../types/character';

/**
 * Creates a placeholder ZoneStats object for zones that haven't been visited yet.
 * This is useful when displaying zone information in modals or UI elements
 * before the player has actually entered the zone.
 *
 * @param zoneName - The name of the zone to create a placeholder for
 * @returns A ZoneStats object with default/empty values
 */
export function createPlaceholderZone(zoneName: string): ZoneStats {
  const now = new Date().toISOString();

  return {
    zone_name: zoneName,
    duration: 0,
    deaths: 0,
    visits: 0,
    first_visited: now,
    last_visited: now,
    is_active: false,
    entry_timestamp: undefined,
    area_id: undefined,
    act: undefined,
    area_level: undefined,
    is_town: false,
    has_waypoint: false,
    bosses: [],
    monsters: [],
    npcs: [],
    connected_zones: [],
    description: undefined,
    points_of_interest: [],
    image_url: undefined,
    wiki_url: undefined,
    last_updated: undefined,
  };
}
