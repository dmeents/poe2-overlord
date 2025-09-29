import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import {
  AlertMessage,
  Button,
  CharacterFormModal,
  CharacterInsights,
  CharacterList,
  DeleteCharacterModal,
  LoadingSpinner,
} from '../components';
import type { CharacterFormData } from '../components/character-modals';
import { useCharacterFiltering, useCharacterFilters } from '../hooks';
import { useCharacterManagement } from '../hooks/useCharacterManagement';
import type { CharacterData } from '../types';

export const Route = createFileRoute('/characters')({
  component: CharactersPage,
});

function CharactersPage() {
  const {
    characters,
    activeCharacter,
    isLoading,
    error,
    createCharacter,
    updateCharacter,
    setActiveCharacterId,
    deleteCharacter,
  } = useCharacterManagement();

  // Modal and form state management
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingCharacter, setEditingCharacter] =
    useState<CharacterData | null>(null);
  const [deletingCharacter, setDeletingCharacter] =
    useState<CharacterData | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Filter and sort state management
  const {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters,
  } = useCharacterFilters();

  // Apply filtering and sorting
  const { filteredCharacters, characterCount, totalCount } =
    useCharacterFiltering(characters, filters, sort);

  // Event handlers
  const handleCreateCharacter = async (data: CharacterFormData) => {
    try {
      setIsSubmitting(true);
      await createCharacter(data);
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
      await updateCharacter(editingCharacter.id, data);
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
      await deleteCharacter(deletingCharacter.id);
      setDeletingCharacter(null);
    } catch {
      // Error is handled by the parent component
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleSelectCharacter = async (characterId: string) => {
    try {
      await setActiveCharacterId(characterId);
    } catch {
      // Error is handled by the parent component
    }
  };

  // Loading state
  if (isLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <div className='flex items-center justify-center h-64'>
            <div className='text-center'>
              <LoadingSpinner />
              <p className='text-zinc-400 mt-4'>Loading characters...</p>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <div className='px-6 py-8'>
        {error && (
          <div className='mb-6'>
            <AlertMessage type='error' message={error} />
          </div>
        )}

        <div className='grid grid-cols-1 lg:grid-cols-3 gap-8'>
          {/* Left Column - Character Management */}
          <div className='lg:col-span-2'>
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
              characterCount={characterCount}
              totalCount={totalCount}
            />
          </div>

          {/* Right Column - Character Metrics */}
          <div className='lg:col-span-1'>
            <div className='sticky top-8 space-y-4'>
              {/* Insights Card */}
              <CharacterInsights characters={characters} />

              {/* Create Character Button */}
              <Button
                onClick={() => setShowCreateModal(true)}
                variant='primary'
                size='sm'
                className='w-full'
              >
                Create Character
              </Button>
            </div>
          </div>
        </div>

        {/* Create Character Modal */}
        <CharacterFormModal
          isOpen={showCreateModal}
          onSubmit={handleCreateCharacter}
          onClose={() => setShowCreateModal(false)}
          isLoading={isSubmitting}
        />

        {/* Edit Character Modal */}
        <CharacterFormModal
          isOpen={!!editingCharacter}
          character={editingCharacter || undefined}
          onSubmit={handleUpdateCharacter}
          onClose={() => setEditingCharacter(null)}
          isLoading={isSubmitting}
        />

        {/* Delete Character Modal */}
        <DeleteCharacterModal
          isOpen={!!deletingCharacter}
          character={deletingCharacter || undefined}
          onConfirm={confirmDeleteCharacter}
          onCancel={() => setDeletingCharacter(null)}
          isLoading={isSubmitting}
        />
      </div>
    </div>
  );
}
