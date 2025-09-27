import { createFileRoute } from '@tanstack/react-router';
import { AlertMessage } from '../components';
import { CharacterManagement } from '../components/character-management';
import { PageHeader } from '../components/page-header';
import { useCharacterManagement } from '../hooks/useCharacterManagement';

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

        <CharacterManagement
          characters={characters}
          activeCharacter={activeCharacter || undefined}
          createCharacter={async data => {
            await createCharacter(data);
          }}
          updateCharacter={async (id, data) => {
            await updateCharacter(id, data);
          }}
          deleteCharacter={deleteCharacter}
          setActiveCharacterId={setActiveCharacterId}
        />
      </div>
    </div>
  );
}
