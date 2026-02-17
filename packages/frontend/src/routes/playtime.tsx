import { MapPinIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { ActDistributionChart } from '@/components/charts/act-distribution-chart/act-distribution-chart';
import { PlaytimeInsights } from '@/components/insights/playtime-insights/playtime-insights';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { ZoneList } from '@/components/zones/zone-list/zone-list';
import { useCharacter } from '@/contexts/CharacterContext';
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
  const zones = activeCharacter?.zones || [];

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
      <div className="min-h-screen text-white">
        <div className="px-6 py-8">
          <LoadingSpinner className="py-12" />
        </div>
      </div>
    );
  }

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && (
        <div className="mt-6">
          <Card
            title="Zones"
            subtitle={`${filteredCount} of ${totalCount} zones`}
            icon={<MapPinIcon className="w-5 h-5" />}>
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
        </div>
      )}
    </>
  );

  const rightColumn = (
    <>
      <PlaytimeInsights zones={zones} />
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
