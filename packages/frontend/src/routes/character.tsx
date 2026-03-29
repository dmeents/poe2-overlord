import { ArrowTrendingUpIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { ActDistributionChart } from '@/components/charts/act-distribution-chart/act-distribution-chart';
import { LevelingInsights } from '@/components/insights/leveling-insights/leveling-insights';
import { PlaytimeInsights } from '@/components/insights/playtime-insights/playtime-insights';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { LevelHistoryChart } from '@/components/leveling/level-history-chart/level-history-chart';
import { LevelHistoryTable } from '@/components/leveling/level-history-table/level-history-table';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { CurrentZoneCard } from '@/components/zones/current-zone-card/current-zone-card';
import { ZoneList } from '@/components/zones/zone-list/zone-list';
import { useCharacter } from '@/contexts/CharacterContext';
import { useGameProcess } from '@/contexts/GameProcessContext';
import { useZone } from '@/contexts/ZoneContext';
import {
  type ZoneFilters,
  type ZoneSortField,
  zoneListConfig,
} from '@/hooks/configs/zone-list.config';
import { useActiveLevelTime } from '@/hooks/useActiveLevelTime';
import { useListControls } from '@/hooks/useListControls';
import { useLevelingStats } from '@/queries/leveling';
import type { ZoneStats } from '@/types/character';

export const Route = createFileRoute('/character')({
  component: CharacterDetailPage,
});

function CharacterDetailPage() {
  const { activeCharacter, isLoading } = useCharacter();
  const { allZones: zones } = useZone();
  const { gameRunning } = useGameProcess();
  const { data: stats } = useLevelingStats(activeCharacter?.id);
  const activeZone = zones.find(zone => zone.is_active);

  const isTimerActive =
    !!stats?.is_actively_grinding && !!stats?.last_level_reached_at && gameRunning;

  const currentTimeSeconds = useActiveLevelTime({
    lastLevelTimestamp: stats?.last_level_reached_at ?? undefined,
    isActive: isTimerActive,
    activeSecondsAtLevel: stats?.active_seconds_at_level ?? 0,
  });

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

  const allEvents = stats?.all_events ?? [];

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && activeZone && <CurrentZoneCard zone={activeZone} />}
      {activeCharacter && (
        <Card title="Level History" icon={<ArrowTrendingUpIcon />} accentColor="ember">
          <LevelHistoryChart events={allEvents} alwaysShowTime />
          <div className="border-t border-stone-800/60 mx-5" />
          <LevelHistoryTable
            events={allEvents}
            liveStats={stats}
            currentTimeSeconds={currentTimeSeconds}
          />
        </Card>
      )}
      {activeCharacter && (
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
      )}
    </>
  );

  const rightColumn = (
    <>
      {stats && (
        <LevelingInsights events={allEvents} currentLevel={stats.current_level} liveStats={stats} />
      )}
      {activeCharacter && <PlaytimeInsights zones={zones} />}
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
