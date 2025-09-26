import type { SceneChangeEvent } from '@/types';
import { getSceneEventName } from '@/types';
import { logSceneEventItemStyles } from './log-scene-event-item.styles';

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
      <div className={logSceneEventItemStyles.zoneEvent}>
        <div className={logSceneEventItemStyles.eventHeader}>
          <span className={logSceneEventItemStyles.eventType}>Zone Change</span>
          <span className={logSceneEventItemStyles.timestamp}>
            {formatTimestamp(event.timestamp)}
          </span>
        </div>
        <div className={logSceneEventItemStyles.eventName}>{zoneName}</div>
      </div>
    );
  } else {
    const actName = getSceneEventName(event.data);
    return (
      <div className={logSceneEventItemStyles.actEvent}>
        <div className={logSceneEventItemStyles.eventHeader}>
          <span className={logSceneEventItemStyles.eventType}>Act Change</span>
          <span className={logSceneEventItemStyles.timestamp}>
            {formatTimestamp(event.timestamp)}
          </span>
        </div>
        <div className={logSceneEventItemStyles.eventName}>{actName}</div>
      </div>
    );
  }
}
