import { BookOpenIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import {
  ActDistributionChart,
  CharacterStatusCard,
  EmptyState,
  LoadingSpinner,
  PageLayout,
  WalkthroughDashboard,
} from '../components';
import { useCharacterManagement } from '../hooks/useCharacterManagement';

export const Route = createFileRoute('/walkthrough')({
  component: WalkthroughPage,
});

function WalkthroughPage() {
  const { activeCharacter, isLoading } = useCharacterManagement();

  if (isLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <div className='flex items-center justify-center py-12'>
            <LoadingSpinner />
          </div>
        </div>
      </div>
    );
  }

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
      {!isLoading && !activeCharacter && (
        <EmptyState
          icon={<BookOpenIcon className='h-12 w-12' />}
          title='No Active Character'
          description='Please select an active character to view walkthrough progress.'
        />
      )}
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
