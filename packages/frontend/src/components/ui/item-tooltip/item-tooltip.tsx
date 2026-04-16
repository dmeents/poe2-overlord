import { memo, useCallback, useState } from 'react';
import { HoverCard } from '@/components/ui/hover-card/hover-card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useItemByName } from '@/queries/item-data';
import type { ItemData } from '@/types/item-data';
import { hideOnError } from '@/utils/image-utils';
import { itemTooltipStyles as styles } from './item-tooltip.styles';

interface ItemTooltipProps {
  /** Item name — used to look up game data by exact match */
  itemName: string;
  /**
   * Optional fallback image URL (e.g. from poe.ninja) shown while game data
   * loads or if the CDN image fails. When omitted the image is hidden on error.
   */
  fallbackImageUrl?: string;
  /** The trigger element (any item image or display) */
  children: React.ReactNode;
  /** Additional classes applied to the HoverCard wrapper */
  className?: string;
}

/** Returns the Tailwind text-color class matching a POE rarity frame index. */
function rarityClass(frame: number): string {
  if (frame === 3) return styles.nameUnique;
  if (frame === 2) return styles.nameRare;
  if (frame === 1) return styles.nameMagic;
  return styles.nameNormal;
}

/** Parses `[text]` / `[display|text]` wiki markup into plain/styled segments. */
function parseDescription(text: string) {
  return text.split(/(\[[^\]]+\])/g).map((seg, i) => {
    const match = seg.match(/^\[(?:[^\]|]+\|)?([^\]]+)\]$/);
    return match ? (
      <span key={i} className={styles.descriptionLink}>
        {match[1]}
      </span>
    ) : (
      seg
    );
  });
}

/**
 * Unified item tooltip — enriches any item trigger with game data on hover.
 *
 * Handles all item categories: currency, weapons, armour/shields, gems, and
 * flasks. The relevant stat sections are shown conditionally based on which
 * sub-fields are populated in the ItemData response.
 *
 * Query is deferred until first hover to avoid firing IPC calls for every item
 * on mount. React Query caches results permanently (staleTime: Infinity —
 * game data only changes between patch imports).
 *
 * @example
 * // Economy row currency icon
 * <ItemTooltip itemName={currency.name} fallbackImageUrl={currency.image_url}>
 *   <img src={currency.image_url} alt={currency.name} />
 * </ItemTooltip>
 *
 * @example
 * // Equipment display — no fallback needed
 * <ItemTooltip itemName="Warhammer">
 *   <img src={itemImageUrl} alt="Warhammer" />
 * </ItemTooltip>
 */
export const ItemTooltip = memo(function ItemTooltip({
  itemName,
  fallbackImageUrl,
  children,
  className,
}: ItemTooltipProps) {
  const [hasHovered, setHasHovered] = useState(false);

  const { data: itemData, isLoading } = useItemByName(hasHovered ? itemName : null);

  const handleOpenChange = useCallback((open: boolean) => {
    if (open) setHasHovered(true);
  }, []);

  const popupContent = isLoading ? (
    <div className="py-6">
      <LoadingSpinner />
    </div>
  ) : itemData ? (
    <ItemTooltipContent
      itemName={itemName}
      itemData={itemData}
      fallbackImageUrl={fallbackImageUrl}
    />
  ) : (
    <div className={styles.content}>
      {fallbackImageUrl && (
        <img src={fallbackImageUrl} alt={itemName} className={styles.image} onError={hideOnError} />
      )}
      <div className={`${styles.name} ${styles.nameNormal}`}>{itemName}</div>
      <div className={styles.noData}>No game data available</div>
    </div>
  );

  return (
    <HoverCard
      content={popupContent}
      showDelay={200}
      width="w-72"
      className={`flex-shrink-0${className ? ` ${className}` : ''}`}
      onOpenChange={handleOpenChange}>
      {children}
    </HoverCard>
  );
});

// ---------------------------------------------------------------------------
// Internal content component — separated to keep ItemTooltip readable.
// ---------------------------------------------------------------------------

interface ItemTooltipContentProps {
  itemName: string;
  itemData: ItemData;
  fallbackImageUrl?: string;
}

function ItemTooltipContent({ itemName, itemData, fallbackImageUrl }: ItemTooltipContentProps) {
  const { weapon, defences, shield, gem, flask, currency, requirements } = itemData;

  const hasDefences =
    defences &&
    (defences.armour > 0 ||
      defences.evasion > 0 ||
      defences.energy_shield > 0 ||
      defences.ward > 0);
  const hasRequirements =
    requirements.str_req > 0 || requirements.dex_req > 0 || requirements.int_req > 0;
  const hasMeta = currency !== null || itemData.drop_level > 0;

  return (
    <div className={styles.content}>
      {/* Image */}
      <img
        src={itemData.image_url ?? fallbackImageUrl}
        alt={itemName}
        className={styles.image}
        onError={e => {
          if (fallbackImageUrl && e.currentTarget.src !== fallbackImageUrl) {
            e.currentTarget.src = fallbackImageUrl;
          } else {
            hideOnError(e);
          }
        }}
      />

      {/* Name + category */}
      <div className={`${styles.name} ${rarityClass(itemData.rarity_frame)}`}>{itemName}</div>
      <div className={styles.categoryBadge}>{itemData.category}</div>

      {/* Currency description */}
      {currency?.description && (
        <p className={styles.description}>{parseDescription(currency.description)}</p>
      )}

      {/* Weapon stats */}
      {weapon && (
        <div className={styles.section}>
          <div className={styles.statsGrid}>
            <span className={styles.statsLabel}>Physical Damage</span>
            <span className={styles.statsValue}>
              {weapon.damage_min}–{weapon.damage_max}
            </span>
            <span className={styles.statsLabel}>Critical Strike</span>
            <span className={styles.statsValue}>{(weapon.critical / 100).toFixed(2)}%</span>
            <span className={styles.statsLabel}>Attack Speed</span>
            <span className={styles.statsValue}>{(weapon.attack_speed / 100).toFixed(2)} aps</span>
            {weapon.range_max > 0 && (
              <>
                <span className={styles.statsLabel}>Range</span>
                <span className={styles.statsValue}>{weapon.range_max}</span>
              </>
            )}
          </div>
        </div>
      )}

      {/* Armour / shield defences */}
      {(hasDefences || shield) && (
        <div className={styles.section}>
          <div className={styles.statsGrid}>
            {defences && defences.armour > 0 && (
              <>
                <span className={styles.statsLabel}>Armour</span>
                <span className={styles.statsValue}>{defences.armour}</span>
              </>
            )}
            {defences && defences.evasion > 0 && (
              <>
                <span className={styles.statsLabel}>Evasion</span>
                <span className={styles.statsValue}>{defences.evasion}</span>
              </>
            )}
            {defences && defences.energy_shield > 0 && (
              <>
                <span className={styles.statsLabel}>Energy Shield</span>
                <span className={styles.statsValue}>{defences.energy_shield}</span>
              </>
            )}
            {defences && defences.ward > 0 && (
              <>
                <span className={styles.statsLabel}>Ward</span>
                <span className={styles.statsValue}>{defences.ward}</span>
              </>
            )}
            {shield && (
              <>
                <span className={styles.statsLabel}>Block Chance</span>
                <span className={styles.statsValue}>{shield.block}%</span>
              </>
            )}
          </div>
        </div>
      )}

      {/* Gem info */}
      {gem && (
        <div className={styles.section}>
          <div className={styles.statsGrid}>
            {gem.gem_type && (
              <>
                <span className={styles.statsLabel}>Type</span>
                <span className={styles.statsValue}>{gem.gem_type}</span>
              </>
            )}
            {gem.gem_colour && (
              <>
                <span className={styles.statsLabel}>Colour</span>
                <span className={styles.statsValue}>{gem.gem_colour}</span>
              </>
            )}
            {gem.gem_min_level > 0 && (
              <>
                <span className={styles.statsLabel}>Min Level</span>
                <span className={styles.statsValue}>{gem.gem_min_level}</span>
              </>
            )}
            {gem.gem_tier !== null && (
              <>
                <span className={styles.statsLabel}>Tier</span>
                <span className={styles.statsValue}>{gem.gem_tier}</span>
              </>
            )}
          </div>
        </div>
      )}

      {/* Flask info */}
      {flask && (
        <div className={styles.section}>
          <div className={styles.statsGrid}>
            {flask.flask_type && (
              <>
                <span className={styles.statsLabel}>Type</span>
                <span className={styles.statsValue}>{flask.flask_type}</span>
              </>
            )}
            {flask.flask_life > 0 && (
              <>
                <span className={styles.statsLabel}>Life Recovery</span>
                <span className={styles.statsValue}>{flask.flask_life}</span>
              </>
            )}
            {flask.flask_mana > 0 && (
              <>
                <span className={styles.statsLabel}>Mana Recovery</span>
                <span className={styles.statsValue}>{flask.flask_mana}</span>
              </>
            )}
            {flask.flask_recovery_time > 0 && (
              <>
                <span className={styles.statsLabel}>Duration</span>
                <span className={styles.statsValue}>
                  {(flask.flask_recovery_time / 1000).toFixed(1)}s
                </span>
              </>
            )}
          </div>
        </div>
      )}

      {/* Common metadata: stack size + drop level */}
      {hasMeta && (
        <div className={styles.statsGrid}>
          {currency && (
            <>
              <span className={styles.statsLabel}>Stack size</span>
              <span className={styles.statsValue}>{currency.stack_size}</span>
            </>
          )}
          {itemData.drop_level > 0 && (
            <>
              <span className={styles.statsLabel}>Drop level</span>
              <span className={styles.statsValue}>{itemData.drop_level}</span>
            </>
          )}
        </div>
      )}

      {/* Attribute requirements */}
      {hasRequirements && (
        <div className={styles.section}>
          <div className={styles.sectionHeader}>Requirements</div>
          <div className={styles.statsGrid}>
            {requirements.str_req > 0 && (
              <>
                <span className={styles.statsLabel}>Strength</span>
                <span className={styles.statsValue}>{requirements.str_req}</span>
              </>
            )}
            {requirements.dex_req > 0 && (
              <>
                <span className={styles.statsLabel}>Dexterity</span>
                <span className={styles.statsValue}>{requirements.dex_req}</span>
              </>
            )}
            {requirements.int_req > 0 && (
              <>
                <span className={styles.statsLabel}>Intelligence</span>
                <span className={styles.statsValue}>{requirements.int_req}</span>
              </>
            )}
          </div>
        </div>
      )}

      {/* Implicit mods */}
      {itemData.implicit_mods.length > 0 && (
        <div className={styles.section}>
          {itemData.implicit_mods.map(mod => (
            <div key={mod.id} className={styles.modLine}>
              {mod.text}
            </div>
          ))}
        </div>
      )}

      {/* Explicit mods */}
      {itemData.explicit_mods.length > 0 && (
        <div className={styles.section}>
          {itemData.explicit_mods.map(mod => (
            <div key={mod.id} className={styles.modLine}>
              {mod.text}
            </div>
          ))}
        </div>
      )}

      {/* Flavour text */}
      {itemData.flavour_text && <p className={styles.flavourText}>{itemData.flavour_text}</p>}
    </div>
  );
}
