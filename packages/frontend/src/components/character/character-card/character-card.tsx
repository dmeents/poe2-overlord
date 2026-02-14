import {
  CalendarDaysIcon,
  ClockIcon,
  ExclamationTriangleIcon,
  MapIcon,
  MapPinIcon,
  XCircleIcon,
} from '@heroicons/react/24/outline';
import { memo, useCallback, useMemo } from 'react';
import type { CharacterData } from '@/types/character';
import { getAscendencyImage } from '@/utils/ascendency-assets';
import { formatDuration } from '@/utils/format-duration';
import { getDisplayAct } from '@/utils/zone-utils';
import { Button } from '../../ui/button/button';
import {
  formatDate,
  getAscendencyBackgroundStyles,
  getAscendencyOverlayStyles,
  getHardcoreAccentStyles,
  characterCardStyles as styles,
} from './character-card.styles';

function formatDeathsPerHour(deaths: number, playTimeSeconds: number): string {
  if (playTimeSeconds < 60) return '0.0'; // Not enough data
  const hours = playTimeSeconds / 3600;
  const deathsPerHour = deaths / hours;
  return deathsPerHour.toFixed(1);
}

function formatCharacterAge(createdAt: string): string {
  const created = new Date(createdAt);
  const now = new Date();
  const diffMs = now.getTime() - created.getTime();
  const days = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  if (days === 0) return 'Today';
  if (days === 1) return '1 day';
  return `${days} days`;
}

export interface CharacterCardProps {
  character: CharacterData;
  isActive: boolean;
  onSelect: () => void;
  onEdit: () => void;
  onDelete: () => void;
  interactive?: boolean;
}

export const CharacterCard = memo(function CharacterCard({
  character,
  isActive,
  onSelect,
  onEdit,
  onDelete,
  interactive = true,
}: CharacterCardProps) {
  const ascendencyImage = useMemo(
    () => getAscendencyImage(character.ascendency),
    [character.ascendency],
  );

  const backgroundStyles = useMemo(
    () => getAscendencyBackgroundStyles(ascendencyImage),
    [ascendencyImage],
  );

  const overlayStyles = useMemo(() => getAscendencyOverlayStyles(), []);

  const hardcoreStyles = useMemo(
    () => (character.hardcore ? getHardcoreAccentStyles() : undefined),
    [character.hardcore],
  );

  const locationDisplay = useMemo(() => {
    const location = character.current_location;
    if (!location) return null;

    const displayAct = getDisplayAct(location);

    if (location.zone_name) return { act: displayAct, zone: location.zone_name };
    return displayAct ? { act: displayAct, zone: null } : null;
  }, [character.current_location]);

  const handleSelect = useCallback(() => {
    if (interactive) onSelect();
  }, [interactive, onSelect]);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (interactive && (e.key === 'Enter' || e.key === ' ')) {
        e.preventDefault();
        onSelect();
      }
    },
    [interactive, onSelect],
  );

  const handleEdit = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onEdit();
    },
    [onEdit],
  );

  const handleDelete = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onDelete();
    },
    [onDelete],
  );

  const containerClasses = useMemo(() => {
    const classes: string[] = [styles.base];

    if (interactive) {
      classes.push(styles.baseInteractive, styles.baseHoverBg, styles.hoverBorder(character.class));
    }

    if (isActive) {
      classes.push(styles.activeBorder(character.class));
    }

    return classes.join(' ');
  }, [interactive, isActive, character.class]);

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: handleSelect is a no-op when interactive is false
    <div
      className={containerClasses}
      onClick={handleSelect}
      onKeyDown={handleKeyDown}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}>
      {ascendencyImage && (
        <div className={styles.ascendencyBg} style={backgroundStyles}>
          <div className={styles.ascendencyOverlay} style={overlayStyles} />
        </div>
      )}
      {hardcoreStyles && <div className={styles.ascendencyOverlay} style={hardcoreStyles} />}
      <div className={`${styles.header} ${styles.headerGradient(character.class)}`}>
        <div className={styles.headerContent}>
          <div className={`${styles.levelBadge} ${styles.levelBadgeStyles(character.class)}`}>
            <span className={styles.levelText(character.class)}>{character.level}</span>
          </div>
          <div className={styles.info}>
            <h3 className={styles.name}>{character.name}</h3>
            <div className={styles.details}>
              <span className={styles.classText(character.class)}>{character.class}</span>
              <span className={styles.separator}>/</span>
              <span className={styles.ascendency}>{character.ascendency}</span>
            </div>
            <div className={styles.leagueRow}>
              {character.hardcore && <span className={styles.hardcoreBadge}>HC</span>}
              {character.solo_self_found && <span className={styles.ssfBadge}>SSF</span>}
              <span className={styles.leagueBadge}>{character.league}</span>
            </div>
          </div>
          {interactive && (
            <div className={styles.actions}>
              <Button
                onClick={handleEdit}
                variant="outline"
                size="sm"
                className={styles.actionButton}>
                Edit
              </Button>
              <Button
                onClick={handleDelete}
                variant="outline"
                size="sm"
                className={styles.deleteButton}>
                Delete
              </Button>
            </div>
          )}
        </div>
      </div>
      <div className={styles.footer}>
        <div className={styles.statsGrid}>
          <div className={styles.stat} title="Play Time">
            <ClockIcon className={styles.statIcon} aria-hidden="true" />
            <span className={styles.statValue}>
              {formatDuration(character.summary?.total_play_time || 0)}
            </span>
          </div>
          <div className={styles.stat} title="Deaths">
            <XCircleIcon className={styles.statIcon} aria-hidden="true" />
            <span className={styles.statValue}>{character.summary?.total_deaths || 0}</span>
          </div>
          <div className={styles.stat} title="Deaths per Hour">
            <ExclamationTriangleIcon className={styles.statIcon} aria-hidden="true" />
            <span className={styles.statValue}>
              {formatDeathsPerHour(
                character.summary?.total_deaths || 0,
                character.summary?.total_play_time || 0,
              )}
              /hr
            </span>
          </div>
          <div className={styles.stat} title="Zones Visited">
            <MapIcon className={styles.statIcon} aria-hidden="true" />
            <span className={styles.statValue}>{character.summary?.total_zones_visited || 0}</span>
          </div>
          <div className={styles.stat} title="Character Age">
            <CalendarDaysIcon className={styles.statIcon} aria-hidden="true" />
            <span className={styles.statValue}>{formatCharacterAge(character.created_at)}</span>
          </div>
          {locationDisplay && (
            <div
              className={styles.stat}
              title={`${locationDisplay.act}${locationDisplay.zone ? ` - ${locationDisplay.zone}` : ''}`}>
              <MapPinIcon className={styles.statIcon} aria-hidden="true" />
              <span className={styles.statValue}>
                {locationDisplay.zone || locationDisplay.act}
              </span>
            </div>
          )}
          <div className={styles.stat} title="Last Played">
            <span className={styles.statValue}>
              {character.last_played ? formatDate(character.last_played) : 'Not Played'}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
});
