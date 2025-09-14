import type { Character } from '../../types';
import { Button } from '../button';
import { formatDuration } from '../../utils';

interface CharacterCardProps {
  character: Character;
  isActive: boolean;
  onSelect: () => void;
  onEdit: () => void;
  onDelete: () => void;
  totalPlayTime: number;
  interactive?: boolean;
}

export function CharacterCard({
  character,
  isActive,
  onSelect,
  onEdit,
  onDelete,
  totalPlayTime,
  interactive = true,
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

  const getClassBorderColor = (characterClass: string) => {
    const colors: Record<string, string> = {
      Warrior: 'border-red-500',
      Sorceress: 'border-blue-500',
      Ranger: 'border-green-500',
      Huntress: 'border-yellow-500',
      Monk: 'border-purple-500',
      Mercenary: 'border-orange-500',
      Witch: 'border-pink-500',
    };
    return colors[characterClass] || 'border-zinc-500';
  };

  const getClassBgColor = (characterClass: string) => {
    const colors: Record<string, string> = {
      Warrior: 'from-red-500/10 to-red-600/5',
      Sorceress: 'from-blue-500/10 to-blue-600/5',
      Ranger: 'from-green-500/10 to-green-600/5',
      Huntress: 'from-yellow-500/10 to-yellow-600/5',
      Monk: 'from-purple-500/10 to-purple-600/5',
      Mercenary: 'from-orange-500/10 to-orange-600/5',
      Witch: 'from-pink-500/10 to-pink-600/5',
    };
    return colors[characterClass] || 'from-zinc-500/10 to-zinc-600/5';
  };

  const getClassLevelColors = (characterClass: string) => {
    const colors: Record<string, { bg: string; border: string; text: string }> = {
      Warrior: { bg: 'from-red-500/20 to-red-600/20', border: 'border-red-500/30', text: 'text-red-400' },
      Sorceress: { bg: 'from-blue-500/20 to-blue-600/20', border: 'border-blue-500/30', text: 'text-blue-400' },
      Ranger: { bg: 'from-green-500/20 to-green-600/20', border: 'border-green-500/30', text: 'text-green-400' },
      Huntress: { bg: 'from-yellow-500/20 to-yellow-600/20', border: 'border-yellow-500/30', text: 'text-yellow-400' },
      Monk: { bg: 'from-purple-500/20 to-purple-600/20', border: 'border-purple-500/30', text: 'text-purple-400' },
      Mercenary: { bg: 'from-orange-500/20 to-orange-600/20', border: 'border-orange-500/30', text: 'text-orange-400' },
      Witch: { bg: 'from-pink-500/20 to-pink-600/20', border: 'border-pink-500/30', text: 'text-pink-400' },
    };
    return colors[characterClass] || { bg: 'from-zinc-500/20 to-zinc-600/20', border: 'border-zinc-500/30', text: 'text-zinc-400' };
  };



  return (
    <div
      className={`group relative rounded-xl border transition-all duration-200 overflow-hidden ${
        interactive ? 'cursor-pointer' : ''
      } ${
        isActive
          ? `${getClassBorderColor(character.class)} bg-gradient-to-br ${getClassBgColor(character.class)}`
          : 'border-zinc-700 bg-gradient-to-br from-zinc-800/50 to-zinc-900/30 hover:border-zinc-600 hover:bg-gradient-to-br hover:from-zinc-800/70 hover:to-zinc-900/50'
      }`}
      onClick={interactive ? onSelect : undefined}
    >
      {/* Header Section */}
      <div className='p-5 pb-4'>
        <div className='flex items-center gap-3 mb-5'>
          {/* Character Level */}
          <div className={`w-8 h-8 rounded-full bg-gradient-to-br ${getClassLevelColors(character.class).bg} border ${getClassLevelColors(character.class).border} flex items-center justify-center flex-shrink-0`}>
            <span className={`text-sm font-bold ${getClassLevelColors(character.class).text}`}>{character.level}</span>
          </div>
          
          {/* Character Name */}
          <h3 className='text-xl font-bold text-white truncate flex-1'>
            {character.name}
          </h3>

          {/* Action Buttons */}
          {interactive && (
            <div className='flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200'>
              <div onClick={(e) => e.stopPropagation()}>
                <Button 
                  onClick={() => onEdit()} 
                  variant='outline' 
                  size='sm'
                  className='bg-zinc-800/80 backdrop-blur-sm'
                >
                  Edit
                </Button>
              </div>
              <div onClick={(e) => e.stopPropagation()}>
                <Button
                  onClick={() => onDelete()}
                  variant='outline'
                  size='sm'
                  className='text-red-400 hover:text-red-300 hover:border-red-400 bg-zinc-800/80 backdrop-blur-sm'
                >
                  Delete
                </Button>
              </div>
            </div>
          )}
        </div>

        {/* Character Details */}
        <div className='flex items-center gap-6 mb-2'>
          <div className='space-y-1'>
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>Class</div>
            <div className={`text-sm font-medium ${getClassColor(character.class)}`}>
              {character.class}
            </div>
          </div>
          <div className='space-y-1'>
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>Ascendency</div>
            <div className='text-sm text-zinc-300 font-medium'>{character.ascendency}</div>
          </div>
          <div className='space-y-1'>
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>League</div>
            <div className='text-sm text-zinc-300 font-medium'>{character.league}</div>
          </div>
        </div>

        {/* Special Modes */}
        {(character.hardcore || character.solo_self_found) && (
          <div className='flex gap-2 mb-4'>
            {character.hardcore && (
              <span className='px-3 py-1.5 text-xs font-semibold bg-red-500/20 text-red-400 rounded-lg border border-red-500/30'>
                Hardcore
              </span>
            )}
            {character.solo_self_found && (
              <span className='px-3 py-1.5 text-xs font-semibold bg-yellow-500/20 text-yellow-400 rounded-lg border border-yellow-500/30'>
                SSF
              </span>
            )}
          </div>
        )}
      </div>

      {/* Footer Section */}
      <div className='px-5 py-4 bg-gradient-to-r from-zinc-900/50 to-zinc-800/30 border-t border-zinc-700/50'>
        <div className='grid grid-cols-1 gap-3'>
          {/* Play Time */}
          <div className='flex items-center justify-between'>
            <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>Play Time</span>
            <span className='text-sm font-medium text-zinc-300'>{formatDuration(totalPlayTime)}</span>
          </div>
          
          {/* Deaths */}
          {character.death_count > 0 && (
            <div className='flex items-center justify-between'>
              <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>Deaths</span>
              <span className='text-sm font-medium text-zinc-300'>{character.death_count}</span>
            </div>
          )}
          
          {/* Last Played */}
          {character.last_played && (
            <div className='flex items-center justify-between'>
              <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>Last Played</span>
              <span className='text-sm font-medium text-zinc-300'>{formatDate(character.last_played)}</span>
            </div>
          )}
          
          {/* Last Location */}
          {character.last_known_location && (
            <div className='flex items-center justify-between'>
              <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>Location</span>
              <span className='text-sm font-medium text-zinc-300 truncate max-w-32' title={character.last_known_location.location_name}>
                {character.last_known_location.location_name}
              </span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
