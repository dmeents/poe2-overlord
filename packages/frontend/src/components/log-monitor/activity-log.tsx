import type { SceneChangeEvent } from '@/types';
import { Button } from '../button';
import { SceneEventItem } from './scene-event-item';

// Combined event type for unified display
type SceneEvent = {
  type: 'zone' | 'act';
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
    <div className='bg-zinc-900/50 p-4 border border-zinc-800'>
      <div className='flex items-center justify-between mb-4'>
        <h3 className='text-lg font-semibold text-white'>Activity Log</h3>
        <Button
          onClick={onClearEvents}
          variant='outline'
          size='sm'
          disabled={sceneEvents.length === 0}
        >
          Clear Log
        </Button>
      </div>

      <div className='space-y-2 max-h-[64rem] overflow-y-auto'>
        {sceneEvents.length > 0 ? (
          sceneEvents.map((event, index) => (
            <SceneEventItem key={index} event={event} />
          ))
        ) : (
          <div className='text-center text-zinc-500 py-8'>
            {isMonitoring
              ? 'No scene changes detected yet. Changes will appear here as they occur.'
              : 'Scene monitoring is inactive. Start Path of Exile 2 to begin monitoring.'}
          </div>
        )}
      </div>
    </div>
  );
}
