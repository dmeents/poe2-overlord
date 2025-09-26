// Log Scene Event Item Styles
// Centralized styling utilities for the SceneEventItem component

export const logSceneEventItemStyles = {
  zoneEvent: 'bg-zinc-800/30 border-l-4 border-blue-500 p-2 rounded-r',
  actEvent: 'bg-zinc-800/30 border-l-4 border-blue-800 p-2 rounded-r',
  eventHeader: 'flex items-center justify-between',
  eventType: 'text-zinc-300 font-medium text-sm',
  timestamp: 'text-zinc-500 text-xs',
  eventName: 'text-zinc-200 text-sm mt-1',
} as const;
