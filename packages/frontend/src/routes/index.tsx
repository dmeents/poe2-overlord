import { createFileRoute, Link } from '@tanstack/react-router';
import { Button } from '../components/button';
import {
  ActTimeChart,
  CharacterStatusCard,
  GameStatusIndicator,
  QuickActionsPanel,
  QuickStatsGrid,
  RecentActivityPanel,
} from '../components/dashboard';
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
      <div className='max-w-7xl mx-auto px-6'>
        <div className='grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8'>
          <div className='lg:col-span-2 space-y-6'>
            <div className='grid grid-cols-1 md:grid-cols-2 gap-6'>
              <CharacterStatusCard />
              <GameStatusIndicator />
            </div>
            <ActTimeChart stats={allStats} />
            <QuickStatsGrid />
            <RecentActivityPanel />
          </div>
          <div className='space-y-6'>
            <QuickActionsPanel />
            <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
              <h3 className='text-lg font-semibold text-white mb-4'>
                Navigation
              </h3>
              <div className='space-y-3'>
                <Link to='/characters' className='block'>
                  <Button variant='outline' size='sm' className='w-full'>
                    Character Management
                  </Button>
                </Link>
                <Link to='/time-tracking' className='block'>
                  <Button variant='outline' size='sm' className='w-full'>
                    Time Tracking Dashboard
                  </Button>
                </Link>
                <Link to='/activity' className='block'>
                  <Button variant='outline' size='sm' className='w-full'>
                    Activity Monitor
                  </Button>
                </Link>
                <Link to='/settings' className='block'>
                  <Button variant='outline' size='sm' className='w-full'>
                    Settings
                  </Button>
                </Link>
              </div>
            </div>
          </div>
        </div>
        <div className='grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto'>
          <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
            <h3 className='text-lg font-semibold text-white mb-3'>
              Character Management
            </h3>
            <p className='text-zinc-300 text-sm'>
              Create and manage multiple characters with individual time
              tracking, class selection, and league support.
            </p>
          </div>

          <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
            <h3 className='text-lg font-semibold text-white mb-3'>
              Real-time Monitoring
            </h3>
            <p className='text-zinc-300 text-sm'>
              Track zone changes, act transitions, and character movements in
              real-time as you play Path of Exile 2.
            </p>
          </div>

          <div className='bg-zinc-900/50 p-6 rounded-lg border border-zinc-800'>
            <h3 className='text-lg font-semibold text-white mb-3'>
              Process Detection
            </h3>
            <p className='text-zinc-300 text-sm'>
              Automatically detect when POE2 is running and start monitoring
              your game session.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
