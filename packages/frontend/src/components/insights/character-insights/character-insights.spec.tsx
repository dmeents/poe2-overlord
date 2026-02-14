import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { CharacterData, CharacterSummary } from '@/types/character';
import { CharacterInsights } from './character-insights';

const createMockSummary = (overrides: Partial<CharacterSummary> = {}): CharacterSummary => ({
  character_id: 'char-1',
  total_play_time: 3600,
  total_hideout_time: 600,
  total_zones_visited: 10,
  total_deaths: 2,
  play_time_act1: 1800,
  play_time_act2: 900,
  play_time_act3: 600,
  play_time_act4: 300,
  play_time_interlude: 0,
  play_time_endgame: 0,
  ...overrides,
});

const createMockCharacter = (overrides: Partial<CharacterData> = {}): CharacterData =>
  ({
    id: 'char-1',
    name: 'TestChar',
    class: 'Witch',
    ascendency: 'Infernalist',
    league: 'Standard',
    hardcore: false,
    solo_self_found: false,
    level: 50,
    created_at: '2024-01-01T00:00:00Z',
    last_updated: '2024-01-10T00:00:00Z',
    summary: createMockSummary(),
    zones: [],
    walkthrough_progress: { current_step_id: null, completed_step_ids: [] },
    ...overrides,
  }) as CharacterData;

describe('CharacterInsights', () => {
  describe('Empty State', () => {
    it('shows empty state when no characters', () => {
      render(<CharacterInsights characters={[]} />);

      expect(screen.getByText('No Characters to Analyze')).toBeInTheDocument();
      expect(screen.getByText('Create some characters to view insights')).toBeInTheDocument();
    });

    it('renders Character Insights title in empty state', () => {
      render(<CharacterInsights characters={[]} />);

      expect(screen.getByText('Character Insights')).toBeInTheDocument();
    });
  });

  describe('Rendering', () => {
    it('renders Insights title when characters exist', () => {
      render(<CharacterInsights characters={[createMockCharacter()]} />);

      expect(screen.getByText('Insights')).toBeInTheDocument();
    });

    it('renders total characters count', () => {
      const characters = [
        createMockCharacter({ id: 'char-1' }),
        createMockCharacter({ id: 'char-2' }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Total Characters')).toBeInTheDocument();
      // There may be multiple '2' values (total characters, total deaths, etc.)
      expect(screen.getAllByText('2').length).toBeGreaterThanOrEqual(1);
    });

    it('renders highest level', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', level: 50 }),
        createMockCharacter({ id: 'char-2', level: 75 }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Highest Level')).toBeInTheDocument();
      expect(screen.getByText('75')).toBeInTheDocument();
    });

    it('renders average level', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', level: 40 }),
        createMockCharacter({ id: 'char-2', level: 60 }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Average Level')).toBeInTheDocument();
      expect(screen.getByText('50')).toBeInTheDocument();
    });

    it('renders total deaths', () => {
      const characters = [
        createMockCharacter({
          id: 'char-1',
          summary: createMockSummary({ total_deaths: 5 }),
        }),
        createMockCharacter({
          id: 'char-2',
          summary: createMockSummary({ total_deaths: 3 }),
        }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Total Deaths')).toBeInTheDocument();
      expect(screen.getByText('8')).toBeInTheDocument();
    });
  });

  describe('Most Played Character', () => {
    it('renders most played character', () => {
      const characters = [
        createMockCharacter({
          id: 'char-1',
          name: 'CasualPlayer',
          level: 30,
          summary: createMockSummary({ total_play_time: 3600 }), // 1 hour
        }),
        createMockCharacter({
          id: 'char-2',
          name: 'HardcoreGamer',
          level: 80,
          summary: createMockSummary({ total_play_time: 36000 }), // 10 hours
        }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Most Played')).toBeInTheDocument();
      expect(screen.getByText('HardcoreGamer')).toBeInTheDocument();
      expect(screen.getByText('Level 80 • 10h')).toBeInTheDocument();
    });
  });

  describe('Play Time Formatting', () => {
    it('formats total play time in hours and minutes', () => {
      const characters = [
        createMockCharacter({
          summary: createMockSummary({ total_play_time: 5400 }), // 1h 30m
        }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Total Play Time')).toBeInTheDocument();
      expect(screen.getByText('1h 30m')).toBeInTheDocument();
    });

    it('formats play time as minutes when under 1 hour', () => {
      const characters = [
        createMockCharacter({
          summary: createMockSummary({ total_play_time: 1800 }), // 30m
        }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('30m')).toBeInTheDocument();
    });
  });

  describe('Hardcore and SSF Counts', () => {
    it('renders hardcore character count', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', hardcore: true }),
        createMockCharacter({ id: 'char-2', hardcore: false }),
        createMockCharacter({ id: 'char-3', hardcore: true }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Hardcore Characters')).toBeInTheDocument();
      // Need to find 2 for hardcore count (there are multiple 2s so use getAllByText)
      const twoElements = screen.getAllByText('2');
      expect(twoElements.length).toBeGreaterThanOrEqual(1);
    });

    it('renders SSF character count', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', solo_self_found: true }),
        createMockCharacter({ id: 'char-2', solo_self_found: false }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('SSF Characters')).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('handles single character', () => {
      const character = createMockCharacter({
        name: 'OnlyChar',
        level: 45,
      });

      render(<CharacterInsights characters={[character]} />);

      expect(screen.getByText('OnlyChar')).toBeInTheDocument();
      // 45 appears in both highest level and average level
      expect(screen.getAllByText('45').length).toBeGreaterThanOrEqual(1);
    });

    it('handles characters with zero play time', () => {
      const characters = [
        createMockCharacter({
          summary: createMockSummary({ total_play_time: 0 }),
        }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('0m')).toBeInTheDocument();
    });

    it('handles characters with zero deaths', () => {
      const characters = [
        createMockCharacter({
          summary: createMockSummary({ total_deaths: 0 }),
        }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Total Deaths')).toBeInTheDocument();
    });

    it('calculates average level correctly with odd number of characters', () => {
      const characters = [
        createMockCharacter({ id: 'char-1', level: 10 }),
        createMockCharacter({ id: 'char-2', level: 20 }),
        createMockCharacter({ id: 'char-3', level: 30 }),
      ];

      render(<CharacterInsights characters={characters} />);

      expect(screen.getByText('Average Level')).toBeInTheDocument();
      // Average of 10, 20, 30 = 20
      expect(screen.getByText('20')).toBeInTheDocument();
    });
  });
});
