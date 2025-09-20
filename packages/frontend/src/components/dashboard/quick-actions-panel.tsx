import { Link } from '@tanstack/react-router';
import { 
  UserGroupIcon, 
  ClockIcon, 
  ChartBarIcon, 
  Cog6ToothIcon 
} from '@heroicons/react/24/outline';
import { Button } from '../button';

interface QuickActionsPanelProps {
  className?: string;
}

export function QuickActionsPanel({ className = '' }: QuickActionsPanelProps) {
  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 ${className}`}
    >
      <h3 className='text-lg font-semibold text-white mb-6'>Quick Actions</h3>

      <div className='space-y-4'>
        {/* Primary Actions */}
        <div className='space-y-3'>
          <Link to='/characters' className='block'>
            <Button variant='primary' size='md' className='w-full justify-start gap-3'>
              <UserGroupIcon className='w-5 h-5' />
              Character Management
            </Button>
          </Link>

          <Link to='/time-tracking' className='block'>
            <Button variant='secondary' size='md' className='w-full justify-start gap-3'>
              <ClockIcon className='w-5 h-5' />
              Time Tracking
            </Button>
          </Link>

          <Link to='/activity' className='block'>
            <Button variant='secondary' size='md' className='w-full justify-start gap-3'>
              <ChartBarIcon className='w-5 h-5' />
              Activity Monitor
            </Button>
          </Link>
        </div>

        {/* Secondary Actions */}
        <div className='pt-4 border-t border-zinc-700'>
          <Link to='/settings' className='block'>
            <Button variant='outline' size='sm' className='w-full justify-start gap-2'>
              <Cog6ToothIcon className='w-4 h-4' />
              Settings
            </Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
