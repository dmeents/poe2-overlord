import {
  ChartBarIcon,
  ClockIcon,
  Cog6ToothIcon,
  UserGroupIcon,
} from '@heroicons/react/24/outline';
import { Link } from '@tanstack/react-router';
import { Button } from '../button';
import { quickActionsPanelStyles } from './quick-actions-panel.styles';

interface QuickActionsPanelProps {
  className?: string;
}

export function QuickActionsPanel({ className = '' }: QuickActionsPanelProps) {
  return (
    <div className={`${quickActionsPanelStyles.container} ${className}`}>
      <h3 className={quickActionsPanelStyles.title}>Quick Actions</h3>

      <div className={quickActionsPanelStyles.actionsContainer}>
        {/* Primary Actions */}
        <div className={quickActionsPanelStyles.primaryActions}>
          <Link to='/characters' className='block'>
            <Button
              variant='primary'
              size='md'
              className={quickActionsPanelStyles.actionButton}
            >
              <UserGroupIcon className='w-5 h-5' />
              Character Management
            </Button>
          </Link>

          <Link to='/time-tracking' className='block'>
            <Button
              variant='secondary'
              size='md'
              className={quickActionsPanelStyles.actionButton}
            >
              <ClockIcon className='w-5 h-5' />
              Time Tracking
            </Button>
          </Link>

          <Link to='/activity' className='block'>
            <Button
              variant='secondary'
              size='md'
              className={quickActionsPanelStyles.actionButton}
            >
              <ChartBarIcon className='w-5 h-5' />
              Activity Monitor
            </Button>
          </Link>
        </div>

        {/* Secondary Actions */}
        <div className={quickActionsPanelStyles.secondaryActions}>
          <Link to='/settings' className='block'>
            <Button
              variant='outline'
              size='sm'
              className={quickActionsPanelStyles.secondaryButton}
            >
              <Cog6ToothIcon className='w-4 h-4' />
              Settings
            </Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
