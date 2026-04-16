import { useRef, useState } from 'react';
import { createPortal } from 'react-dom';
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
 */
export function CurrencyItemPopup({
  currencyName,
  fallbackImageUrl,
  children,
}: CurrencyItemPopupProps) {
  const [isVisible, setIsVisible] = useState(false);
  const [hasHovered, setHasHovered] = useState(false);
  const [position, setPosition] = useState<{ top: number; left: number } | null>(null);
  const triggerRef = useRef<HTMLDivElement>(null);
  const showTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Only fetch when hovered for the first time. After that, the result is cached
  // in React Query and subsequent hovers are instant.
  const { data: itemData, isLoading } = useItemByName(hasHovered ? currencyName : null);

  const handleMouseEnter = () => {
    if (!triggerRef.current) return;
    const rect = triggerRef.current.getBoundingClientRect();
    setPosition({
      top: rect.top,
      left: rect.left + rect.width / 2,
    });
    setHasHovered(true);
    showTimerRef.current = setTimeout(() => setIsVisible(true), 200);
  };

  const handleMouseLeave = () => {
    if (showTimerRef.current) {
      clearTimeout(showTimerRef.current);
      showTimerRef.current = null;
    }
    setIsVisible(false);
  };

  const popupNode =
    isVisible && position ? (
      <div
        role="tooltip"
        className={styles.popup}
        style={{
          position: 'fixed',
          top: `${position.top}px`,
          left: `${position.left}px`,
          transform: 'translate(-50%, calc(-100% - 8px))',
        }}>
        {isLoading ? (
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
        )}
        <div className={styles.arrow} />
      </div>
    ) : null;

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: hover trigger for informational popup
    <div
      ref={triggerRef}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      className={styles.trigger}>
      {children}
      {typeof document !== 'undefined' && createPortal(popupNode, document.body)}
    </div>
  );
}
