import { ArrowTrendingUpIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { ActDistributionChart } from '@/components/charts/act-distribution-chart/act-distribution-chart';
import { LevelingInsights } from '@/components/insights/leveling-insights/leveling-insights';
import { LevelHistoryChart } from '@/components/leveling/level-history-chart/level-history-chart';
import { LevelHistoryTable } from '@/components/leveling/level-history-table/level-history-table';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { CurrentZoneCard } from '@/components/zones/current-zone-card/current-zone-card';
import { ZoneOverviewCard } from '@/components/zones/zone-overview-card/zone-overview-card';
import { useCharacter } from '@/contexts/CharacterContext';
import { useGameProcess } from '@/contexts/GameProcessContext';
import { useZone } from '@/contexts/ZoneContext';
import { useActiveLevelTime } from '@/hooks/useActiveLevelTime';
import { useLevelingStats } from '@/queries/leveling';

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

  if (isLoading) {
    return (
      <div className="min-h-screen text-stone-50">
        <div className="px-6 py-8">
          <LoadingSpinner className="py-12" />
        </div>
      </div>
    );
  }

  const allEvents = stats?.all_events ?? [];

  const leftColumn = (
    <>
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
    </>
  );

  const rightColumn = (
    <>
      {stats && (
        <LevelingInsights
          events={allEvents}
          currentLevel={stats.current_level}
          liveStats={stats}
        />
      )}
      {activeCharacter && <ZoneOverviewCard zones={zones} />}
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} showCharacterCard />;
}
