import { useMemo } from 'react';

/**
 * Generic filter function type
 * @template T - The type of the data item
 * @template F - The type of the filter object
 */
export type FilterFunction<T, F> = (item: T, filters: F) => boolean;

/**
 * Generic sort function type
 * @template T - The type of the data item
 * @template S - The type of the sort option object
 */
export type SortFunction<T, S> = (a: T, b: T, sort: S) => number;

/**
 * Generic summary function type
 * @template T - The type of the data item
 */
export type SummaryFunction<T> = (data: T[], filteredData: T[]) => Record<string, unknown>;

/**
 * Configuration for data filtering and sorting
 * @template T - The type of the data items
 * @template F - The type of the filter object
 * @template S - The type of the sort option object
 */
export interface DataFilteringConfig<T, F, S> {
  /** Array of data items to filter and sort */
  data: T[];
  /** Filter object */
  filters: F;
  /** Sort option object */
  sort: S;
  /** Function to filter data items */
  filterFunction: FilterFunction<T, F>;
  /** Function to sort data items */
  sortFunction: SortFunction<T, S>;
  /** Optional function to calculate summary statistics */
  summaryFunction?: SummaryFunction<T>;
}

/**
 * Generic hook for data filtering and sorting
 * 
 * This hook provides a reusable pattern for filtering and sorting data arrays
 * with memoization for performance optimization. It can replace the duplicate
 * logic in useCharacterFiltering and useZoneFiltering.
 * 
 * @template T - The type of the data items
 * @template F - The type of the filter object
 * @template S - The type of the sort option object
 * @param config - Configuration object for filtering and sorting
 * @returns Object containing filtered/sorted data and statistics
 * 
 * @example
 * ```typescript
 * interface MyData {
 *   id: string;
 *   name: string;
 *   value: number;
 * }
 * 
 * interface MyFilters {
 *   search: string;
 *   minValue: number | null;
 * }
 * 
 * interface MySort {
 *   field: 'name' | 'value';
 *   direction: 'asc' | 'desc';
 * }
 * 
 * const config: DataFilteringConfig<MyData, MyFilters, MySort> = {
 *   data: myDataArray,
 *   filters: { search: '', minValue: null },
 *   sort: { field: 'name', direction: 'asc' },
 *   filterFunction: (item, filters) => {
 *     if (filters.search && !item.name.includes(filters.search)) return false;
 *     if (filters.minValue !== null && item.value < filters.minValue) return false;
 *     return true;
 *   },
 *   sortFunction: (a, b, sort) => {
 *     let comparison = 0;
 *     if (sort.field === 'name') {
 *       comparison = a.name.localeCompare(b.name);
 *     } else if (sort.field === 'value') {
 *       comparison = a.value - b.value;
 *     }
 *     return sort.direction === 'asc' ? comparison : -comparison;
 *   }
 * };
 * 
 * const { filteredData, count, totalCount, summary } = useDataFiltering(config);
 * ```
 */
export function useDataFiltering<T, F, S>(
  config: DataFilteringConfig<T, F, S>
) {
  const { data, filters, sort, filterFunction, sortFunction, summaryFunction } = config;

  const filteredAndSortedData = useMemo(() => {
    // Filter the data
    const filtered = data.filter(item => filterFunction(item, filters));

    // Sort the filtered data
    const sorted = [...filtered].sort((a, b) => sortFunction(a, b, sort));

    return sorted;
  }, [data, filters, sort, filterFunction, sortFunction]);

  const count = filteredAndSortedData.length;
  const totalCount = data.length;

  // Calculate summary statistics if provided
  const summary = useMemo(() => {
    if (summaryFunction) {
      return summaryFunction(data, filteredAndSortedData);
    }
    return undefined;
  }, [data, filteredAndSortedData, summaryFunction]);

  return {
    filteredData: filteredAndSortedData,
    count,
    totalCount,
    summary,
  };
}

/**
 * Helper function to create common filter functions
 * @template T - The type of the data item
 * @template F - The type of the filter object
 */
export class FilterHelpers {
  /**
   * Creates a string search filter function
   * @param searchFields - Array of field names to search in
   * @param searchKey - Key in the filter object that contains the search term
   * @returns Filter function for string searching
   */
  static createStringSearchFilter<T, F extends Record<string, unknown>>(
    searchFields: (keyof T)[],
    searchKey: keyof F
  ): FilterFunction<T, F> {
    return (item, filters) => {
      const searchTerm = filters[searchKey];
      if (!searchTerm || typeof searchTerm !== 'string' || searchTerm.trim() === '') {
        return true;
      }

      const searchLower = searchTerm.toLowerCase().trim();
      return searchFields.some(field => {
        const value = item[field];
        if (typeof value === 'string') {
          return value.toLowerCase().includes(searchLower);
        }
        return false;
      });
    };
  }

  /**
   * Creates an exact match filter function
   * @param field - Field name to match
   * @param filterKey - Key in the filter object
   * @param defaultValue - Value to consider as "all" (e.g., 'All', null)
   * @returns Filter function for exact matching
   */
  static createExactMatchFilter<T, F extends Record<string, unknown>>(
    field: keyof T,
    filterKey: keyof F,
    defaultValue: string = 'All'
  ): FilterFunction<T, F> {
    return (item, filters) => {
      const filterValue = filters[filterKey];
      if (filterValue === defaultValue || filterValue === null || filterValue === undefined) {
        return true;
      }
      return item[field] === filterValue;
    };
  }

  /**
   * Creates a boolean filter function
   * @param field - Field name to check
   * @param filterKey - Key in the filter object
   * @returns Filter function for boolean matching
   */
  static createBooleanFilter<T, F extends Record<string, unknown>>(
    field: keyof T,
    filterKey: keyof F
  ): FilterFunction<T, F> {
    return (item, filters) => {
      const filterValue = filters[filterKey];
      if (filterValue === null || filterValue === undefined) {
        return true;
      }
      return item[field] === filterValue;
    };
  }

  /**
   * Creates a numeric range filter function
   * @param field - Field name to check
   * @param minKey - Key for minimum value in filter object
   * @param maxKey - Key for maximum value in filter object
   * @returns Filter function for numeric range matching
   */
  static createNumericRangeFilter<T, F extends Record<string, unknown>>(
    field: keyof T,
    minKey: keyof F,
    maxKey: keyof F
  ): FilterFunction<T, F> {
    return (item, filters) => {
      const value = item[field] as number;
      const minValue = filters[minKey] as number | null;
      const maxValue = filters[maxKey] as number | null;

      if (minValue !== null && value < minValue) return false;
      if (maxValue !== null && value > maxValue) return false;
      return true;
    };
  }

  /**
   * Creates an array inclusion filter function
   * @param field - Field name to check
   * @param filterKey - Key in the filter object that contains the array
   * @returns Filter function for array inclusion
   */
  static createArrayInclusionFilter<T, F extends Record<string, unknown>>(
    field: keyof T,
    filterKey: keyof F
  ): FilterFunction<T, F> {
    return (item, filters) => {
      const filterArray = filters[filterKey];
      if (!Array.isArray(filterArray) || filterArray.length === 0) {
        return true;
      }
      return filterArray.includes(item[field]);
    };
  }
}

/**
 * Helper function to create common sort functions
 * @template T - The type of the data item
 * @template S - The type of the sort option object
 */
export class SortHelpers {
  /**
   * Creates a string sort function
   * @param field - Field name to sort by
   * @param directionKey - Key in sort object that contains direction
   * @returns Sort function for string fields
   */
  static createStringSort<T, S extends Record<string, unknown>>(
    field: keyof T,
    directionKey: keyof S = 'direction'
  ): SortFunction<T, S> {
    return (a, b, sort) => {
      const aValue = a[field] as string;
      const bValue = b[field] as string;
      const comparison = aValue.localeCompare(bValue);
      return sort[directionKey] === 'asc' ? comparison : -comparison;
    };
  }

  /**
   * Creates a numeric sort function
   * @param field - Field name to sort by
   * @param directionKey - Key in sort object that contains direction
   * @returns Sort function for numeric fields
   */
  static createNumericSort<T, S extends Record<string, unknown>>(
    field: keyof T,
    directionKey: keyof S = 'direction'
  ): SortFunction<T, S> {
    return (a, b, sort) => {
      const aValue = a[field] as number;
      const bValue = b[field] as number;
      const comparison = aValue - bValue;
      return sort[directionKey] === 'asc' ? comparison : -comparison;
    };
  }

  /**
   * Creates a date sort function
   * @param field - Field name to sort by
   * @param directionKey - Key in sort object that contains direction
   * @returns Sort function for date fields
   */
  static createDateSort<T, S extends Record<string, unknown>>(
    field: keyof T,
    directionKey: keyof S = 'direction'
  ): SortFunction<T, S> {
    return (a, b, sort) => {
      const aValue = a[field] as string | Date;
      const bValue = b[field] as string | Date;
      const aTime = new Date(aValue).getTime();
      const bTime = new Date(bValue).getTime();
      const comparison = aTime - bTime;
      return sort[directionKey] === 'asc' ? comparison : -comparison;
    };
  }

  /**
   * Creates a conditional sort function that handles null/undefined values
   * @param field - Field name to sort by
   * @param directionKey - Key in sort object that contains direction
   * @param nullValue - Value to use for null/undefined (default: 0)
   * @returns Sort function that handles null values
   */
  static createConditionalSort<T, S extends Record<string, unknown>>(
    field: keyof T,
    directionKey: keyof S = 'direction',
    nullValue: number = 0
  ): SortFunction<T, S> {
    return (a, b, sort) => {
      const aValue = a[field] as number | null | undefined;
      const bValue = b[field] as number | null | undefined;
      const aNum = aValue ?? nullValue;
      const bNum = bValue ?? nullValue;
      const comparison = aNum - bNum;
      return sort[directionKey] === 'asc' ? comparison : -comparison;
    };
  }
}

/**
 * Helper function to create data filtering configurations
 * @template T - The type of the data items
 * @template F - The type of the filter object
 * @template S - The type of the sort option object
 * @param data - Array of data items
 * @param filters - Filter object
 * @param sort - Sort option object
 * @param filterFunction - Function to filter data items
 * @param sortFunction - Function to sort data items
 * @param summaryFunction - Optional function to calculate summary statistics
 * @returns Data filtering configuration object
 */
export function createDataFilteringConfig<T, F, S>(
  data: T[],
  filters: F,
  sort: S,
  filterFunction: FilterFunction<T, F>,
  sortFunction: SortFunction<T, S>,
  summaryFunction?: SummaryFunction<T>
): DataFilteringConfig<T, F, S> {
  return {
    data,
    filters,
    sort,
    filterFunction,
    sortFunction,
    summaryFunction,
  };
}
