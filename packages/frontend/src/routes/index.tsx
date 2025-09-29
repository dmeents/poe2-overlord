import { createFileRoute } from '@tanstack/react-router';
import {
  ActDistributionChart,
  CharacterStatusCard,
  DashboardInsights,
  PageLayout,
  WalkthroughDashboard,
} from '../components';
import { useCharacterManagement } from '../hooks';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { activeCharacter } = useCharacterManagement();

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && (
        <WalkthroughDashboard characterId={activeCharacter.id} />
      )}
    </>
  );

  const rightColumn = (
    <>
      <DashboardInsights />
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
