import { memo, useCallback, useState } from 'react';
import { HoverCard } from '@/components/ui/hover-card/hover-card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useItemByName } from '@/queries/item-data';
import type { ItemData, ModDisplay } from '@/types/item-data';
import { hideOnError } from '@/utils/image-utils';
import { itemTooltipStyles as styles } from './item-tooltip.styles';

const FLASK_TYPE_LABEL: Record<string, string> = {
  LIFE: 'Life Flask',
  MANA: 'Mana Flask',
  HYBRID: 'Hybrid Flask',
  UTILITY: 'Charm',
};

/** Returns a user-facing summary of which attribute(s) a gem scales with. */
function gemScalingAttributes(gem: NonNullable<ItemData['gem']>): string | null {
  const parts: string[] = [];
  if (gem.str_req_percent > 0) parts.push('Strength');
  if (gem.dex_req_percent > 0) parts.push('Dexterity');
  if (gem.int_req_percent > 0) parts.push('Intelligence');
  return parts.length ? parts.join(' / ') : null;
}

/** Partition soul-core mods into groups keyed by slot label, preserving order. */
function groupModsBySlot(mods: ModDisplay[]): { slot: string | null; mods: ModDisplay[] }[] {
  if (!mods.some(m => m.slot)) return [{ slot: null, mods }];
  const groups: { slot: string | null; mods: ModDisplay[] }[] = [];
  for (const mod of mods) {
    const key = mod.slot ?? null;
    const last = groups[groups.length - 1];
    if (last && last.slot === key) last.mods.push(mod);
    else groups.push({ slot: key, mods: [mod] });
  }
  return groups;
}

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
  const { weapon, defences, shield, gem, flask, currency, requirements, soul_core, essence } = itemData;

  const hasDefences =
    defences &&
    (defences.armour > 0 ||
      defences.evasion > 0 ||
      defences.energy_shield > 0 ||
      defences.ward > 0 ||
      defences.movement_speed > 0);
  const hasRequirements =
    requirements.str_req > 0 || requirements.dex_req > 0 || requirements.int_req > 0;
  const hasMeta = currency !== null || itemData.drop_level > 0 || soul_core !== null;
  const gemScaling = gem ? gemScalingAttributes(gem) : null;
  const flaskTypeLabel = flask?.flask_type ? FLASK_TYPE_LABEL[flask.flask_type] ?? null : null;
  const implicitModGroups = groupModsBySlot(itemData.implicit_mods);

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

      {/* Essence: tier, per-category guaranteed modifiers, upgrade chain */}
      {essence && (
        <div className={styles.section}>
          <div className={styles.statsGrid}>
            <span className={styles.statsLabel}>Tier</span>
            <span className={styles.statsValue}>
              {essence.tier}{essence.is_perfect ? ' (Perfect)' : ''}
            </span>
          </div>
          {essence.modifiers.length > 0 && (
            <div className="w-full flex flex-col gap-1 mt-1">
              {essence.modifiers.map((m, i) => (
                <div key={`${m.mod_id}-${i}`} className="flex flex-col">
                  {m.target_category && (
                    <div className={styles.sectionHeader}>{m.target_category}</div>
                  )}
                  <div className={styles.modLine}>{m.mod_text}</div>
                </div>
              ))}
            </div>
          )}
          {essence.upgrade_to_name && (
            <div className={`${styles.modLine} text-stone-400 italic mt-1`}>
              Upgrades into {essence.upgrade_to_name}
            </div>
          )}
        </div>
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
            {weapon.reload_time > 0 && (
              <>
                <span className={styles.statsLabel}>Reload Time</span>
                <span className={styles.statsValue}>
                  {(weapon.reload_time / 1000).toFixed(2)}s
                </span>
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
            {defences && defences.movement_speed > 0 && (
              <>
                <span className={styles.statsLabel}>Movement Speed</span>
                <span className={styles.statsValue}>+{defences.movement_speed}%</span>
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
            {gemScaling && (
              <>
                <span className={styles.statsLabel}>Scales With</span>
                <span className={styles.statsValue}>{gemScaling}</span>
              </>
            )}
          </div>
        </div>
      )}

      {/* Flask info */}
      {flask && (
        <div className={styles.section}>
          <div className={styles.statsGrid}>
            {flaskTypeLabel && (
              <>
                <span className={styles.statsLabel}>Type</span>
                <span className={styles.statsValue}>{flaskTypeLabel}</span>
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

      {/* Common metadata: stack size + drop level + soul-core required level */}
      {hasMeta && (
        <div className={styles.statsGrid}>
          {currency && (
            <>
              <span className={styles.statsLabel}>Stack size</span>
              <span className={styles.statsValue}>{currency.stack_size}</span>
            </>
          )}
          {soul_core && soul_core.required_level > 0 && (
            <>
              <span className={styles.statsLabel}>Required Level</span>
              <span className={styles.statsValue}>{soul_core.required_level}</span>
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

      {/* Implicit mods — grouped by slot for runes / soul cores */}
      {itemData.implicit_mods.length > 0 && (
        <div className={styles.section}>
          {implicitModGroups.map((group, i) => (
            <div key={group.slot ?? `group-${i}`} className={i > 0 ? 'mt-1 pt-1 border-t border-stone-700/30' : ''}>
              {group.slot && <div className={styles.sectionHeader}>{group.slot}</div>}
              {group.mods.map((mod, j) => (
                <div key={`${mod.id}-${j}`} className={styles.modLine}>
                  {mod.domain === 'BONDED' && <span className="text-bone-400 mr-1">Bonded:</span>}
                  {mod.text}
                </div>
              ))}
            </div>
          ))}
        </div>
      )}

      {/* Soul-core socket limit. Prefer the designer-authored text; fall
          back to a synthesized message from the numeric limit. */}
      {soul_core && (soul_core.limit_text || soul_core.limit_count != null) && (
        <div className={`${styles.modLine} text-center italic text-stone-400`}>
          {soul_core.limit_text ?? `Only ${soul_core.limit_count} per item`}
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
