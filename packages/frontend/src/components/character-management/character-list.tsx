import type { Character } from '../../types';
import { Button } from '../button';
import { formatDuration } from '../../utils';

interface CharacterListProps {
  characters: Character[];
  activeCharacterId?: string;
  onSelectCharacter: (characterId: string) => void;
  onEditCharacter: (character: Character) => void;
  onDeleteCharacter: (characterId: string) => void;
  onCreateCharacter: () => void;
  getPlayTime: (characterId: string) => number;
}

export function CharacterList({
  characters,
  activeCharacterId,
  onSelectCharacter,
  onEditCharacter,
  onDeleteCharacter,
  onCreateCharacter,
  getPlayTime,
}: CharacterListProps) {
  if (characters.length === 0) {
    return (
      <div className='text-center py-12'>
        <div className='text-zinc-400 mb-4'>
          <svg
            className='mx-auto h-12 w-12'
            fill='none'
            viewBox='0 0 24 24'
            stroke='currentColor'
          >
            <path
              strokeLinecap='round'
              strokeLinejoin='round'
              strokeWidth={1.5}
              d='M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z'
            />
          </svg>
        </div>
        <h3 className='text-lg font-medium text-white mb-2'>No Characters</h3>
        <p className='text-zinc-400 mb-6'>
          Create your first character to start tracking your adventures.
        </p>
        <Button onClick={onCreateCharacter} variant='primary'>
          Create Character
        </Button>
      </div>
    );
  }

  return (
    <div className='space-y-4'>
      <div className='flex justify-between items-center'>
        <h2 className='text-xl font-semibold text-white'>Your Characters</h2>
        <Button onClick={onCreateCharacter} variant='primary' size='sm'>
          Create Character
        </Button>
      </div>

      <div className='grid gap-3'>
        {characters.map(character => (
          <CharacterCard
            key={character.id}
            character={character}
            isActive={character.id === activeCharacterId}
            onSelect={() => onSelectCharacter(character.id)}
            onEdit={() => onEditCharacter(character)}
            onDelete={() => onDeleteCharacter(character.id)}
            totalPlayTime={getPlayTime(character.id)}
          />
        ))}
      </div>
    </div>
  );
}

interface CharacterCardProps {
  character: Character;
  isActive: boolean;
  onSelect: () => void;
  onEdit: () => void;
  onDelete: () => void;
  totalPlayTime: number;
}

function CharacterCard({
  character,
  isActive,
  onSelect,
  onEdit,
  onDelete,
  totalPlayTime,
}: CharacterCardProps) {
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  const getClassColor = (characterClass: string) => {
    const colors: Record<string, string> = {
      Warrior: 'text-red-400',
      Sorceress: 'text-blue-400',
      Ranger: 'text-green-400',
      Huntress: 'text-yellow-400',
      Monk: 'text-purple-400',
      Mercenary: 'text-orange-400',
      Witch: 'text-pink-400',
    };
    return colors[characterClass] || 'text-zinc-400';
  };

  return (
    <div
      className={`p-4 rounded-lg border transition-all cursor-pointer ${
        isActive
          ? 'border-blue-500 bg-blue-500/10'
          : 'border-zinc-700 bg-zinc-800/50 hover:border-zinc-600 hover:bg-zinc-800/70'
      }`}
      onClick={onSelect}
    >
      <div className='flex items-center justify-between'>
        <div className='flex-1'>
          <div className='flex items-center gap-3 mb-2'>
            <h3 className='text-lg font-semibold text-white'>
              {character.name}
            </h3>
            {isActive && (
              <span className='px-2 py-1 text-xs font-medium bg-blue-500 text-white rounded-full'>
                Active
              </span>
            )}
          </div>

          <div className='flex items-center gap-4 text-sm text-zinc-400'>
            <span className={getClassColor(character.class)}>
              {character.class}
            </span>
            <span>•</span>
            <span>{character.ascendency}</span>
            <span>•</span>
            <span>{character.league}</span>
          </div>

          <div className='flex items-center gap-4 mt-2 text-xs text-zinc-500'>
            <span>Created: {formatDate(character.created_at)}</span>
            {character.last_played && (
              <>
                <span>•</span>
                <span>Last played: {formatDate(character.last_played)}</span>
              </>
            )}
            {character.last_known_location && (
              <>
                <span>•</span>
                <span className='text-zinc-500'>
                  Last seen: {character.last_known_location.location_name}
                </span>
              </>
            )}
            <span>•</span>
            <span className='text-zinc-500'>
              Play time: {formatDuration(totalPlayTime)}
            </span>
          </div>

          {(character.hardcore || character.solo_self_found) && (
            <div className='flex gap-2 mt-2'>
              {character.hardcore && (
                <span className='px-2 py-1 text-xs font-medium bg-red-500/20 text-red-400 rounded'>
                  Hardcore
                </span>
              )}
              {character.solo_self_found && (
                <span className='px-2 py-1 text-xs font-medium bg-yellow-500/20 text-yellow-400 rounded'>
                  SSF
                </span>
              )}
            </div>
          )}
        </div>

        <div className='flex gap-2 ml-4'>
          <Button onClick={() => onEdit()} variant='outline' size='sm'>
            Edit
          </Button>
          <Button
            onClick={() => onDelete()}
            variant='outline'
            size='sm'
            className='text-red-400 hover:text-red-300 hover:border-red-400'
          >
            Delete
          </Button>
        </div>
      </div>
    </div>
  );
}
