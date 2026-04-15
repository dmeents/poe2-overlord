import { StarIcon } from '@heroicons/react/24/solid';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';
import { calculateItemsSoldPerHour } from '@/utils/economy-utils';
import { hideOnError } from '@/utils/image-utils';
import { starredCurrenciesCardStyles } from './starred-currencies-card.styles';

export function StarredCurrenciesCard() {
  const { starredCurrencies, isLoadingStarred, toggleStar } = useEconomy();

  const renderContent = () => {
    if (isLoadingStarred) {
      return <LoadingSpinner className="py-6" />;
    }

    if (starredCurrencies.length === 0) {
      return (
        <div className={starredCurrenciesCardStyles.emptyState}>
          Star currencies from the Economy page to track them here.
        </div>
      );
    }

    return (
      <div>
        {starredCurrencies.map(currency => (
          <div
            key={`${currency.economy_type}:${currency.id}`}
            className={starredCurrenciesCardStyles.row}>
            <div className={starredCurrenciesCardStyles.leftSection}>
              <img
                src={currency.image_url}
                alt={currency.name}
                className={starredCurrenciesCardStyles.image}
                onError={hideOnError}
              />
              <div>
                <div className={starredCurrenciesCardStyles.name}>{currency.name}</div>
                {currency.volume !== null && (
                  <div className={starredCurrenciesCardStyles.volumeRow}>
                    {calculateItemsSoldPerHour(currency.volume, currency.primary_value)} / hr
                  </div>
                )}
              </div>
            </div>
            <div className={starredCurrenciesCardStyles.rightSection}>
              <div className={starredCurrenciesCardStyles.valueRow}>
                <span>
                  {currency.primary_value.toLocaleString('en-US', {
                    minimumFractionDigits: currency.primary_value >= 10 ? 1 : 2,
                    maximumFractionDigits: currency.primary_value >= 10 ? 1 : 2,
                  })}
                </span>
                <img
                  src={currency.primary_currency_image_url}
                  alt={currency.primary_currency_name}
                  className={starredCurrenciesCardStyles.valueIcon}
                  title={currency.primary_currency_name}
                  onError={hideOnError}
                />
                <button
                  type="button"
                  onClick={() => {
                    toggleStar(currency.id, currency.economy_type);
                  }}
                  title="Unstar currency"
                  className="text-amber-400 hover:text-amber-300 transition-colors focus:outline-none">
                  <StarIcon className="w-3.5 h-3.5" />
                </button>
              </div>
              {currency.change_percent !== null && (
                <span
                  className={
                    currency.change_percent >= 0
                      ? starredCurrenciesCardStyles.changePositive
                      : starredCurrenciesCardStyles.changeNegative
                  }>
                  {currency.change_percent >= 0 ? '+' : ''}
                  {currency.change_percent.toFixed(2)}%
                </span>
              )}
            </div>
          </div>
        ))}
      </div>
    );
  };

  return (
    <Card title="Starred" icon={<StarIcon className="w-5 h-5" />} accentColor="molten">
      {renderContent()}
    </Card>
  );
}
