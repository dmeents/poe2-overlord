import { ChevronDown } from 'lucide-react';
import { memo, useEffect, useRef, useState } from 'react';
import { useDropdownPosition } from '../../../hooks/useDropdownPosition';
import type { ActiveChip } from '../../../hooks/useListControls';
import { Input } from '../../forms/form-input/form-input';
import { SortSelect } from '../../forms/form-sort-select/form-sort-select';
import { Button } from '../../ui/button/button';
import { DropdownPortal } from '../../ui/dropdown-portal/dropdown-portal';
import { ActiveFilterChips } from '../active-filter-chips/active-filter-chips';
import {
  chipRowClasses,
  controlBarClasses,
  countBadgeClasses,
  filterBadgeClasses,
  filterButtonActiveClasses,
  filterButtonClasses,
  filterPopoverClasses,
  filterPopoverContent,
  filterPopoverHeader,
  filterPopoverTitle,
  resetButtonClasses,
  searchInputClasses,
  sortSelectWrapperClasses,
} from './list-control-bar.styles';

interface ListControlBarProps {
  // Search
  searchValue: string;
  onSearchChange: (value: string) => void;
  searchPlaceholder?: string;
  searchDebounceMs?: number;
  isSearching?: boolean;

  // Filters
  filterContent?: React.ReactNode;
  activeFilterCount?: number;
  hasActiveFilters?: boolean;
  onClearFilters?: () => void;

  // Sort
  sortField: string;
  sortDirection: 'asc' | 'desc';
  sortOptions: { value: string; label: string }[];
  onSortChange: (field: string, direction?: 'asc' | 'desc') => void;
  onResetSort: () => void;

  // Counts
  filteredCount: number;
  totalCount: number;
  countLabel?: string;

  // Active chips
  activeChips?: ActiveChip[];
  onResetAll?: () => void;
}

/**
 * Unified list control bar for search, filter, and sort
 *
 * @example
 * <ListControlBar
 *   searchValue={filters.nameSearch}
 *   onSearchChange={(value) => updateFilter('nameSearch', value)}
 *   searchPlaceholder="Search characters..."
 *   filterContent={<CharacterFilterContent />}
 *   activeFilterCount={activeFilterCount}
 *   hasActiveFilters={hasActiveFilters}
 *   onClearFilters={clearFilters}
 *   sortField={sort.field}
 *   sortDirection={sort.direction}
 *   sortOptions={SORT_OPTIONS}
 *   onSortChange={updateSort}
 *   onResetSort={resetSort}
 *   filteredCount={filteredCount}
 *   totalCount={totalCount}
 *   countLabel="characters"
 *   activeChips={activeChips}
 *   onResetAll={resetAll}
 * />
 */
export const ListControlBar = memo(function ListControlBar({
  searchValue,
  onSearchChange,
  searchPlaceholder = 'Search...',
  searchDebounceMs = 0,
  isSearching = false,
  filterContent,
  activeFilterCount = 0,
  hasActiveFilters = false,
  onClearFilters,
  sortField,
  sortDirection,
  sortOptions,
  onSortChange,
  onResetSort,
  filteredCount,
  totalCount,
  countLabel = 'items',
  activeChips = [],
  onResetAll,
}: ListControlBarProps) {
  // Search debouncing
  const [displayValue, setDisplayValue] = useState(searchValue);
  const debounceTimerRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    setDisplayValue(searchValue);
  }, [searchValue]);

  const handleSearchChange = (value: string) => {
    setDisplayValue(value);

    if (debounceTimerRef.current) {
      clearTimeout(debounceTimerRef.current);
    }

    if (searchDebounceMs > 0) {
      debounceTimerRef.current = setTimeout(() => {
        onSearchChange(value);
      }, searchDebounceMs);
    } else {
      onSearchChange(value);
    }
  };

  useEffect(() => {
    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, []);

  // Filter popover
  const [isFilterOpen, setIsFilterOpen] = useState(false);
  const { dropdownRef, triggerRef, dropdownPosition } = useDropdownPosition({
    isOpen: isFilterOpen,
    onClose: () => setIsFilterOpen(false),
    includeWidth: false,
  });

  // Show reset button if filters or sort differ from defaults
  const showResetButton = hasActiveFilters || onResetAll;

  return (
    <>
      {/* Main control bar */}
      <div className={controlBarClasses}>
        {/* Search input */}
        <div className={searchInputClasses}>
          <Input
            id="list-search"
            value={displayValue}
            onChange={handleSearchChange}
            type="search"
            placeholder={searchPlaceholder}
            disabled={isSearching}
          />
        </div>

        {/* Filter button (optional) */}
        {filterContent && (
          <button
            ref={triggerRef as React.RefObject<HTMLButtonElement>}
            type="button"
            onClick={() => setIsFilterOpen(!isFilterOpen)}
            className={hasActiveFilters ? filterButtonActiveClasses : filterButtonClasses}>
            Filters
            {activeFilterCount > 0 && (
              <span className={filterBadgeClasses}>{activeFilterCount}</span>
            )}
            <ChevronDown className="w-4 h-4 ml-1 inline" />
          </button>
        )}

        {/* Sort select */}
        <div className={sortSelectWrapperClasses}>
          <SortSelect
            id="list-sort"
            value={sortField}
            direction={sortDirection}
            onChange={onSortChange}
            onReset={onResetSort}
            options={sortOptions}
          />
        </div>

        {/* Reset button */}
        {showResetButton && (
          <Button
            onClick={onResetAll || onClearFilters}
            variant="outline"
            size="sm"
            className={resetButtonClasses}>
            Reset
          </Button>
        )}

        {/* Count badge */}
        <span className={countBadgeClasses}>
          {filteredCount === totalCount
            ? `${totalCount} ${countLabel}`
            : `${filteredCount} of ${totalCount}`}
        </span>
      </div>

      {/* Filter popover - rendered in portal to avoid transform-based positioning issues */}
      {filterContent && (
        <DropdownPortal
          isOpen={isFilterOpen}
          dropdownRef={dropdownRef}
          position={dropdownPosition}
          className={filterPopoverClasses}
          style={{ minWidth: '300px' }}>
          <div className={filterPopoverHeader}>
            <h4 className={filterPopoverTitle}>Filter Options</h4>
          </div>
          <div className={filterPopoverContent}>{filterContent}</div>
        </DropdownPortal>
      )}

      {/* Active chips row */}
      {activeChips.length > 0 && (
        <div className={chipRowClasses}>
          <ActiveFilterChips chips={activeChips} onClearAll={onClearFilters} />
        </div>
      )}
    </>
  );
});
