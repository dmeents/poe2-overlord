import { memo, useCallback, useEffect, useMemo, useState } from 'react';
import type { CharacterData } from '../../../types/character';
import { formatDuration } from '../../../utils/format-duration';
import { getAscendencyImage } from '../../../utils/ascendency-assets';
import { getDisplayAct } from '../../../utils/zone-utils';
import { Button } from '../../ui/button/button';
import {
  formatDate,
  getAscendencyBackgroundStyles,
  getAscendencyOverlayStyles,
  getClassBgColor,
  getClassBorderColor,
  getClassColor,
  getClassLevelColors,
  getHeaderSectionBackgroundStyles,
} from './character-card.styles';

export interface CharacterCardProps {
  character: CharacterData;
  isActive: boolean;
  onSelect: () => void;
  onEdit: () => void;
  onDelete: () => void;
  interactive?: boolean;
  showDetails?: boolean;
}

export const CharacterCard = memo(function CharacterCard({
  character,
  isActive,
  onSelect,
  onEdit,
  onDelete,
  interactive = true,
  showDetails = true,
}: CharacterCardProps) {
  // State for collapsible details - default to expanded for active characters
  const [isDetailsExpanded, setIsDetailsExpanded] = useState(isActive);

  // Update expanded state when character becomes active/inactive
  useEffect(() => {
    setIsDetailsExpanded(isActive);
  }, [isActive]);

  // Get current location (always needed for header display)
  const currentLocation = character.current_location;

  // Memoize expensive computations
  const ascendencyImage = useMemo(
    () => getAscendencyImage(character.ascendency),
    [character.ascendency]
  );
  const backgroundStyles = useMemo(
    () => getAscendencyBackgroundStyles(ascendencyImage),
    [ascendencyImage]
  );
  const overlayStyles = useMemo(() => getAscendencyOverlayStyles(), []);

  const headerBackgroundStyles = useMemo(
    () => getHeaderSectionBackgroundStyles(character.hardcore),
    [character.hardcore]
  );

  // Memoize class computations
  const classColor = useMemo(
    () => getClassColor(character.class),
    [character.class]
  );
  const classBorderColor = useMemo(
    () => getClassBorderColor(character.class),
    [character.class]
  );
  const classBgColor = useMemo(
    () => getClassBgColor(character.class),
    [character.class]
  );
  const classLevelColors = useMemo(
    () => getClassLevelColors(character.class),
    [character.class]
  );

  // Memoize helper function
  const formatCurrentLocation = useCallback(
    (location: typeof currentLocation) => {
      if (!location) return 'Unknown';

      const parts = [];
      const displayAct = getDisplayAct(location);
      if (displayAct) parts.push(displayAct);
      if (location.zone_name) parts.push(location.zone_name);

      return parts.length > 0 ? parts.join(' - ') : 'Unknown';
    },
    []
  );

  // Memoize event handlers
  const handleSelect = useCallback(() => {
    if (interactive) onSelect();
  }, [interactive, onSelect]);

  const handleToggleDetails = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    setIsDetailsExpanded(prev => !prev);
  }, []);

  // Wrapper functions for Button onClick (Button expects () => void)
  const handleEditClick = useCallback(() => {
    onEdit();
  }, [onEdit]);

  const handleDeleteClick = useCallback(() => {
    onDelete();
  }, [onDelete]);

  return (
    <div
      className={`group relative border transition-all duration-200 overflow-hidden ${
        interactive ? 'cursor-pointer' : ''
      } ${
        isActive
          ? `${classBorderColor} bg-gradient-to-br ${classBgColor}`
          : 'border-zinc-700 bg-gradient-to-br from-zinc-800/50 to-zinc-900/30 hover:border-zinc-600 hover:bg-gradient-to-br hover:from-zinc-800/70 hover:to-zinc-900/50'
      }`}
      style={backgroundStyles}
      onClick={handleSelect}
    >
      {/* Ascendency Background Overlay */}
      {ascendencyImage && (
        <div className='absolute inset-0' style={overlayStyles} />
      )}
      {/* Header Section with Gradient Background */}
      <div className='relative p-5 pb-4' style={headerBackgroundStyles}>
        <div className='flex items-center gap-3 mb-5'>
          {/* Character Level */}
          <div
            className={`w-8 h-8 bg-gradient-to-br ${classLevelColors.bg} border ${classLevelColors.border} flex items-center justify-center flex-shrink-0`}
          >
            <span className={`text-sm font-bold ${classLevelColors.text}`}>
              {character.level}
            </span>
          </div>

          {/* Character Name */}
          <h3 className='text-xl font-bold text-white truncate flex-1'>
            {character.name}
          </h3>

          {/* Toggle Details Button */}
          {showDetails && (
            <button
              onClick={handleToggleDetails}
              className='p-1.5 bg-zinc-800/50 hover:bg-zinc-700/50 transition-colors duration-200 opacity-0 group-hover:opacity-100'
              title={isDetailsExpanded ? 'Hide details' : 'Show details'}
            >
              <svg
                className={`w-4 h-4 text-zinc-400 transition-transform duration-200 ${
                  isDetailsExpanded ? 'rotate-180' : ''
                }`}
                fill='none'
                stroke='currentColor'
                viewBox='0 0 24 24'
              >
                <path
                  strokeLinecap='round'
                  strokeLinejoin='round'
                  strokeWidth={2}
                  d='M19 9l-7 7-7-7'
                />
              </svg>
            </button>
          )}

          {/* Action Buttons */}
          {interactive && (
            <div className='flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200'>
              <Button
                onClick={handleEditClick}
                variant='outline'
                size='sm'
                className='bg-zinc-800/80 backdrop-blur-sm'
              >
                Edit
              </Button>
              <Button
                onClick={handleDeleteClick}
                variant='outline'
                size='sm'
                className='text-red-400 hover:text-red-300 hover:border-red-400 bg-zinc-800/80 backdrop-blur-sm'
              >
                Delete
              </Button>
            </div>
          )}
        </div>

        {/* Character Details */}
        <div className='flex items-center gap-6 mb-2'>
          <div className='space-y-1'>
            <div className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
              Class
            </div>
            <div className={`text-sm font-medium ${classColor}`}>
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
            <div className={`text-sm font-medium text-zinc-300`}>
              {character.solo_self_found && 'SSF '}
              {character.hardcore && 'HC '}
              {character.league}
            </div>
          </div>
        </div>
      </div>

      {/* Footer Section - only show when showDetails is true and expanded */}
      {showDetails && (
        <div
          className={`relative overflow-hidden transition-all duration-300 ease-in-out ${
            isDetailsExpanded ? 'max-h-96 opacity-100' : 'max-h-0 opacity-0'
          }`}
        >
          <div className='px-5 py-4 bg-zinc-900 border-t border-zinc-700/50'>
            <div className='grid grid-cols-1 gap-3'>
              {/* Play Time */}
              <div className='flex items-center justify-between'>
                <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                  Play Time
                </span>
                <span className='text-sm font-medium text-zinc-300'>
                  {formatDuration(character.summary?.total_play_time || 0)}
                </span>
              </div>

              {/* Deaths */}
              <div className='flex items-center justify-between'>
                <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                  Deaths
                </span>
                <span className='text-sm font-medium text-zinc-300'>
                  {character.summary?.total_deaths || 0}
                </span>
              </div>

              {/* Zones Visited */}
              <div className='flex items-center justify-between'>
                <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                  Zones Visited
                </span>
                <span className='text-sm font-medium text-zinc-300'>
                  {character.summary?.total_zones_visited || 0}
                </span>
              </div>

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

              {/* Location */}
              {currentLocation && (
                <div className='flex items-center justify-between'>
                  <span className='text-xs text-zinc-500 uppercase tracking-wide font-medium'>
                    {isActive ? 'Current Location' : 'Location'}
                  </span>
                  <span
                    className='text-sm font-medium text-zinc-300'
                    title={formatCurrentLocation(currentLocation)}
                  >
                    {formatCurrentLocation(currentLocation)}
                  </span>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
});
