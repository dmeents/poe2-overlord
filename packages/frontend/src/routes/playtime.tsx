import {
  ActDistributionChart,
  CharacterStatusCard,
  PlaytimeInsights,
  ZoneTracker,
} from '@/components';
import { LoadingSpinner } from '@/components/loading-spinner';
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

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <div className='px-6 py-8 pb-16'>
        <div className='grid grid-cols-1 lg:grid-cols-3 gap-6'>
          {/* Left Column - Active Character Card */}
          <div className='lg:col-span-2'>
            <CharacterStatusCard />
            {activeCharacter && (
              <div className='mt-6'>
                <ZoneTracker zones={zones} />
              </div>
            )}
          </div>

          {/* Right Column - Time Tracking Content */}
          <div className='lg:col-span-1 space-y-6'>
            <PlaytimeInsights zones={zones} />
            {!isLoading && activeCharacter && zones.length === 0 && (
              <div className='text-center py-12'>
                <div className='text-zinc-500 mb-4'>
                  <ClockIcon className='mx-auto h-12 w-12' />
                </div>
                <h3 className='text-lg font-medium text-zinc-300 mb-2'>
                  No Time Tracking Data
                </h3>
                <p className='text-zinc-500'>
                  Start playing Path of Exile 2 with {activeCharacter?.name} to
                  begin tracking your time in different locations.
                </p>
              </div>
            )}
            {activeCharacter && (
              <ActDistributionChart character={activeCharacter} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
