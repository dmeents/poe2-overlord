import type { ZoneSortOption } from '@/hooks/useZoneFilters';
import { memo } from 'react';
import { SortSelect } from '../';

interface ZoneSortProps {
  sort: ZoneSortOption;
  onSortChange: (
    field: ZoneSortOption['field'],
    direction?: ZoneSortOption['direction']
  ) => void;
  onResetSort: () => void;
}

const SORT_OPTIONS = [
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
  const handleChange = (field: string, direction?: 'asc' | 'desc') => {
    onSortChange(field as ZoneSortOption['field'], direction);
  };

  return (
    <SortSelect
      id='zone-sort'
      value={sort.field}
      direction={sort.direction}
      onChange={handleChange}
      onReset={onResetSort}
      options={SORT_OPTIONS}
    />
  );
});
