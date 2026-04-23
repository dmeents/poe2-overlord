import { MapPinIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { ActDistributionChart } from '@/components/charts/act-distribution-chart/act-distribution-chart';
import { PlaytimeInsights } from '@/components/insights/playtime-insights/playtime-insights';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { ZoneList } from '@/components/zones/zone-list/zone-list';
import { useCharacter } from '@/contexts/CharacterContext';
import { useZone } from '@/contexts/ZoneContext';
import {
  type ZoneFilters,
  type ZoneSortField,
  zoneListConfig,
} from '@/hooks/configs/zone-list.config';
import { useListControls } from '@/hooks/useListControls';
import type { ZoneStats } from '@/types/character';

export const Route = createFileRoute('/playtime')({
  component: PlaytimePage,
});

function PlaytimePage() {
  const { activeCharacter, isLoading } = useCharacter();
  const { allZones: zones } = useZone();

  const {
    filters,
    sort,
    updateFilter,
    updateSort,
    clearFilters,
    resetSort,
    resetAll,
    hasActiveFilters,
    activeFilterCount,
    activeChips,
    result: filteredZones,
    filteredCount,
    totalCount,
  } = useListControls(zones, zoneListConfig) as ReturnType<
    typeof useListControls<ZoneStats, ZoneFilters, ZoneSortField>
  > & {
    filters: ZoneFilters;
    updateFilter: <K extends keyof ZoneFilters>(key: K, value: ZoneFilters[K]) => void;
  };

  if (isLoading) {
    return (
      <div className="min-h-screen text-stone-50">
        <div className="px-6 py-8">
          <LoadingSpinner className="py-12" />
        </div>
      </div>
    );
  }

  const leftColumn = (
    <>
      {activeCharacter && (
        <Card title="Zone History" icon={<MapPinIcon />} accentColor="ember">
          <ZoneList
            zones={filteredZones}
            filters={filters}
            onFilterChange={updateFilter}
            onClearFilters={clearFilters}
            hasActiveFilters={hasActiveFilters}
            activeFilterCount={activeFilterCount}
            activeChips={activeChips}
            sort={sort}
            onSortChange={updateSort}
            onResetSort={resetSort}
            onResetAll={resetAll}
            filteredCount={filteredCount}
            totalCount={totalCount}
          />
        </Card>
      )}
    </>
  );

  const rightColumn = (
    <>
      <PlaytimeInsights zones={zones} />
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} showCharacterCard />;
}
