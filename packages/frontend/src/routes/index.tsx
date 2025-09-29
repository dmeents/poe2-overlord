import { createFileRoute } from '@tanstack/react-router';
import {
  ActDistributionChart,
  CharacterStatusCard,
  DashboardInsights,
  WalkthroughDashboard,
} from '../components';
import { useCharacterManagement } from '../hooks';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { activeCharacter } = useCharacterManagement();

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <div className='px-6 py-8 pb-16'>
        <div className='grid grid-cols-1 lg:grid-cols-3 gap-6'>
          {/* Left Column - Takes up 2/3 of the space */}
          <div className='lg:col-span-2 space-y-6'>
            <CharacterStatusCard />
            {activeCharacter && (
              <WalkthroughDashboard characterId={activeCharacter.id} />
            )}
          </div>

          {/* Right Column - Takes up 1/3 of the space */}
          <div className='space-y-6'>
            <DashboardInsights />
            {activeCharacter && (
              <ActDistributionChart character={activeCharacter} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
