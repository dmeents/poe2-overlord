import { ActDistributionChart } from '@/components/charts/act-distribution-chart/act-distribution-chart';
import { CharacterStatusCard } from '@/components/character/character-status-card/character-status-card';
import { EmptyState } from '@/components/ui/empty-state/empty-state';
import { PageLayout } from '@/components/layout/page-layout/page-layout';
import { PlaytimeInsights } from '@/components/insights/playtime-insights/playtime-insights';
import { ZoneTracker } from '@/components/zones/zone-tracker/zone-tracker';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useCharacter } from '@/contexts/CharacterContext';
import { ClockIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/playtime')({
  component: PlaytimePage,
});

function PlaytimePage() {
  const { activeCharacter, isLoading } = useCharacter();
  const zones = activeCharacter?.zones || [];

  if (isLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <LoadingSpinner className='py-12' />
        </div>
      </div>
    );
  }

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && (
        <div className='mt-6'>
          <ZoneTracker zones={zones} />
        </div>
      )}
    </>
  );

  const rightColumn = (
    <>
      <PlaytimeInsights zones={zones} />
      {!isLoading && activeCharacter && zones.length === 0 && (
        <EmptyState
          icon={<ClockIcon className='h-12 w-12' />}
          title='No Time Tracking Data'
          description={`Start playing Path of Exile 2 with ${activeCharacter?.name} to begin tracking your time in different locations.`}
        />
      )}
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
