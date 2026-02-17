import { MapPinIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { open } from '@tauri-apps/plugin-shell';
import { CharacterStatusCard } from '../components/character/character-status-card/character-status-card';
import { ExchangeRatesCard } from '../components/economy/exchange-rates-card/exchange-rates-card';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { Card } from '../components/ui/card/card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { WalkthroughStepCard } from '../components/walkthrough/walkthrough-step-card/walkthrough-step-card';
import { CurrentZoneCard } from '../components/zones/current-zone-card/current-zone-card';
import { useCharacter } from '../contexts/CharacterContext';
import { useWalkthrough } from '../contexts/WalkthroughContext';
import type { StepLink } from '../types/walkthrough';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { activeCharacter } = useCharacter();
  const { progress } = useWalkthrough();
  const activeZone = activeCharacter?.zones?.find(zone => zone.is_active);

  const handleLinkClick = async (link: StepLink) => {
    try {
      await open(link.url);
    } catch (error) {
      console.error('Failed to open link:', error);
    }
  };

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && progress && !progress.is_completed && (
        <WalkthroughStepCard
          key={`${progress.current_step_id}-${progress.last_updated}`}
          variant="active"
          onLinkClick={handleLinkClick}
          className="mt-6"
        />
      )}
    </>
  );

  const rightColumn = (
    <>
      {activeCharacter && (
        <>
          <ExchangeRatesCard />
          {activeZone ? (
            <CurrentZoneCard zone={activeZone} />
          ) : (
            <Card title="Current Location" icon={<MapPinIcon className="w-5 h-5" />}>
              <EmptyState
                icon={<MapPinIcon className="w-12 h-12" />}
                title="No active zone"
                description="Start playing to track your location"
              />
            </Card>
          )}
        </>
      )}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
