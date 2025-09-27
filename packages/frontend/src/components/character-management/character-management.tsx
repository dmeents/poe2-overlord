import { useState } from 'react';
import { useCharacterFiltering, useCharacterFilters } from '../../hooks';
import type { CharacterData } from '../../types';
import { CharacterList } from '../character-list/character-list';
import type { CharacterFormData } from '../character-modals';
import { CharacterFormModal, DeleteCharacterModal } from '../character-modals';

interface CharacterManagementProps {
  characters: CharacterData[];
  activeCharacter?: CharacterData;
  createCharacter: (data: CharacterFormData) => Promise<void>;
  updateCharacter: (id: string, data: CharacterFormData) => Promise<void>;
  deleteCharacter: (id: string) => Promise<void>;
  setActiveCharacterId: (id: string) => Promise<void>;
}

export function CharacterManagement({
  characters,
  activeCharacter,
  createCharacter,
  updateCharacter,
  deleteCharacter,
  setActiveCharacterId,
}: CharacterManagementProps) {
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

  return (
    <>
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
    </>
  );
}
