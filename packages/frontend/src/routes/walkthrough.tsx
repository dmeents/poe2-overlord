import { BookOpenIcon, ChartBarIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { ActDistributionChart } from '../components/charts/act-distribution-chart/act-distribution-chart';
import { CampaignInsights } from '../components/insights/campaign-insights/campaign-insights';
import { CharacterStatusCard } from '../components/character/character-status-card/character-status-card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { LoadingSpinner } from '../components/ui/loading-spinner/loading-spinner';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { SectionHeader } from '../components/ui/section-header/section-header';
import { WalkthroughActiveStepCard } from '../components/walkthrough/walkthrough-active-step-card/walkthrough-active-step-card';
import { WalkthroughGuide } from '../components/walkthrough/walkthrough-guide/walkthrough-guide';
import { useCharacter } from '../contexts/CharacterContext';
import { useWalkthrough } from '../contexts/WalkthroughContext';
import { handleWikiClick } from '../utils/wiki-utils';

export const Route = createFileRoute('/walkthrough')({
  component: WalkthroughPage,
});

function WalkthroughPage() {
  const { activeCharacter, isLoading } = useCharacter();
  const { guide, guideLoading, progress } = useWalkthrough();

  if (isLoading || guideLoading) {
    return (
      <div className='min-h-screen bg-zinc-900 text-white'>
        <div className='px-6 py-8'>
          <LoadingSpinner className='py-12' />
        </div>
      </div>
    );
  }

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && (
        <>
          <SectionHeader
            title='Progress'
            icon={<ChartBarIcon className='w-4 h-4' />}
          />
          {progress && (
            <WalkthroughActiveStepCard
              key={`${progress.current_step_id}-${progress.last_updated}`}
              onWikiClick={handleWikiClick}
              className='mb-6'
            />
          )}
          {guide && (
            <WalkthroughGuide
              guide={guide}
              currentStepId={progress?.current_step_id || undefined}
              characterId={activeCharacter.id}
            />
          )}
        </>
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
      {activeCharacter && guide && (
        <>
          <CampaignInsights guide={guide} className='mb-6' />
          <ActDistributionChart character={activeCharacter} />
        </>
      )}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />;
}
