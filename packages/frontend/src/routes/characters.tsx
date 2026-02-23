import { UsersIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { useMemo, useState } from 'react';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import { CharacterFormModal } from '../components/character/character-form-modal/character-form-modal';
import { CharacterList } from '../components/character/character-list/character-list';
import { DeleteCharacterModal } from '../components/character/delete-character-modal/delete-character-modal';
import { ClassDistributionChart } from '../components/charts/class-distribution-chart/class-distribution-chart';
import { LeagueDistributionChart } from '../components/charts/league-distribution-chart/league-distribution-chart';
import { AlertMessage } from '../components/forms/form-alert-message/form-alert-message';
import { CharacterInsights } from '../components/insights/character-insights/character-insights';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { Card } from '../components/ui/card/card';
import { LoadingSpinner } from '../components/ui/loading-spinner/loading-spinner';
import { useCharacter } from '../contexts/CharacterContext';
import {
  type CharacterFilters,
  type CharacterSortField,
  characterListConfig,
} from '../hooks/configs/character-list.config';
import { useListControls } from '../hooks/useListControls';
import {
  useCreateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
  useUpdateCharacter,
} from '../queries/characters';
import type { CharacterSummaryData } from '../types/character';

export const Route = createFileRoute('/characters')({
  component: CharactersPage,
});

function CharactersPage() {
  const { characters, activeCharacter, isLoading, error } = useCharacter();
  const createCharacterMutation = useCreateCharacter();
  const updateCharacterMutation = useUpdateCharacter();
  const deleteCharacterMutation = useDeleteCharacter();
  const setActiveCharacterMutation = useSetActiveCharacter();
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingCharacter, setEditingCharacter] = useState<CharacterSummaryData | null>(null);
  const [deletingCharacter, setDeletingCharacter] = useState<CharacterSummaryData | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    resetAll,
    hasActiveFilters,
    activeFilterCount,
    activeChips,
    result: filteredCharacters,
    filteredCount,
    totalCount,
  } = useListControls(characters, characterListConfig) as ReturnType<
    typeof useListControls<CharacterSummaryData, CharacterFilters, CharacterSortField>
  > & {
    filters: CharacterFilters;
    updateFilter: <K extends keyof CharacterFilters>(key: K, value: CharacterFilters[K]) => void;
  };

  const handleCreateCharacter = async (data: CharacterFormData) => {
    try {
      setIsSubmitting(true);
      await createCharacterMutation.mutateAsync(data);
      setShowCreateModal(false);
    } catch {
      // Error is handled by the parent component
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleUpdateCharacter = async (data: CharacterFormData) => {
    if (!editingCharacter) return;

    try {
      setIsSubmitting(true);
      await updateCharacterMutation.mutateAsync({
        characterId: editingCharacter.id,
        data,
        currentLevel: editingCharacter.level,
      });
      setEditingCharacter(null);
    } catch {
      // Error is handled by the parent component
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleEditCharacter = (character: CharacterSummaryData) => {
    setEditingCharacter(character);
  };

  const handleDeleteCharacter = (characterId: string) => {
    const character = characters.find(c => c.id === characterId);
    if (character) {
      setDeletingCharacter(character);
    }
  };

  const confirmDeleteCharacter = async () => {
    if (!deletingCharacter) return;

    try {
      setIsSubmitting(true);
      await deleteCharacterMutation.mutateAsync(deletingCharacter.id);
      setDeletingCharacter(null);
    } catch {
      // Error is handled by the parent component
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleSelectCharacter = async (characterId: string) => {
    try {
      await setActiveCharacterMutation.mutateAsync(characterId);
    } catch {
      // Error is handled by the parent component
    }
  };

  const leftColumn = (
    <Card
      title="Characters"
      subtitle={`${filteredCount} of ${totalCount} characters`}
      icon={<UsersIcon />}
      rightAction={{
        label: 'Create Character',
        onClick: () => setShowCreateModal(true),
      }}>
      {isLoading ? (
        <LoadingSpinner message="Loading characters..." className="h-64" />
      ) : (
        <CharacterList
          characters={filteredCharacters}
          activeCharacterId={activeCharacter?.id}
          onSelectCharacter={handleSelectCharacter}
          onEditCharacter={handleEditCharacter}
          onDeleteCharacter={handleDeleteCharacter}
          onCreateCharacter={() => setShowCreateModal(true)}
          filters={filters}
          onFilterChange={updateFilter}
          onClearFilters={clearFilters}
          hasActiveFilters={hasActiveFilters}
          activeFilterCount={activeFilterCount}
          activeChips={activeChips}
          sort={sort}
          onSortChange={updateSort}
          onResetSort={resetSort}
          onResetAll={resetAll}
          filteredCount={filteredCount}
          totalCount={totalCount}
        />
      )}
    </Card>
  );

  const classDistribution = useMemo(
    () =>
      characters.reduce(
        (acc, c) => {
          acc[c.class] = (acc[c.class] || 0) + 1;
          return acc;
        },
        {} as Record<string, number>,
      ),
    [characters],
  );

  const leagueDistribution = useMemo(
    () =>
      characters.reduce(
        (acc, c) => {
          acc[c.league] = (acc[c.league] || 0) + 1;
          return acc;
        },
        {} as Record<string, number>,
      ),
    [characters],
  );

  const rightColumn = (
    <div className="space-y-4">
      <CharacterInsights characters={characters} />
      <LeagueDistributionChart leagueDistribution={leagueDistribution} />
      <ClassDistributionChart classDistribution={classDistribution} />
    </div>
  );

  return (
    <PageLayout
      leftColumn={
        <>
          {error && (
            <div className="mb-6">
              <AlertMessage type="error" message={error.message} />
            </div>
          )}
          {leftColumn}
        </>
      }
      rightColumn={rightColumn}>
      <CharacterFormModal
        isOpen={showCreateModal}
        onSubmit={handleCreateCharacter}
        onClose={() => setShowCreateModal(false)}
        isLoading={isSubmitting}
      />
      <CharacterFormModal
        isOpen={!!editingCharacter}
        character={editingCharacter || undefined}
        onSubmit={handleUpdateCharacter}
        onClose={() => setEditingCharacter(null)}
        isLoading={isSubmitting}
      />
      <DeleteCharacterModal
        isOpen={!!deletingCharacter}
        character={deletingCharacter || undefined}
        onConfirm={confirmDeleteCharacter}
        onCancel={() => setDeletingCharacter(null)}
        isLoading={isSubmitting}
      />
    </PageLayout>
  );
}
