import { createFileRoute } from '@tanstack/react-router';
import { MapPinIcon } from '@heroicons/react/24/outline';
import { CharacterStatusCard } from '../components/character/character-status-card/character-status-card';
import { ZoneCard } from '../components/zones/zone-card/zone-card';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { WalkthroughActiveStepCard } from '../components/walkthrough/walkthrough-active-step-card/walkthrough-active-step-card';
import { Card } from '../components/ui/card/card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { useCharacter } from '../contexts/CharacterContext';
import { useWalkthrough } from '../contexts/WalkthroughContext';
import { handleWikiClick } from '../utils/wiki-utils';
import { ExchangeRatesCard } from '../components/economy/exchange-rates-card/exchange-rates-card';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { activeCharacter } = useCharacter();
  const { progress } = useWalkthrough();
  const activeZone = activeCharacter?.zones?.find(zone => zone.is_active);

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && progress && (
        <WalkthroughActiveStepCard
          key={`${progress.current_step_id}-${progress.last_updated}`}
          onWikiClick={handleWikiClick}
          className='mt-6'
        />
      )}
    </>
  );

  const rightColumn = (
    <>
      {activeCharacter && (
        <>
          {activeZone ? (
            <ZoneCard
              zone={activeZone}
              allZones={activeCharacter.zones || []}
            />
          ) : (
            <Card
              title='Current Location'
              icon={<MapPinIcon className='w-5 h-5' />}
              className='mt-6'
            >
              <EmptyState
                icon={<MapPinIcon className='w-12 h-12' />}
                title='No active zone'
                description='Start playing to track your location'
              />
            </Card>
          )}
        </>
      )}
      <ExchangeRatesCard />
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
