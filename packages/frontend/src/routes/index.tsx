import { MapPinIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { open } from '@tauri-apps/plugin-shell';
import { CharacterStatusCard } from '../components/character/character-status-card/character-status-card';
import { ExchangeRatesCard } from '../components/economy/exchange-rates-card/exchange-rates-card';
import { StarredCurrenciesCard } from '../components/economy/starred-currencies-card/starred-currencies-card';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { LevelingStatsCard } from '../components/leveling/leveling-stats-card/leveling-stats-card';
import { PinnedNotesCard } from '../components/notes/pinned-notes-card/pinned-notes-card';
import { Card } from '../components/ui/card/card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { WalkthroughStepCard } from '../components/walkthrough/walkthrough-step-card/walkthrough-step-card';
import { CurrentZoneCard } from '../components/zones/current-zone-card/current-zone-card';
import { useCharacter } from '../contexts/CharacterContext';
import { useWalkthrough } from '../contexts/WalkthroughContext';
import { useZone } from '../contexts/ZoneContext';
import type { StepLink } from '../types/walkthrough';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { activeCharacter } = useCharacter();
  const { progress } = useWalkthrough();
  const { allZones } = useZone();
  const activeZone = allZones.find(zone => zone.is_active);

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
      <PinnedNotesCard />
    </>
  );

  const rightColumn = (
    <>
      <ExchangeRatesCard />
      <StarredCurrenciesCard />
      {activeZone ? (
        <CurrentZoneCard zone={activeZone} />
      ) : (
        <Card title="Current Location" icon={<MapPinIcon />}>
          <EmptyState
            icon={<MapPinIcon className="w-12 h-12" />}
            title={activeCharacter ? 'No active zone' : 'No Active Character'}
            description={
              activeCharacter
                ? 'Start playing to track your location'
                : 'Select a character to track your location'
            }
          />
        </Card>
      )}
      <LevelingStatsCard />
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
