import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { AlertMessage } from '../components/forms/form-alert-message/form-alert-message';
import { Card } from '../components/ui/card/card';
import { CharacterFormModal } from '../components/character/character-form-modal/character-form-modal';
import { CharacterInsights } from '../components/insights/character-insights/character-insights';
import { CharacterList } from '../components/character/character-list/character-list';
import { ClassDistributionChart } from '../components/charts/class-distribution-chart/class-distribution-chart';
import { DeleteCharacterModal } from '../components/character/delete-character-modal/delete-character-modal';
import { LoadingSpinner } from '../components/ui/loading-spinner/loading-spinner';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import type { CharacterFormData } from '../components/character/character-form-modal/character-form-modal';
import { useCharacterList } from '../hooks/useCharacterList';
import { useCharacter } from '../contexts/CharacterContext';
import {
  useCreateCharacter,
  useUpdateCharacter,
  useDeleteCharacter,
  useSetActiveCharacter,
} from '../queries/characters';
import type { CharacterData } from '../types/character';
import { UsersIcon } from '@heroicons/react/24/outline';

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
  const [editingCharacter, setEditingCharacter] =
    useState<CharacterData | null>(null);
  const [deletingCharacter, setDeletingCharacter] =
    useState<CharacterData | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters,
    filteredCharacters,
    totalCount,
  } = useCharacterList(characters);

  // Event handlers
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
      });
      setEditingCharacter(null);
    } catch {
      // Error is handled by the parent component
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleEditCharacter = (character: CharacterData) => {
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

  // Loading state
  if (isLoading) {
    return (
      <PageLayout
        leftColumn={
          <LoadingSpinner message='Loading characters...' className='h-64' />
        }
        rightColumn={<div />}
      />
    );
  }

  const leftColumn = (
    <Card
      title='Characters'
      subtitle={`${filteredCharacters.length} of ${totalCount} characters`}
      icon={<UsersIcon className='w-5 h-5' />}
      rightAction={{
        label: 'Create Character',
        onClick: () => setShowCreateModal(true),
      }}
    >
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
        sort={sort}
        onSortChange={updateSort}
        onResetSort={resetSort}
        totalCount={totalCount}
      />
    </Card>
  );

  // Compute class distribution
  const classDistribution = characters.reduce(
    (acc, c) => {
      acc[c.class] = (acc[c.class] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>
  );

  const rightColumn = (
    <div className='space-y-4'>
      <CharacterInsights characters={characters} />
      <ClassDistributionChart classDistribution={classDistribution} />
    </div>
  );

  return (
    <PageLayout
      leftColumn={
        <>
          {error && (
            <div className='mb-6'>
              <AlertMessage type='error' message={error} />
            </div>
          )}
          {leftColumn}
        </>
      }
      rightColumn={rightColumn}
    >
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
