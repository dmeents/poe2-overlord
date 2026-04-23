import { StarIcon } from '@heroicons/react/24/outline';
import { EconomyRow } from '@/components/economy/economy-row/economy-row';
import { Card } from '@/components/ui/card/card';
import { LoadingSpinner } from '@/components/ui/loading-spinner/loading-spinner';
import { useEconomy } from '@/contexts/EconomyContext';
import { searchResultToExchangeRate } from '@/types/economy';

export function StarredCurrenciesCard() {
  const { starredCurrencies, isLoadingStarred, toggleStar } = useEconomy();

  const renderContent = () => {
    if (isLoadingStarred) {
      return <LoadingSpinner className="py-6" />;
    }

    if (starredCurrencies.length === 0) {
      return (
        <div className="text-center py-6 text-stone-500 text-sm">
          Star currencies from the Economy page to track them here.
        </div>
      );
    }

    return (
      <div>
        {starredCurrencies.map(currency => (
          <EconomyRow
            key={`${currency.economy_type}:${currency.id}`}
            currency={searchResultToExchangeRate(currency)}
            economyType={currency.economy_type}
            isStarred={true}
            onToggleStar={toggleStar}
            variant="compact"
            compactValueDisplay={{
              value: currency.primary_value,
              imageUrl: currency.primary_currency_image_url,
              imageName: currency.primary_currency_name,
            }}
          />
        ))}
      </div>
    );
  };

  return (
    <Card title="Starred" icon={<StarIcon />} accentColor="molten">
      {renderContent()}
    </Card>
  );
}
