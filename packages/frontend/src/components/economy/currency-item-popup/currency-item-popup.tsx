import { memo, useCallback, useState } from 'react';
import { HoverCard } from '@/components/ui/hover-card/hover-card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useItemByName } from '@/queries/item-data';
import { hideOnError } from '@/utils/image-utils';
import { currencyItemPopupStyles as styles } from './currency-item-popup.styles';

interface CurrencyItemPopupProps {
  /** Currency name — used to look up game data by exact match */
  currencyName: string;
  /** poe.ninja image URL, shown while game data loads or as fallback */
  fallbackImageUrl: string;
  /** The trigger element (currency icon image) */
  children: React.ReactNode;
}

/**
 * Hover popup that enriches an economy currency icon with game data.
 *
 * Query is deferred until first hover to avoid firing IPC calls for every row
 * on mount. After the first fetch, React Query caches the result permanently
 * (staleTime: Infinity — game data only changes between patch imports).
 *
 * Delegates all positioning, portal, and interaction logic to HoverCard.
 * onOpenChange fires immediately on hover (before the 200ms showDelay) so the
 * IPC call starts while the user waits for the card to appear.
 */
export const CurrencyItemPopup = memo(function CurrencyItemPopup({
  currencyName,
  fallbackImageUrl,
  children,
}: CurrencyItemPopupProps) {
  const [hasHovered, setHasHovered] = useState(false);

  // Only fetch when hovered for the first time. After that, the result is cached
  // in React Query and subsequent hovers are instant.
  const { data: itemData, isLoading } = useItemByName(hasHovered ? currencyName : null);

  const handleOpenChange = useCallback((open: boolean) => {
    if (open) setHasHovered(true);
  }, []);

  const popupContent = isLoading ? (
    <div className="py-6">
      <LoadingSpinner />
    </div>
  ) : itemData ? (
    <div className={styles.content}>
      <img
        src={itemData.image_url ?? fallbackImageUrl}
        alt={currencyName}
        className={styles.image}
        onError={hideOnError}
      />
      <div className={styles.name}>{currencyName}</div>
      <div className={styles.categoryBadge}>{itemData.category}</div>

      {itemData.currency?.description && (
        <p className={styles.description}>{itemData.currency.description}</p>
      )}

      <div className={styles.metaGrid}>
        {itemData.currency && (
          <>
            <span className={styles.metaLabel}>Stack size</span>
            <span className={styles.metaValue}>{itemData.currency.stack_size}</span>
          </>
        )}
        {itemData.drop_level > 0 && (
          <>
            <span className={styles.metaLabel}>Drop level</span>
            <span className={styles.metaValue}>{itemData.drop_level}</span>
          </>
        )}
      </div>

      {itemData.implicit_mods.length > 0 && (
        <div className={styles.modSection}>
          {itemData.implicit_mods.map(mod => (
            <div key={mod.id} className={styles.modLine}>
              {mod.text}
            </div>
          ))}
        </div>
      )}

      {itemData.flavour_text && <p className={styles.flavourText}>{itemData.flavour_text}</p>}
    </div>
  ) : (
    <div className={styles.content}>
      <img
        src={fallbackImageUrl}
        alt={currencyName}
        className={styles.image}
        onError={hideOnError}
      />
      <div className={styles.name}>{currencyName}</div>
      <div className={styles.noData}>No game data available</div>
    </div>
  );

  return (
    <HoverCard
      content={popupContent}
      showDelay={200}
      width="w-56"
      className="flex-shrink-0"
      onOpenChange={handleOpenChange}>
      {children}
    </HoverCard>
  );
});
