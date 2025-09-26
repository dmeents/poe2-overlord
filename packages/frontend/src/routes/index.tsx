import { createFileRoute } from '@tanstack/react-router';
import {
  ActTimeChart,
  CharacterStatusCard,
  GameStatusIndicator,
  QuickActionsPanel,
  QuickStatsGrid,
  RecentActivityPanel,
} from '../components';
import { PageHeader } from '../components/page-header';
import { useCharacterTimeTracking } from '../hooks/useCharacterTimeTracking';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { allStats } = useCharacterTimeTracking();

  return (
    <div className='min-h-screen bg-zinc-900 text-white'>
      <PageHeader
        title='POE Overlord'
        subtitle='Your comprehensive Path of Exile 2 monitoring and activity tracking companion.'
        showBackButton={false}
      />
      <div className='max-w-7xl mx-auto px-6 pb-16'>
        <div className='grid grid-cols-1 lg:grid-cols-3 gap-6'>
          <div className='lg:col-span-2 space-y-6'>
            <div className='grid grid-cols-1 md:grid-cols-2 gap-6'>
              <CharacterStatusCard />
              <GameStatusIndicator />
            </div>
            <QuickStatsGrid />
            <ActTimeChart stats={allStats} />
          </div>
          <div className='space-y-6'>
            <QuickActionsPanel />
            <RecentActivityPanel />
          </div>
        </div>
      </div>
    </div>
  );
}
