import type { SceneChangeEvent } from '@/types';
import { getSceneEventName } from '@/types';

// Combined event type for unified display
type SceneEvent = {
  type: 'zone' | 'act' | 'hideout';
  data: SceneChangeEvent;
  timestamp: string;
};

interface SceneEventItemProps {
  event: SceneEvent;
}

export function SceneEventItem({ event }: SceneEventItemProps) {
  const formatTimestamp = (timestamp: string) => {
    try {
      const date = new Date(timestamp);
      return date.toLocaleTimeString();
    } catch {
      return timestamp;
    }
  };

  if (event.type === 'zone') {
    const zoneName = getSceneEventName(event.data);
    return (
      <div className='bg-zinc-800/30 border-l-4 border-blue-500 p-2 rounded-r'>
        <div className='flex items-center justify-between'>
          <span className='text-zinc-300 font-medium text-sm'>Zone Change</span>
          <span className='text-zinc-500 text-xs'>
            {formatTimestamp(event.timestamp)}
          </span>
        </div>
        <div className='text-zinc-200 text-sm mt-1'>{zoneName}</div>
      </div>
    );
  } else {
    const actName = getSceneEventName(event.data);
    return (
      <div className='bg-zinc-800/30 border-l-4 border-blue-800 p-2 rounded-r'>
        <div className='flex items-center justify-between'>
          <span className='text-zinc-300 font-medium text-sm'>Act Change</span>
          <span className='text-zinc-500 text-xs'>
            {formatTimestamp(event.timestamp)}
          </span>
        </div>
        <div className='text-zinc-200 text-sm mt-1'>{actName}</div>
      </div>
    );
  }
}
