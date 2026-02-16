import type {
  CharacterData,
  CharacterSummary,
  EnrichedLocationState,
  ZoneStats,
} from '../types/character';
import type { CurrencyExchangeRate } from '../types/economy';
import type {
  Objective,
  WalkthroughGuide,
  WalkthroughProgress,
  WalkthroughStep,
} from '../types/walkthrough';

/**
 * Mock data factories for testing.
 * Each factory creates a valid object with sensible defaults and allows overrides.
 */

export function createMockCharacter(overrides?: Partial<CharacterData>): CharacterData {
  const defaultCharacter: CharacterData = {
    id: 'test-character-id',
    name: 'TestCharacter',
    class: 'Warrior',
    ascendency: 'Titan',
    level: 50,
    league: 'Standard',
    hardcore: false,
    solo_self_found: false,
    created_at: '2024-01-01T00:00:00Z',
    last_updated: '2024-01-10T00:00:00Z',
    last_played: '2024-01-10T00:00:00Z',
    current_location: createMockLocation(),
    summary: createMockSummary(),
    zones: [],
    walkthrough_progress: createMockWalkthroughProgress(),
  };

  return { ...defaultCharacter, ...overrides };
}

export function createMockLocation(
  overrides?: Partial<EnrichedLocationState>,
): EnrichedLocationState {
  const defaultLocation: EnrichedLocationState = {
    zone_name: 'The Coast',
    act: 1,
    is_town: false,
    location_type: 'Zone',
    has_waypoint: true,
    area_level: 2,
    last_updated: '2024-01-10T00:00:00Z',
  };

  return { ...defaultLocation, ...overrides };
}

export function createMockSummary(overrides?: Partial<CharacterSummary>): CharacterSummary {
  const defaultSummary: CharacterSummary = {
    character_id: 'test-character-id',
    total_play_time: 3600,
    total_hideout_time: 600,
    total_town_time: 300,
    total_zones_visited: 20,
    total_deaths: 5,
    play_time_act1: 900,
    play_time_act2: 900,
    play_time_act3: 900,
    play_time_act4: 300,
    play_time_act5: 0,
    play_time_interlude: 0,
    play_time_endgame: 0,
  };

  return { ...defaultSummary, ...overrides };
}

export function createMockZone(overrides?: Partial<ZoneStats>): ZoneStats {
  const defaultZone: ZoneStats = {
    zone_name: 'The Coast',
    duration: 300,
    deaths: 1,
    visits: 2,
    first_visited: '2024-01-01T10:00:00Z',
    last_visited: '2024-01-01T10:05:00Z',
    is_active: false,
    act: 1,
    is_town: false,
    has_waypoint: true,
    bosses: [],
    monsters: [],
    npcs: [],
    connected_zones: [],
    points_of_interest: [],
  };

  return { ...defaultZone, ...overrides };
}

export function createMockCurrency(
  overrides?: Partial<CurrencyExchangeRate>,
): CurrencyExchangeRate {
  const defaultCurrency: CurrencyExchangeRate = {
    id: 'test-currency-id',
    name: 'Test Currency',
    image_url: 'https://example.com/currency.png',
    display_value: {
      tier: 'Primary',
      value: 1.5,
      inverted: false,
      currency_id: 'divine-orb',
      currency_name: 'Divine Orb',
      currency_image_url: 'https://example.com/divine.png',
    },
    primary_value: 1.5,
    secondary_value: 150,
    tertiary_value: 2,
    volume: 1000,
    change_percent: 5.5,
    price_history: [1.0, 1.2, 1.5],
  };

  return { ...defaultCurrency, ...overrides };
}

export function createMockWalkthroughProgress(
  overrides?: Partial<WalkthroughProgress>,
): WalkthroughProgress {
  const defaultProgress: WalkthroughProgress = {
    current_step_id: null,
    is_completed: false,
    last_updated: '2024-01-10T00:00:00Z',
  };

  return { ...defaultProgress, ...overrides };
}

export function createMockObjective(overrides?: Partial<Objective>): Objective {
  const defaultObjective: Objective = {
    text: 'Test objective',
    required: true,
    rewards: [],
  };

  return { ...defaultObjective, ...overrides };
}

export function createMockWalkthroughStep(overrides?: Partial<WalkthroughStep>): WalkthroughStep {
  const defaultStep: WalkthroughStep = {
    id: 'test-step-id',
    title: 'Test Step',
    description: 'Test description',
    current_zone: 'The Coast',
    completion_zone: 'Mud Burrow',
    next_step_id: null,
    previous_step_id: null,
    objectives: [createMockObjective()],
    wiki_items: [],
  };

  return { ...defaultStep, ...overrides };
}

export function createMockWalkthroughGuide(
  overrides?: Partial<WalkthroughGuide>,
): WalkthroughGuide {
  const defaultGuide: WalkthroughGuide = {
    acts: {
      'act-1': {
        act_name: 'Act 1',
        act_number: 1,
        steps: {
          'step-1': createMockWalkthroughStep({ id: 'step-1' }),
        },
      },
    },
  };

  return { ...defaultGuide, ...overrides };
}
