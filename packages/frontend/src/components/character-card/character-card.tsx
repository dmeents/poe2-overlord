import type { Character } from '../../types';
import { formatDuration } from '../../utils';
import { Button } from '../button';
import {
  formatDate,
  getClassBgColor,
  getClassBorderColor,
  getClassColor,
  getClassLevelColors,
} from './character-card.styles';

export interface CharacterCardProps {
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
          <div
            className={`w-8 h-8 rounded-full bg-gradient-to-br ${getClassLevelColors(character.class).bg} border ${getClassLevelColors(character.class).border} flex items-center justify-center flex-shrink-0`}
          >
            <span
              className={`text-sm font-bold ${getClassLevelColors(character.class).text}`}
            >
              {character.level}
            </span>
          </div>

          {/* Character Name */}
          <h3 className='text-xl font-bold text-white truncate flex-1'>
            {character.name}
          </h3>

          {/* Action Buttons */}
          {interactive && (
            <div className='flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200'>
              <div onClick={e => e.stopPropagation()}>
                <Button
                  onClick={() => onEdit()}
                  variant='outline'
                  size='sm'
                  className='bg-zinc-800/80 backdrop-blur-sm'
                >
                  Edit
                </Button>
              </div>
              <div onClick={e => e.stopPropagation()}>
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
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
              Class
            </div>
            <div
              className={`text-sm font-medium ${getClassColor(character.class)}`}
            >
              {character.class}
            </div>
          </div>
          <div className='space-y-1'>
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
              Ascendency
            </div>
            <div className='text-sm text-zinc-300 font-medium'>
              {character.ascendency}
            </div>
          </div>
          <div className='space-y-1'>
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
              League
            </div>
            <div className='text-sm text-zinc-300 font-medium'>
              {character.league}
            </div>
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
            <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
              Play Time
            </span>
            <span className='text-sm font-medium text-zinc-300'>
              {formatDuration(totalPlayTime)}
            </span>
          </div>

          {/* Deaths */}
          {character.death_count > 0 && (
            <div className='flex items-center justify-between'>
              <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                Deaths
              </span>
              <span className='text-sm font-medium text-zinc-300'>
                {character.death_count}
              </span>
            </div>
          )}

          {/* Last Played */}
          {character.last_played && (
            <div className='flex items-center justify-between'>
              <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                Last Played
              </span>
              <span className='text-sm font-medium text-zinc-300'>
                {formatDate(character.last_played)}
              </span>
            </div>
          )}

          {/* Last Location */}
          {character.last_known_location && (
            <div className='flex items-center justify-between'>
              <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                Location
              </span>
              <span
                className='text-sm font-medium text-zinc-300 truncate max-w-32'
                title={character.last_known_location.location_name}
              >
                {character.last_known_location.location_name}
              </span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
