import type { SceneChangeEvent } from '@/types';
import { Button } from '../button';
import { SceneEventItem } from '../log-scene-event-item';
import { logActivityLogStyles } from './log-activity-log.styles';

// Combined event type for unified display
type SceneEvent = {
  type: 'zone' | 'act' | 'hideout';
  data: SceneChangeEvent;
  timestamp: string;
};

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
