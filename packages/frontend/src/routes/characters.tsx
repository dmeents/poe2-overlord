import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import {
  CharacterFormModal,
  CharacterList,
  DeleteCharacterModal,
} from '../components/character-management';
import type { CharacterFormData } from '../components/character-management/character-form-modal';
import { AlertMessage } from '../components/form/alert-message';
import { PageHeader } from '../components/page-header';
import { useCharacterManagement } from '../hooks/useCharacterManagement';
import { useCharacterTotalPlayTime } from '../hooks/useCharacterTotalPlayTime';
import type { Character } from '../types';

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

  const { getPlayTime } = useCharacterTotalPlayTime(characters);

  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingCharacter, setEditingCharacter] = useState<Character | null>(
    null
  );
  const [deletingCharacter, setDeletingCharacter] = useState<Character | null>(
    null
  );
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleCreateCharacter = async (data: CharacterFormData) => {
    try {
      setIsSubmitting(true);
      await createCharacter(data);
      setShowCreateModal(false);
    } catch {
      // Error is handled by the hook
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
      // Error is handled by the hook
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleEditCharacter = (character: Character) => {
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
      // Error is handled by the hook
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleSelectCharacter = async (characterId: string) => {
    try {
      await setActiveCharacterId(characterId);
    } catch {
      // Error is handled by the hook
    }
  };

  if (isLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <PageHeader
          title='Character Management'
          subtitle='Manage your Path of Exile 2 characters and track their individual progress.'
        />
        <div className='max-w-4xl mx-auto px-6'>
          <div className='flex items-center justify-center h-64'>
            <div className='text-center'>
              <div className='animate-spin rounded-full h-8 w-8 border-b-2 border-white mx-auto mb-4'></div>
              <p className='text-zinc-400'>Loading characters...</p>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <PageHeader
        title='Character Management'
        subtitle='Manage your Path of Exile 2 characters and track their individual progress.'
      />
      <div className='max-w-4xl mx-auto px-6'>
        {error && (
          <div className='mb-6'>
            <AlertMessage type='error' message={error} />
          </div>
        )}


        <CharacterList
          characters={characters}
          activeCharacterId={activeCharacter?.id}
          onSelectCharacter={handleSelectCharacter}
          onEditCharacter={handleEditCharacter}
          onDeleteCharacter={handleDeleteCharacter}
          onCreateCharacter={() => setShowCreateModal(true)}
          getPlayTime={getPlayTime}
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
      </div>
    </div>
  );
}
