import type { ZoneMetadata } from '../queries/zones';
import type { EnrichedLocationState, ZoneStats } from '../types/character';

/**
 * Maps a zone to its display act name.
 *
 * @param zone - The zone stats object or enriched location state
 * @returns The display name for the act (e.g., "Endgame", "Interlude", "Act 1")
 *
 * @example
 * getDisplayAct(hideoutZone) // "Endgame"
 * getDisplayAct(act10Zone) // "Endgame"
 * getDisplayAct(act6Zone) // "Interlude"
 * getDisplayAct(act3Zone) // "Act 3"
 */
export function getDisplayAct(zone: ZoneStats | EnrichedLocationState): string | undefined {
  if (zone.zone_name.toLowerCase().includes('hideout')) {
    return 'Endgame';
  }

  // Act-based mapping
  if (zone.act !== undefined) {
    switch (zone.act) {
      case 10:
        return 'Endgame';
      case 6:
        return 'Interlude';
      default:
        return `Act ${zone.act}`;
    }
  }

  return undefined;
}

/**
 * Creates a placeholder ZoneStats object for zones that haven't been visited yet.
 * This is useful when displaying zone information in modals or UI elements
 * before the player has actually entered the zone.
 *
 * @param zoneName - The name of the zone to create a placeholder for
 * @param metadata - Optional global zone metadata to populate wiki fields
 * @returns A ZoneStats object with zero player stats and optional wiki data
 */
export function createPlaceholderZone(zoneName: string, metadata?: ZoneMetadata): ZoneStats {
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
    area_id: metadata?.area_id,
    act: metadata ? metadata.act || undefined : undefined,
    area_level: metadata?.area_level,
    is_town: metadata?.is_town ?? false,
    has_waypoint: metadata?.has_waypoint ?? false,
    bosses: metadata?.bosses ?? [],
    monsters: metadata?.monsters ?? [],
    npcs: metadata?.npcs ?? [],
    connected_zones: metadata?.connected_zones ?? [],
    description: metadata?.description,
    points_of_interest: metadata?.points_of_interest ?? [],
    image_url: metadata?.image_url,
    wiki_url: metadata?.wiki_url,
    last_updated: metadata?.last_updated,
  };
}
