import { ClockIcon, ExclamationTriangleIcon, XCircleIcon } from '@heroicons/react/24/outline';
import { memo, useCallback, useMemo } from 'react';
import type { CharacterSummaryData } from '@/types/character';
import { getAscendencyImage } from '@/utils/ascendency-assets';
import { formatDurationMinutes } from '@/utils/format-duration';
import { Button } from '../../ui/button/button';
import { formatDate, characterCardStyles as styles } from './character-card.styles';

function formatDeathsPerHour(deaths: number, playTimeSeconds: number): string {
  if (playTimeSeconds < 60) return '0.0'; // Not enough data
  const hours = playTimeSeconds / 3600;
  return (deaths / hours).toFixed(1);
}

export interface CharacterCardProps {
  character: CharacterSummaryData;
  isActive: boolean;
  onSelect?: () => void;
  onEdit?: () => void;
  onDelete?: () => void;
  interactive?: boolean;
}

export const CharacterCard = memo(function CharacterCard({
  character,
  isActive,
  onSelect = () => {},
  onEdit = () => {},
  onDelete = () => {},
  interactive = true,
}: CharacterCardProps) {
  const ascendencyImage = useMemo(
    () => getAscendencyImage(character.ascendency),
    [character.ascendency],
  );

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
    if (isActive) {
      classes.push(styles.activeBorder(character.class));
    } else {
      classes.push(styles.borderDefault);
      if (interactive) classes.push(styles.hoverBorder(character.class));
    }
    if (interactive) classes.push(styles.baseInteractive, styles.baseHoverBg);
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
      {/* Class accent bar — 4px left edge in class color */}
      <div className={`absolute left-0 inset-y-0 w-1 z-10 ${styles.accentBar(character.class)}`} />

      <div className={styles.layout}>
        {/* Portrait region */}
        <div className={styles.portrait}>
          {ascendencyImage ? (
            <>
              <img src={ascendencyImage} alt="" className={styles.portraitImg} />
              {character.hardcore && <div className={styles.portraitHardcoreOverlay} />}
              <div className={styles.portraitFade} />
            </>
          ) : (
            <div className={styles.portraitFallback} />
          )}
        </div>

        {/* Identity block */}
        <div className={styles.identity}>
          <div>
            <div className={styles.nameRow}>
              <h3 className={styles.name}>{character.name}</h3>
              {(character.hardcore || character.solo_self_found) && (
                <div className={styles.badges}>
                  {character.hardcore && <span className={styles.hardcoreBadge}>HC</span>}
                  {character.solo_self_found && <span className={styles.ssfBadge}>SSF</span>}
                </div>
              )}
            </div>
            <div className={styles.classRow}>
              <span className={styles.levelText(character.class)}>{character.level}</span>
              <span className={styles.classDot}>·</span>
              <span className={styles.classText(character.class)}>{character.class}</span>
              <span className={styles.separator}>/</span>
              <span className={styles.ascendencyText}>{character.ascendency}</span>
            </div>
          </div>
          <div className={styles.bottomRow}>
            <span className={styles.leagueBadge}>{character.league}</span>
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

        {/* Stats column — 2×2 grid */}
        <div className={styles.statsColumn}>
          <div className={styles.statsGrid}>
            <div className={styles.statCell}>
              <div className={styles.statRow}>
                <ClockIcon className={styles.statIcon} aria-hidden="true" />
                <span className={styles.statValue}>
                  {formatDurationMinutes(character.summary?.total_play_time || 0)}
                </span>
              </div>
              <span className={styles.statLabel}>Play Time</span>
            </div>
            <div className={styles.statCell}>
              <div className={styles.statRow}>
                <XCircleIcon className={styles.statIcon} aria-hidden="true" />
                <span className={styles.statValue}>{character.summary?.total_deaths || 0}</span>
              </div>
              <span className={styles.statLabel}>Deaths</span>
            </div>
            <div className={styles.statCell}>
              <div className={styles.statRow}>
                <ExclamationTriangleIcon className={styles.statIcon} aria-hidden="true" />
                <span className={styles.statValue}>
                  {formatDeathsPerHour(
                    character.summary?.total_deaths || 0,
                    character.summary?.total_play_time || 0,
                  )}
                  /hr
                </span>
              </div>
              <span className={styles.statLabel}>Deaths/hr</span>
            </div>
            <div className={styles.statCell}>
              <span className={styles.statValue}>
                {character.last_played ? formatDate(character.last_played) : '—'}
              </span>
              <span className={styles.statLabel}>Last Played</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
});
