import { createFileRoute, Link } from '@tanstack/react-router';
import { Button } from '../components/button';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <div className='min-h-screen bg-zinc-900 text-white p-6'>
      <div className='max-w-6xl mx-auto'>
        <h1 className='text-4xl font-bold text-center mb-8 text-white'>
          The overlord has risen!
        </h1>

        <div className='text-center space-y-6'>
          <p className='text-xl text-zinc-300 max-w-2xl mx-auto'>
            Welcome to POE2 Overlord - your comprehensive Path of Exile 2
            monitoring and activity tracking companion.
          </p>

          <div className='flex justify-center space-x-4'>
            <Link to='/characters'>
              <Button variant='primary' size='lg'>
                Manage Characters
              </Button>
            </Link>
            <Link to='/activity'>
              <Button variant='primary' size='lg'>
                View Activity Monitor
              </Button>
            </Link>
            <Link to='/time-tracking'>
              <Button variant='primary' size='lg'>
                Time Tracking Dashboard
              </Button>
            </Link>
            <Link to='/settings'>
              <Button variant='outline' size='lg'>
                Settings
              </Button>
            </Link>
          </div>

          <div className='grid grid-cols-1 md:grid-cols-3 gap-6 mt-12 max-w-4xl mx-auto'>
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
    </div>
  );
}
