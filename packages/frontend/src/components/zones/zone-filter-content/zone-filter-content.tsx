import { memo } from 'react';
import type { ZoneFilters } from '../../../hooks/configs/zone-list.config';
import { Select } from '../../forms/form-select/form-select';

interface ZoneFilterContentProps {
  filters: ZoneFilters;
  onFilterChange: <K extends keyof ZoneFilters>(key: K, value: ZoneFilters[K]) => void;
}

const ACT_OPTIONS = [
  { value: 'All', label: 'All Acts' },
  { value: 'Act 1', label: 'Act 1' },
  { value: 'Act 2', label: 'Act 2' },
  { value: 'Act 3', label: 'Act 3' },
  { value: 'Interlude', label: 'Interlude' },
  { value: 'Endgame', label: 'Endgame' },
];

/**
 * Zone-specific filter controls for use in ListControlBar popover
 */
export const ZoneFilterContent = memo(function ZoneFilterContent({
  filters,
  onFilterChange,
}: ZoneFilterContentProps) {
  return (
    <div className="space-y-2.5 min-w-[280px]">
      <Select
        id="act-filter"
        value={filters.act}
        onChange={(value: string) => onFilterChange('act', value)}
        options={ACT_OPTIONS}
        variant="dropdown"
        label="Act"
      />

      <Select
        id="town-filter"
        value={filters.isTown === null ? 'all' : filters.isTown ? 'towns' : 'zones'}
        onChange={(value: string) => {
          const newValue = value === 'all' ? null : value === 'towns';
          onFilterChange('isTown', newValue);
        }}
        options={[
          { value: 'all', label: 'All' },
          { value: 'towns', label: 'Towns Only' },
          { value: 'zones', label: 'Zones Only' },
        ]}
        variant="dropdown"
        label="Location Type"
      />

      <Select
        id="active-filter"
        value={filters.isActive === null ? 'all' : filters.isActive ? 'active' : 'inactive'}
        onChange={(value: string) => {
          const newValue = value === 'all' ? null : value === 'active';
          onFilterChange('isActive', newValue);
        }}
        options={[
          { value: 'all', label: 'All' },
          { value: 'active', label: 'Active Only' },
          { value: 'inactive', label: 'Inactive Only' },
        ]}
        variant="dropdown"
        label="Status"
      />
    </div>
  );
});
