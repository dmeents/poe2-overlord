import {
  ArrowTopRightOnSquareIcon,
  FlagIcon,
  LinkIcon,
  MapIcon,
  SparklesIcon,
  UserGroupIcon,
} from '@heroicons/react/24/outline';
import { open } from '@tauri-apps/plugin-shell';
import { useZone } from '@/contexts/ZoneContext';
import { createPlaceholderZone, getDisplayAct } from '@/utils/zone-utils';
import { TimeDisplay } from '../../ui/time-display/time-display';
import { Modal } from '../../ui/modal/modal';

export function ZoneDetailsModal() {
  const {
    selectedZone: zone,
    isModalOpen,
    closeModal,
    navigateToZone,
    allZones,
  } = useZone();

  const handleWikiClick = async () => {
    if (!zone || !zone.wiki_url) return;

    try {
      await open(zone.wiki_url);
    } catch (error) {
      console.error('Failed to open wiki link:', error);
    }
  };

  const handleConnectedZoneClick = (zoneName: string) => {
    // Find the zone in the player's data
    const foundZone = allZones.find(z => z.zone_name === zoneName);

    if (foundZone) {
      // If zone found, navigate to it
      navigateToZone(foundZone);
    } else {
      // If zone not found, create a placeholder unvisited zone
      navigateToZone(createPlaceholderZone(zoneName));
    }
  };

  if (!zone) return null;

  const isUnvisitedZone = zone.visits === 0;

  const formatLastVisited = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMinutes = Math.floor(diffMs / (1000 * 60));
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffMinutes < 1) return 'Just now';
    if (diffMinutes < 60)
      return `${diffMinutes} minute${diffMinutes !== 1 ? 's' : ''} ago`;
    if (diffHours < 24)
      return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`;
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 7) return `${diffDays} days ago`;
    if (diffDays < 30)
      return `${Math.floor(diffDays / 7)} week${Math.floor(diffDays / 7) !== 1 ? 's' : ''} ago`;
    return date.toLocaleDateString();
  };

  return (
    <Modal
      isOpen={isModalOpen}
      onClose={closeModal}
      size="2xl"
      title={zone.zone_name}
      icon={<MapIcon className="w-6 h-6" />}
    >
      <div className="space-y-6">
        {/* Unvisited Zone Message */}
        {isUnvisitedZone && (
          <div className="bg-zinc-800/50 border border-zinc-700/50 p-6 text-center">
            <div className="flex flex-col items-center gap-3">
              <div className="w-12 h-12 rounded-full bg-zinc-800 flex items-center justify-center">
                <MapIcon className="w-6 h-6 text-zinc-500" />
              </div>
              <h3 className="text-lg font-medium text-zinc-300">
                Zone Not Yet Visited
              </h3>
              <p className="text-sm text-zinc-400 max-w-md">
                You haven't visited this zone yet. Once you explore it in-game,
                detailed statistics and information will be available here.
              </p>
            </div>
          </div>
        )}
        {/* Zone Image */}
        {!isUnvisitedZone && zone.image_url && (
          <div className="relative w-full h-64 overflow-hidden bg-zinc-800">
            <img
              src={zone.image_url}
              alt=""
              className="w-full h-full object-cover"
              onError={e => {
                e.currentTarget.style.display = 'none';
              }}
            />
          </div>
        )}

        {/* Description */}
        {!isUnvisitedZone && zone.description && (
          <div className="bg-zinc-800/50 p-4 border border-zinc-700/50">
            <h3 className="text-sm font-medium text-zinc-300 mb-2 flex items-center gap-2">
              <SparklesIcon className="w-4 h-4" />
              Description
            </h3>
            <p className="text-sm text-zinc-400 leading-relaxed">
              {zone.description}
            </p>
          </div>
        )}

        {/* Zone Metadata Section - Always show */}
        <div className="bg-zinc-800/50 p-4 border border-zinc-700/50">
          <h3 className="text-sm font-medium text-zinc-300 mb-3">
            Zone Information
          </h3>
          <div className="grid grid-cols-2 gap-3 text-sm">
            {/* Zone Type */}
            <div>
              <span className="text-zinc-500">Type:</span>
              <span className="ml-2 text-zinc-300">
                {zone.is_town
                  ? 'Town'
                  : zone.zone_name.toLowerCase().includes('hideout')
                    ? 'Hideout'
                    : 'Zone'}
              </span>
            </div>

            {/* Act */}
            {getDisplayAct(zone) && (
              <div>
                <span className="text-zinc-500">Act:</span>
                <span className="ml-2 text-zinc-300">
                  {getDisplayAct(zone)}
                </span>
              </div>
            )}

            {/* Area Level */}
            {zone.area_level && (
              <div>
                <span className="text-zinc-500">Level:</span>
                <span className="ml-2 text-zinc-300">{zone.area_level}</span>
              </div>
            )}

            {/* Waypoint */}
            <div>
              <span className="text-zinc-500">Waypoint:</span>
              <span className="ml-2 text-zinc-300">
                {zone.has_waypoint ? 'Yes' : 'No'}
              </span>
            </div>

            {/* Area ID */}
            {zone.area_id && (
              <div>
                <span className="text-zinc-500">Area ID:</span>
                <span className="ml-2 text-zinc-300 font-mono text-xs">
                  {zone.area_id}
                </span>
              </div>
            )}

            {/* Wiki Link */}
            {zone.wiki_url && (
              <div className="col-span-2">
                <button
                  onClick={handleWikiClick}
                  className="flex items-center gap-2 text-blue-400 hover:text-blue-300 transition-colors cursor-pointer"
                >
                  <ArrowTopRightOnSquareIcon className="w-4 h-4" />
                  <span>View on Wiki</span>
                </button>
              </div>
            )}
          </div>
        </div>

        {/* Player Statistics Section - Only show if visited */}
        {!isUnvisitedZone && (
          <div className="bg-zinc-800/50 p-4 border border-zinc-700/50">
            <h3 className="text-sm font-medium text-zinc-300 mb-3">
              Your Statistics
            </h3>
            <div className="grid grid-cols-2 gap-3 text-sm">
              {/* Time Spent */}
              <div>
                <span className="text-zinc-500">Time Spent:</span>
                <span className="ml-2 text-zinc-300 font-mono">
                  <TimeDisplay seconds={zone.duration} showSeconds={false} />
                </span>
              </div>

              {/* Visits */}
              <div>
                <span className="text-zinc-500">Visits:</span>
                <span className="ml-2 text-zinc-300">{zone.visits}</span>
              </div>

              {/* Deaths */}
              {!zone.is_town &&
                !zone.zone_name.toLowerCase().includes('hideout') && (
                  <div>
                    <span className="text-zinc-500">Deaths:</span>
                    <span
                      className={`ml-2 ${zone.deaths > 0 ? 'text-red-400' : 'text-zinc-300'}`}
                    >
                      {zone.deaths}
                    </span>
                  </div>
                )}

              {/* First Visited */}
              <div>
                <span className="text-zinc-500">First Visited:</span>
                <span className="ml-2 text-zinc-300">
                  {new Date(zone.first_visited).toLocaleDateString()}
                </span>
              </div>

              {/* Last Visited */}
              <div>
                <span className="text-zinc-500">Last Visited:</span>
                <span className="ml-2 text-zinc-300">
                  {formatLastVisited(zone.last_visited)}
                </span>
              </div>
            </div>
          </div>
        )}

        {/* Bosses - Always show if data exists */}
        {zone.bosses && zone.bosses.length > 0 && (
          <div className="bg-zinc-900/80 p-4 border border-zinc-700/50">
            <h3 className="text-sm font-medium text-zinc-300 mb-3 flex items-center gap-2">
              <FlagIcon className="w-4 h-4" />
              Bosses ({zone.bosses.length})
            </h3>
            <div className="flex flex-wrap gap-2">
              {zone.bosses.map(boss => (
                <span
                  key={boss}
                  className="px-3 py-1.5 text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/30 rounded"
                >
                  {boss}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* NPCs - Always show if data exists */}
        {zone.npcs && zone.npcs.length > 0 && (
          <div className="bg-zinc-800/50 p-4 border border-zinc-700/50">
            <h3 className="text-sm font-medium text-zinc-300 mb-3 flex items-center gap-2">
              <UserGroupIcon className="w-4 h-4" />
              NPCs ({zone.npcs.length})
            </h3>
            <div className="flex flex-wrap gap-2">
              {zone.npcs.map(npc => (
                <span
                  key={npc}
                  className="px-3 py-1.5 text-xs font-medium bg-blue-500/10 text-blue-400 border border-blue-500/30 rounded"
                >
                  {npc}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Points of Interest - Always show if data exists */}
        {zone.points_of_interest && zone.points_of_interest.length > 0 && (
          <div className="bg-zinc-900/80 p-4 border border-zinc-700/50">
            <h3 className="text-sm font-medium text-zinc-300 mb-3 flex items-center gap-2">
              <MapIcon className="w-4 h-4" />
              Points of Interest ({zone.points_of_interest.length})
            </h3>
            <div className="flex flex-wrap gap-2">
              {zone.points_of_interest.map(poi => (
                <span
                  key={poi}
                  className="px-3 py-1.5 text-xs font-medium bg-purple-500/10 text-purple-400 border border-purple-500/30 rounded"
                >
                  {poi}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Connected Zones - Always show if data exists */}
        {zone.connected_zones && zone.connected_zones.length > 0 && (
          <div className="bg-zinc-800/50 p-4 border border-zinc-700/50">
            <h3 className="text-sm font-medium text-zinc-300 mb-3 flex items-center gap-2">
              <LinkIcon className="w-4 h-4" />
              Connected Zones ({zone.connected_zones.length})
            </h3>
            <div className="flex flex-wrap gap-2">
              {zone.connected_zones.map(connectedZone => (
                <button
                  key={connectedZone}
                  onClick={() => handleConnectedZoneClick(connectedZone)}
                  className="px-3 py-1.5 text-xs font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 rounded hover:bg-emerald-500/20 hover:border-emerald-500/50 transition-colors cursor-pointer"
                  title="Click to view zone details"
                >
                  {connectedZone}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </Modal>
  );
}
