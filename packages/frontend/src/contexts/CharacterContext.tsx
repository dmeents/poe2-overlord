/* eslint-disable react-refresh/only-export-components */

import { useQueryClient } from '@tanstack/react-query';
import { createContext, useContext, useMemo } from 'react';
import { useAppEventListener } from '@/hooks/useAppEventListener';
import { characterQueryKeys, useActiveCharacter, useCharacters } from '@/queries/characters';
import type { CharacterData, CharacterSummaryData } from '@/types/character';
import type { AppError } from '@/types/error';
import { parseError } from '@/utils/error-handling';
import {
  type CharacterDeletedEvent,
  type CharacterUpdatedEvent,
  EVENT_KEYS,
  type ExtractPayload,
} from '@/utils/events/registry';

interface CharacterContextValue {
  characters: CharacterSummaryData[];
  activeCharacter: CharacterData | null;
  isLoading: boolean;
  error: AppError | null;
}

const CharacterContext = createContext<CharacterContextValue | undefined>(undefined);

function toSummary(data: CharacterData, is_active: boolean): CharacterSummaryData {
  return {
    id: data.id,
    name: data.name,
    class: data.class,
    ascendency: data.ascendency,
    league: data.league,
    hardcore: data.hardcore,
    solo_self_found: data.solo_self_found,
    level: data.level,
    created_at: data.created_at,
    last_played: data.last_played,
    last_updated: data.last_updated,
    current_location: data.current_location,
    summary: data.summary,
    walkthrough_progress: data.walkthrough_progress,
    is_active,
  };
}

export function CharacterProvider({ children }: React.PropsWithChildren) {
  const queryClient = useQueryClient();

  const {
    data: characters = [],
    isLoading: charactersLoading,
    error: charactersError,
  } = useCharacters();

  const {
    data: activeCharacter = null,
    isLoading: activeCharacterLoading,
    error: activeCharacterError,
  } = useActiveCharacter();

  useAppEventListener(
    [
      {
        eventType: EVENT_KEYS.CharacterUpdated,
        handler: (payload: unknown) => {
          const { character_id, data: characterData } =
            payload as ExtractPayload<CharacterUpdatedEvent>;

          // Update active character cache if this is the active character
          queryClient.setQueryData(
            characterQueryKeys.active(),
            (prev: CharacterData | null | undefined) =>
              prev?.id === character_id ? characterData : prev,
          );

          // Update character list cache — preserve is_active from existing entry
          queryClient.setQueryData(
            characterQueryKeys.lists(),
            (prev: CharacterSummaryData[] | undefined) =>
              prev?.map(c => (c.id === character_id ? toSummary(characterData, c.is_active) : c)),
          );

          // Invalidate zones so zone-consuming components fetch fresh data
          queryClient.invalidateQueries({ queryKey: characterQueryKeys.zones(character_id) });
        },
      },
      {
        eventType: EVENT_KEYS.CharacterDeleted,
        handler: (payload: unknown) => {
          const { character_id } = payload as ExtractPayload<CharacterDeletedEvent>;

          queryClient.setQueryData(
            characterQueryKeys.lists(),
            (prev: CharacterSummaryData[] | undefined) => prev?.filter(c => c.id !== character_id),
          );

          queryClient.setQueryData(
            characterQueryKeys.active(),
            (prev: CharacterData | null | undefined) => (prev?.id === character_id ? null : prev),
          );
        },
      },
    ],
    [],
  );

  const isLoading = charactersLoading || activeCharacterLoading;
  const error = charactersError
    ? parseError(charactersError)
    : activeCharacterError
      ? parseError(activeCharacterError)
      : null;

  const value = useMemo(
    () => ({ characters, activeCharacter, isLoading, error }),
    [characters, activeCharacter, isLoading, error],
  );

  return <CharacterContext.Provider value={value}>{children}</CharacterContext.Provider>;
}

export function useCharacter() {
  const context = useContext(CharacterContext);

  if (context === undefined) {
    throw new Error('useCharacter must be used within CharacterProvider');
  }

  return context;
}
