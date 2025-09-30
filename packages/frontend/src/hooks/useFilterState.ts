import { useCallback, useState } from 'react';

/**
 * Generic interface for filter state management
 * @template T - The type of the filter object
 * @template S - The type of the sort option object
 */
export interface FilterStateConfig<T, S> {
  /** Default filter values */
  defaultFilters: T;
  /** Default sort option */
  defaultSort: S;
  /** Function to check if filters are active */
  hasActiveFiltersFn?: (filters: T) => boolean;
}

/**
 * Generic hook for managing filter and sort state
 *
 * This hook provides a reusable pattern for managing filter state across different
 * components, replacing the duplicate logic in useCharacterFilters and useZoneFilters.
 *
 * @template T - The type of the filter object
 * @template S - The type of the sort option object
 * @param config - Configuration object containing defaults and custom logic
 * @returns Object containing filter state and management functions
 *
 * @example
 * ```typescript
 * interface MyFilters {
 *   search: string;
 *   category: string;
 *   active: boolean | null;
 * }
 *
 * interface MySort {
 *   field: 'name' | 'date';
 *   direction: 'asc' | 'desc';
 * }
 *
 * const config: FilterStateConfig<MyFilters, MySort> = {
 *   defaultFilters: { search: '', category: 'All', active: null },
 *   defaultSort: { field: 'name', direction: 'asc' },
 *   hasActiveFiltersFn: (filters) =>
 *     filters.search !== '' || filters.category !== 'All' || filters.active !== null
 * };
 *
 * const { filters, sort, updateFilter, updateSort, clearFilters, resetSort, hasActiveFilters } =
 *   useFilterState(config);
 * ```
 */
export function useFilterState<
  T extends Record<string, unknown>,
  S extends Record<string, unknown>,
>(config: FilterStateConfig<T, S>) {
  const { defaultFilters, defaultSort, hasActiveFiltersFn } = config;

  const [filters, setFilters] = useState<T>(defaultFilters);
  const [sort, setSort] = useState<S>(defaultSort);

  /**
   * Update a specific filter value
   * @param key - The key of the filter to update
   * @param value - The new value for the filter
   */
  const updateFilter = useCallback(<K extends keyof T>(key: K, value: T[K]) => {
    setFilters(prev => ({ ...prev, [key]: value }));
  }, []);

  /**
   * Update sort configuration
   * @param field - The field to sort by
   * @param direction - Optional direction, defaults to toggling current direction
   */
  const updateSort = useCallback(
    (field: S['field'], direction?: S['direction']) => {
      setSort(prev => ({
        ...prev,
        field,
        direction:
          direction ??
          (prev.field === field && prev.direction === 'desc' ? 'asc' : 'desc'),
      }));
    },
    []
  );

  /**
   * Reset all filters to their default values
   */
  const clearFilters = useCallback(() => {
    setFilters(defaultFilters);
  }, [defaultFilters]);

  /**
   * Reset sort to default values
   */
  const resetSort = useCallback(() => {
    setSort(defaultSort);
  }, [defaultSort]);

  /**
   * Check if any filters are currently active
   * Uses custom function if provided, otherwise uses default logic
   */
  const hasActiveFilters = useCallback(() => {
    if (hasActiveFiltersFn) {
      return hasActiveFiltersFn(filters);
    }

    // Default logic: check if any filter differs from its default value
    return Object.keys(filters).some(key => {
      const currentValue = filters[key];
      const defaultValue = defaultFilters[key];

      // Handle arrays
      if (Array.isArray(currentValue) && Array.isArray(defaultValue)) {
        return currentValue.length !== defaultValue.length;
      }

      // Handle null/undefined comparison
      if (currentValue === null || currentValue === undefined) {
        return defaultValue !== null && defaultValue !== undefined;
      }

      // Handle string comparison (trim for search fields)
      if (
        typeof currentValue === 'string' &&
        typeof defaultValue === 'string'
      ) {
        return currentValue.trim() !== defaultValue.trim();
      }

      // Default comparison
      return currentValue !== defaultValue;
    });
  }, [filters, defaultFilters, hasActiveFiltersFn]);

  return {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    hasActiveFilters: hasActiveFilters(),
  };
}

/**
 * Type helper for creating filter state configurations
 * @template T - The type of the filter object
 * @template S - The type of the sort option object
 */
export type CreateFilterStateConfig<T, S> = (
  defaultFilters: T,
  defaultSort: S,
  hasActiveFiltersFn?: (filters: T) => boolean
) => FilterStateConfig<T, S>;

/**
 * Helper function to create filter state configurations
 * @param defaultFilters - Default filter values
 * @param defaultSort - Default sort option
 * @param hasActiveFiltersFn - Optional custom function to check active filters
 * @returns Filter state configuration object
 */
export const createFilterStateConfig: CreateFilterStateConfig<
  Record<string, unknown>,
  Record<string, unknown>
> = (defaultFilters, defaultSort, hasActiveFiltersFn) => ({
  defaultFilters,
  defaultSort,
  hasActiveFiltersFn,
});
