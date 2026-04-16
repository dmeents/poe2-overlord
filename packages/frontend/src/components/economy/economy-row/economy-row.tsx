import { ArrowsRightLeftIcon, StarIcon as StarOutlineIcon } from '@heroicons/react/24/outline';
import { StarIcon as StarSolidIcon } from '@heroicons/react/24/solid';
import { HoverCard } from '@/components/ui/hover-card/hover-card';
import { ItemTooltip } from '@/components/ui/item-tooltip/item-tooltip';
import { useEconomy } from '@/contexts/EconomyContext';
import type { BackendEconomyType, CurrencyExchangeRate, EconomyType } from '@/types/economy';
import { calculateItemsSoldPerHour } from '@/utils/economy-utils';
import { hideOnError } from '@/utils/image-utils';
import { economyRowStyles } from './economy-row.styles';

interface EconomyRowProps {
  currency: CurrencyExchangeRate;
  economyType?: EconomyType;
  isStarred?: boolean;
  onToggleStar?: (currencyId: string, economyType: BackendEconomyType) => void;
  onClick?: (currency: CurrencyExchangeRate) => void;
}

export function EconomyRow({
  currency,
  economyType,
  isStarred,
  onToggleStar,
  onClick,
}: EconomyRowProps) {
  const { display_value } = currency;
  const { currencyData } = useEconomy();

  const handleStarClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (onToggleStar && economyType && economyType !== 'All') {
      onToggleStar(currency.id, economyType as BackendEconomyType);
    }
  };

  const handleStarKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      e.stopPropagation();
      if (onToggleStar && economyType && economyType !== 'All') {
        onToggleStar(currency.id, economyType as BackendEconomyType);
      }
    }
  };

  const handleClick = () => {
    if (onClick) {
      onClick(currency);
    }
  };

  const formattedValue = display_value.value.toLocaleString('en-US', {
    minimumFractionDigits: display_value.value >= 10 ? 1 : 2,
    maximumFractionDigits: display_value.value >= 10 ? 1 : 2,
  });

  const tooltipContent = currencyData ? (
    <div className={economyRowStyles.tooltipContainer}>
      <div className={economyRowStyles.tooltipHeader}>Raw Values</div>
      <div className={economyRowStyles.tooltipRow}>
        <span className={economyRowStyles.tooltipLabel}>{currencyData.primary_currency.name}:</span>
        <span className={economyRowStyles.tooltipValue}>{currency.primary_value.toFixed(6)}</span>
      </div>
      <div className={economyRowStyles.tooltipRow}>
        <span className={economyRowStyles.tooltipLabel}>
          {currencyData.secondary_currency.name}:
        </span>
        <span className={economyRowStyles.tooltipValue}>{currency.secondary_value.toFixed(4)}</span>
      </div>
      {currencyData.tertiary_currency && (
        <div className={economyRowStyles.tooltipRow}>
          <span className={economyRowStyles.tooltipLabel}>
            {currencyData.tertiary_currency.name}:
          </span>
          <span className={economyRowStyles.tooltipValue}>
            {currency.tertiary_value.toFixed(2)}
          </span>
        </div>
      )}
    </div>
  ) : null;

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (onClick && (e.key === 'Enter' || e.key === ' ')) {
      e.preventDefault();
      onClick(currency);
    }
  };

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: handleClick is a no-op when onClick is undefined
    <div
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
      className={`${economyRowStyles.container} ${onClick ? economyRowStyles.containerClickable : ''}`}>
      <div className={economyRowStyles.leftSection}>
        {onToggleStar && economyType && economyType !== 'All' && (
          <button
            type="button"
            tabIndex={0}
            onClick={handleStarClick}
            onKeyDown={handleStarKeyDown}
            title={isStarred ? 'Unstar currency' : 'Star currency'}
            className={`${economyRowStyles.starButton} ${isStarred ? economyRowStyles.starActive : economyRowStyles.starInactive}`}>
            {isStarred ? (
              <StarSolidIcon className="w-4 h-4" />
            ) : (
              <StarOutlineIcon className="w-4 h-4" />
            )}
          </button>
        )}
        <ItemTooltip itemName={currency.name} fallbackImageUrl={currency.image_url}>
          <img
            src={currency.image_url}
            alt={currency.name}
            className={economyRowStyles.image}
            onError={hideOnError}
          />
        </ItemTooltip>
        <div className={economyRowStyles.nameContainer}>
          <div className={economyRowStyles.name}>{currency.name}</div>
          <div className={economyRowStyles.statsRow}>
            {currency.volume !== null && (
              <span title="Number of items sold per hour">
                {calculateItemsSoldPerHour(currency.volume, currency.primary_value)} / hr
              </span>
            )}
          </div>
        </div>
      </div>

      <HoverCard content={tooltipContent}>
        <div className={economyRowStyles.valueContainer}>
          <div className={economyRowStyles.valueRow}>
            {display_value.inverted ? (
              <>
                <span>{formattedValue}</span>
                <img
                  src={currency.image_url}
                  alt={currency.name}
                  className={economyRowStyles.valueIcon}
                  title={currency.name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
                <ArrowsRightLeftIcon className={economyRowStyles.exchangeIcon} />
                <span>1</span>
                <img
                  src={display_value.currency_image_url}
                  alt={display_value.currency_name}
                  className={economyRowStyles.valueIcon}
                  title={display_value.currency_name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
              </>
            ) : (
              <>
                <span>{formattedValue}</span>
                <img
                  src={display_value.currency_image_url}
                  alt={display_value.currency_name}
                  className={economyRowStyles.valueIcon}
                  title={display_value.currency_name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
                <ArrowsRightLeftIcon className={economyRowStyles.exchangeIcon} />
                <span>1</span>
                <img
                  src={currency.image_url}
                  alt={currency.name}
                  className={economyRowStyles.valueIcon}
                  title={currency.name}
                  onError={e => {
                    e.currentTarget.style.display = 'none';
                  }}
                />
              </>
            )}
          </div>
          {currency.change_percent !== null && (
            <div className={economyRowStyles.changeContainer}>
              <span
                className={
                  currency.change_percent >= 0
                    ? economyRowStyles.changePositive
                    : economyRowStyles.changeNegative
                }>
                {currency.change_percent >= 0 ? '+' : ''}
                {currency.change_percent.toFixed(2)}%
              </span>
            </div>
          )}
        </div>
      </HoverCard>
    </div>
  );
}
