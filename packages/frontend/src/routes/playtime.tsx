import {
  ActDistributionChart,
  CharacterStatusCard,
  EmptyState,
  PageLayout,
  PlaytimeInsights,
  ZoneTracker,
} from '@/components';
import { LoadingSpinner } from '@/components/ui/loading-spinner';
import { useCharacterManagement } from '@/hooks';
import { ClockIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/playtime')({
  component: PlaytimePage,
});

function PlaytimePage() {
  const { activeCharacter, isLoading } = useCharacterManagement();
  const zones = activeCharacter?.zones || [];

  if (isLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <div className='flex items-center justify-center py-12'>
            <LoadingSpinner />
          </div>
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
