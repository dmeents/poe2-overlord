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
import { hideOnError } from '@/utils/image-utils';
import { getDisplayAct } from '@/utils/zone-utils';
import { Modal } from '../../ui/modal/modal';
import { TimeDisplay } from '../../ui/time-display/time-display';
import { zoneDetailsModalStyles as styles } from './zone-details-modal.styles';

export function ZoneDetailsModal() {
  const { selectedZone: zone, isModalOpen, closeModal, openZone } = useZone();

  const handleWikiClick = async () => {
    if (!zone || !zone.wiki_url) return;

    try {
      await open(zone.wiki_url);
    } catch (error) {
      console.error('Failed to open wiki link:', error);
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
    if (diffMinutes < 60) return `${diffMinutes} minute${diffMinutes !== 1 ? 's' : ''} ago`;
    if (diffHours < 24) return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`;
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
      icon={<MapIcon className="w-6 h-6" />}>
      <div className={styles.container}>
        {/* Unvisited Zone Message */}
        {isUnvisitedZone && (
          <div className={styles.unvisitedContainer}>
            <div className={styles.unvisitedContent}>
              <div className={styles.unvisitedIconContainer}>
                <MapIcon className={styles.unvisitedIcon} />
              </div>
              <h3 className={styles.unvisitedTitle}>Zone Not Yet Visited</h3>
              <p className={styles.unvisitedText}>
                You haven't visited this zone yet. Once you explore it in-game, detailed statistics
                and information will be available here.
              </p>
            </div>
          </div>
        )}
        {/* Zone Image */}
        {zone.image_url && (
          <div className={styles.imageContainer}>
            <img src={zone.image_url} alt="" className={styles.image} onError={hideOnError} />
          </div>
        )}

        {/* Description */}
        {zone.description && (
          <div className={styles.section}>
            <h3 className={styles.sectionTitle}>
              <SparklesIcon className="w-4 h-4" />
              Description
            </h3>
            <p className={styles.sectionText}>{zone.description}</p>
          </div>
        )}

        {/* Zone Metadata Section - Always show */}
        <div className={styles.section}>
          <h3 className={styles.sectionTitleWithMargin}>Zone Information</h3>
          <div className={styles.grid}>
            {/* Zone Type */}
            <div>
              <span className={styles.label}>Type:</span>
              <span className={styles.value}>
                {zone.zone_type === 'Town' || zone.is_town
                  ? 'Town'
                  : zone.zone_type === 'Hideout' || zone.zone_name.toLowerCase().includes('hideout')
                    ? 'Hideout'
                    : zone.zone_type === 'Map'
                      ? 'Map'
                      : 'Zone'}
              </span>
            </div>

            {/* Act */}
            {getDisplayAct(zone) && (
              <div>
                <span className={styles.label}>Act:</span>
                <span className={styles.value}>{getDisplayAct(zone)}</span>
              </div>
            )}

            {/* Area Level */}
            {zone.area_level && (
              <div>
                <span className={styles.label}>Level:</span>
                <span className={styles.value}>{zone.area_level}</span>
              </div>
            )}

            {/* Waypoint */}
            <div>
              <span className={styles.label}>Waypoint:</span>
              <span className={styles.value}>{zone.has_waypoint ? 'Yes' : 'No'}</span>
            </div>

            {/* Wiki Link */}
            {zone.wiki_url && (
              <div className="col-span-2">
                <button type="button" onClick={handleWikiClick} className={styles.wikiButton}>
                  <ArrowTopRightOnSquareIcon className="w-4 h-4" />
                  <span>View on Wiki</span>
                </button>
              </div>
            )}
          </div>
        </div>

        {/* Player Statistics Section - Only show if visited */}
        {!isUnvisitedZone && (
          <div className={styles.section}>
            <h3 className={styles.sectionTitleWithMargin}>Your Statistics</h3>
            <div className={styles.grid}>
              {/* Time Spent */}
              <div>
                <span className={styles.label}>Time Spent:</span>
                <span className={styles.valueMono}>
                  <TimeDisplay seconds={zone.duration} showSeconds={false} />
                </span>
              </div>

              {/* Visits */}
              <div>
                <span className={styles.label}>Visits:</span>
                <span className={styles.value}>{zone.visits}</span>
              </div>

              {/* Deaths */}
              {!zone.is_town && !zone.zone_name.toLowerCase().includes('hideout') && (
                <div>
                  <span className={styles.label}>Deaths:</span>
                  <span className={zone.deaths > 0 ? styles.valueDeaths : styles.value}>
                    {zone.deaths}
                  </span>
                </div>
              )}

              {/* First Visited */}
              <div>
                <span className={styles.label}>First Visited:</span>
                <span className={styles.value}>
                  {new Date(zone.first_visited).toLocaleDateString()}
                </span>
              </div>

              {/* Last Visited */}
              <div>
                <span className={styles.label}>Last Visited:</span>
                <span className={styles.value}>{formatLastVisited(zone.last_visited)}</span>
              </div>
            </div>
          </div>
        )}

        {/* Bosses - Always show if data exists */}
        {zone.bosses && zone.bosses.length > 0 && (
          <div className={styles.sectionAlt}>
            <h3 className={styles.sectionTitleWithIcon}>
              <FlagIcon className="w-4 h-4" />
              Bosses ({zone.bosses.length})
            </h3>
            <div className={styles.tagContainer}>
              {zone.bosses.map(boss => (
                <span key={boss} className={styles.tagBoss}>
                  {boss}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* NPCs - Always show if data exists */}
        {zone.npcs && zone.npcs.length > 0 && (
          <div className={styles.section}>
            <h3 className={styles.sectionTitleWithIcon}>
              <UserGroupIcon className="w-4 h-4" />
              NPCs ({zone.npcs.length})
            </h3>
            <div className={styles.tagContainer}>
              {zone.npcs.map(npc => (
                <span key={npc} className={styles.tagNpc}>
                  {npc}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Points of Interest - Always show if data exists */}
        {zone.points_of_interest && zone.points_of_interest.length > 0 && (
          <div className={styles.sectionAlt}>
            <h3 className={styles.sectionTitleWithIcon}>
              <MapIcon className="w-4 h-4" />
              Points of Interest ({zone.points_of_interest.length})
            </h3>
            <div className={styles.tagContainer}>
              {zone.points_of_interest.map(poi => (
                <span key={poi} className={styles.tagPoi}>
                  {poi}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Connected Zones - Always show if data exists */}
        {zone.connected_zones && zone.connected_zones.length > 0 && (
          <div className={styles.section}>
            <h3 className={styles.sectionTitleWithIcon}>
              <LinkIcon className="w-4 h-4" />
              Connected Zones ({zone.connected_zones.length})
            </h3>
            <div className={styles.tagContainer}>
              {zone.connected_zones.map(connectedZone => (
                <button
                  type="button"
                  key={connectedZone}
                  onClick={() => openZone(connectedZone)}
                  className={styles.tagConnected}
                  title="Click to view zone details">
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
