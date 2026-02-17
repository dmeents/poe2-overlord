import { useCallback, useMemo, useState } from 'react';

/**
 * Active filter chip for display
 */
export interface ActiveChip {
  key: string;
  label: string;
  onRemove: () => void;
}

/**
 * Chip configuration for a single filter field
 */
interface ChipConfig<TFilters, K extends keyof TFilters> {
  key: K;
  label: (value: TFilters[K]) => string;
  isActive: (value: TFilters[K], defaults: TFilters) => boolean;
}

/**
 * Full configuration with both filters and sort
 */
interface ListControlsConfigWithFilters<TItem, TFilters, TSortField extends string> {
  defaultFilters: TFilters;
  defaultSort: { field: TSortField; direction: 'asc' | 'desc' };
  filterFn: (item: TItem, filters: TFilters) => boolean;
  sortFn: (a: TItem, b: TItem, sort: { field: TSortField; direction: 'asc' | 'desc' }) => number;
  chipConfigs: ChipConfig<TFilters, keyof TFilters>[];
}

/**
 * Sort-only configuration (for economy)
 */
interface ListControlsConfigSortOnly<TItem, TSortField extends string> {
  defaultSort: { field: TSortField; direction: 'asc' | 'desc' };
  sortFn: (a: TItem, b: TItem, sort: { field: TSortField; direction: 'asc' | 'desc' }) => number;
}

/**
 * Union type for configuration
 */
type ListControlsConfig<TItem, TFilters, TSortField extends string> =
  | ListControlsConfigWithFilters<TItem, TFilters, TSortField>
  | ListControlsConfigSortOnly<TItem, TSortField>;

/**
 * Full return type with filters
 */
interface ListControlsReturnWithFilters<TItem, TFilters, TSortField extends string> {
  filters: TFilters;
  sort: { field: TSortField; direction: 'asc' | 'desc' };
  updateFilter: <K extends keyof TFilters>(key: K, value: TFilters[K]) => void;
  updateSort: (field: TSortField, direction?: 'asc' | 'desc') => void;
  clearFilters: () => void;
  resetSort: () => void;
  resetAll: () => void;
  result: TItem[];
  filteredCount: number;
  totalCount: number;
  hasActiveFilters: boolean;
  activeFilterCount: number;
  activeChips: ActiveChip[];
}

/**
 * Sort-only return type (for economy)
 */
interface ListControlsReturnSortOnly<TItem, TSortField extends string> {
  sort: { field: TSortField; direction: 'asc' | 'desc' };
  updateSort: (field: TSortField, direction?: 'asc' | 'desc') => void;
  resetSort: () => void;
  result: TItem[];
  filteredCount: number;
  totalCount: number;
}

/**
 * Type guard to check if config has filters
 */
function hasFilters<TItem, TFilters, TSortField extends string>(
  config: ListControlsConfig<TItem, TFilters, TSortField>,
): config is ListControlsConfigWithFilters<TItem, TFilters, TSortField> {
  return 'defaultFilters' in config;
}

/**
 * Generic list controls hook for filtering and sorting
 *
 * @param items - Array of items to filter and sort
 * @param config - Configuration object with filter/sort logic and defaults
 * @returns Control state and computed results
 *
 * @example
 * // With filters
 * const controls = useListControls(characters, characterListConfig);
 *
 * @example
 * // Sort-only (economy)
 * const controls = useListControls(currencies, currencyListConfig);
 */
export function useListControls<TItem, TFilters, TSortField extends string>(
  items: TItem[],
  config: ListControlsConfigWithFilters<TItem, TFilters, TSortField>,
): ListControlsReturnWithFilters<TItem, TFilters, TSortField>;

export function useListControls<TItem, TSortField extends string>(
  items: TItem[],
  config: ListControlsConfigSortOnly<TItem, TSortField>,
): ListControlsReturnSortOnly<TItem, TSortField>;

export function useListControls<TItem, TFilters, TSortField extends string>(
  items: TItem[],
  config: ListControlsConfig<TItem, TFilters, TSortField>,
):
  | ListControlsReturnWithFilters<TItem, TFilters, TSortField>
  | ListControlsReturnSortOnly<TItem, TSortField> {
  const [sort, setSort] = useState(config.defaultSort);

  // Only initialize filters if config has them
  const [filters, setFilters] = useState<TFilters | undefined>(
    hasFilters(config) ? config.defaultFilters : undefined,
  );

  const updateFilter = useCallback(
    <K extends keyof TFilters>(key: K, value: TFilters[K]) => {
      if (!hasFilters(config) || !filters) return;
      setFilters(prev => (prev ? { ...prev, [key]: value } : prev));
    },
    [config, filters],
  );

  const updateSort = useCallback((field: TSortField, direction?: 'asc' | 'desc') => {
    setSort(prev => ({
      field,
      direction: direction ?? (prev.field === field && prev.direction === 'desc' ? 'asc' : 'desc'),
    }));
  }, []);

  const clearFilters = useCallback(() => {
    if (hasFilters(config)) {
      setFilters(config.defaultFilters);
    }
  }, [config]);

  const resetSort = useCallback(() => {
    setSort(config.defaultSort);
  }, [config]);

  const resetAll = useCallback(() => {
    if (hasFilters(config)) {
      setFilters(config.defaultFilters);
    }
    setSort(config.defaultSort);
  }, [config]);

  const { result, filteredCount, hasActiveFilters, activeFilterCount, activeChips } =
    useMemo(() => {
      // Apply filters if config has them
      let filtered = items;
      if (hasFilters(config) && filters) {
        filtered = items.filter(item => config.filterFn(item, filters));
      }

      // Sort the filtered items
      const sorted = [...filtered];
      sorted.sort((a, b) => config.sortFn(a, b, sort));

      // Calculate filter stats if config has filters
      let hasActive = false;
      let activeCount = 0;
      let chips: ActiveChip[] = [];

      if (hasFilters(config) && filters) {
        // Generate active chips from chip configs
        chips = config.chipConfigs
          .filter(chipConfig => chipConfig.isActive(filters[chipConfig.key], config.defaultFilters))
          .map(chipConfig => ({
            key: String(chipConfig.key),
            label: chipConfig.label(filters[chipConfig.key]),
            onRemove: () => updateFilter(chipConfig.key, config.defaultFilters[chipConfig.key]),
          }));

        hasActive = chips.length > 0;
        activeCount = chips.length;
      }

      return {
        result: sorted,
        filteredCount: sorted.length,
        hasActiveFilters: hasActive,
        activeFilterCount: activeCount,
        activeChips: chips,
      };
    }, [items, config, filters, sort, updateFilter]);

  // Return appropriate shape based on config type
  if (hasFilters(config) && filters) {
    return {
      filters,
      sort,
      updateFilter,
      updateSort,
      clearFilters,
      resetSort,
      resetAll,
      result,
      filteredCount,
      totalCount: items.length,
      hasActiveFilters,
      activeFilterCount,
      activeChips,
    };
  }

  // Sort-only return
  return {
    sort,
    updateSort,
    resetSort,
    result,
    filteredCount: result.length,
    totalCount: items.length,
  };
}
