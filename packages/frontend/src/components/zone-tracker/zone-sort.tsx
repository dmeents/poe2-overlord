import type { ZoneSortOption } from '@/hooks/useZoneFilters';
import { CheckIcon, ChevronDownIcon } from '@heroicons/react/24/outline';
import { memo, useEffect, useRef, useState } from 'react';
import { zoneTrackerStyles } from './zone-tracker.styles';

interface ZoneSortProps {
  sort: ZoneSortOption;
  onSortChange: (
    field: ZoneSortOption['field'],
    direction?: ZoneSortOption['direction']
  ) => void;
  onResetSort: () => void;
}

const SORT_OPTIONS: { value: ZoneSortOption['field']; label: string }[] = [
  { value: 'last_visited', label: 'Last Visited' },
  { value: 'duration', label: 'Time Spent' },
  { value: 'visits', label: 'Visit Count' },
  { value: 'deaths', label: 'Death Count' },
  { value: 'zone_level', label: 'Zone Level' },
  { value: 'location_name', label: 'Zone Name' },
  { value: 'first_visited', label: 'First Visited' },
];

export const ZoneSort = memo(function ZoneSort({
  sort,
  onSortChange,
  onResetSort,
}: ZoneSortProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsExpanded(false);
      }
    };

    if (isExpanded) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isExpanded]);

  const handleSortChange = (field: ZoneSortOption['field']) => {
    onSortChange(field);
    setIsExpanded(false);
  };

  const handleDirectionToggle = () => {
    onSortChange(sort.field, sort.direction === 'asc' ? 'desc' : 'asc');
  };

  const getCurrentSortLabel = () => {
    const option = SORT_OPTIONS.find(opt => opt.value === sort.field);
    return option ? option.label : 'Sort by...';
  };

  const getDirectionIcon = () => {
    return sort.direction === 'desc' ? '↓' : '↑';
  };

  return (
    <div className='relative' ref={dropdownRef}>
      {/* Sort Toggle Button */}
      <button
        className={zoneTrackerStyles.sortButton}
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className='text-sm font-medium'>{getCurrentSortLabel()}</span>
        <div className='flex items-center space-x-1'>
          <span className='text-xs text-zinc-400'>{getDirectionIcon()}</span>
          <ChevronDownIcon
            className={`w-4 h-4 text-zinc-400 transition-transform ${isExpanded ? 'rotate-180' : ''}`}
          />
        </div>
      </button>

      {/* Sort Content Overlay */}
      {isExpanded && (
        <div className={zoneTrackerStyles.sortDropdown}>
          <div className='flex items-center justify-between mb-3'>
            <h4 className='text-sm font-medium text-zinc-300'>Sort Options</h4>
            <button
              onClick={() => {
                onResetSort();
                setIsExpanded(false);
              }}
              className='text-xs text-zinc-400 hover:text-white transition-colors'
            >
              Reset
            </button>
          </div>

          <div className={zoneTrackerStyles.sortOptions}>
            {SORT_OPTIONS.map(option => (
              <div
                key={option.value}
                className={`${zoneTrackerStyles.sortOption} ${
                  sort.field === option.value
                    ? zoneTrackerStyles.sortOptionActive
                    : ''
                }`}
                onClick={() => handleSortChange(option.value)}
              >
                <span className={zoneTrackerStyles.sortOptionLabel}>
                  {option.label}
                </span>
                {sort.field === option.value && (
                  <CheckIcon className={zoneTrackerStyles.sortOptionIcon} />
                )}
              </div>
            ))}
          </div>

          {/* Direction Toggle */}
          <div className='mt-4 pt-4 border-t border-zinc-700'>
            <button
              onClick={handleDirectionToggle}
              className='w-full flex items-center justify-center space-x-2 px-3 py-2 bg-zinc-700/50 hover:bg-zinc-700 text-zinc-300 hover:text-white transition-colors'
            >
              <span className='text-sm'>
                {sort.direction === 'desc' ? 'Descending' : 'Ascending'}
              </span>
              <span className='text-lg'>{getDirectionIcon()}</span>
            </button>
          </div>
        </div>
      )}
    </div>
  );
});
