import type { SceneChangeEvent } from '@/types';
import { Button } from '../button';
import { logActivityLogStyles } from './log-activity-log.styles';

// Combined event type for unified display
type SceneEvent = {
  type: 'zone' | 'act' | 'hideout';
  data: SceneChangeEvent;
  timestamp: string;
};

// Simple inline component for scene event items
function SceneEventItem({ event }: { event: SceneEvent }) {
  const getEventIcon = (type: string) => {
    switch (type) {
      case 'zone':
        return '🏞️';
      case 'act':
        return '⚔️';
      case 'hideout':
        return '🏠';
      default:
        return '📍';
    }
  };

  const getEventColor = (type: string) => {
    switch (type) {
      case 'zone':
        return 'text-blue-400';
      case 'act':
        return 'text-purple-400';
      case 'hideout':
        return 'text-green-400';
      default:
        return 'text-gray-400';
    }
  };

  const getLocationName = (data: SceneChangeEvent) => {
    switch (data.type) {
      case 'Zone':
        return data.zone_name;
      case 'Act':
        return data.act_name;
      case 'Hideout':
        return data.hideout_name;
      default:
        return 'Unknown Location';
    }
  };

  return (
    <div className='flex items-center gap-3 p-3 bg-zinc-800/50 rounded-lg border border-zinc-700'>
      <span className='text-lg'>{getEventIcon(event.type)}</span>
      <div className='flex-1 min-w-0'>
        <div className='flex items-center gap-2'>
          <span className={`font-medium ${getEventColor(event.type)}`}>
            {getLocationName(event.data)}
          </span>
          <span className='text-xs text-zinc-500'>{event.timestamp}</span>
        </div>
        <div className='text-sm text-zinc-400'>
          {event.type.charAt(0).toUpperCase() + event.type.slice(1)} transition
        </div>
      </div>
    </div>
  );
}

interface ActivityLogProps {
  sceneEvents: SceneEvent[];
  isMonitoring: boolean;
  onClearEvents: () => void;
}

export function ActivityLog({
  sceneEvents,
  isMonitoring,
  onClearEvents,
}: ActivityLogProps) {
  return (
    <div className={logActivityLogStyles.container}>
      <div className={logActivityLogStyles.header}>
        <h3 className={logActivityLogStyles.title}>Activity Log</h3>
        <Button
          onClick={onClearEvents}
          variant='outline'
          size='sm'
          disabled={sceneEvents.length === 0}
        >
          Clear Log
        </Button>
      </div>

      <div className={logActivityLogStyles.eventsContainer}>
        {sceneEvents.length > 0 ? (
          sceneEvents.map((event, index) => (
            <SceneEventItem key={index} event={event} />
          ))
        ) : (
          <div className={logActivityLogStyles.emptyState}>
            {isMonitoring
              ? 'No scene changes detected yet. Changes will appear here as they occur.'
              : 'Scene monitoring is inactive. Start Path of Exile 2 to begin monitoring.'}
          </div>
        )}
      </div>
    </div>
  );
}
