import { BookOpenIcon, ChartBarIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { open } from '@tauri-apps/plugin-shell';
import { ActDistributionChart } from '../components/charts/act-distribution-chart/act-distribution-chart';
import { CampaignInsights } from '../components/insights/campaign-insights/campaign-insights';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { Card } from '../components/ui/card/card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { LoadingSpinner } from '../components/ui/loading-spinner/loading-spinner';
import { SectionHeader } from '../components/ui/section-header/section-header';
import { CampaignCompleteCard } from '../components/walkthrough/campaign-complete-card/campaign-complete-card';
import { WalkthroughGuide } from '../components/walkthrough/walkthrough-guide/walkthrough-guide';
import { WalkthroughSettingsPanel } from '../components/walkthrough/walkthrough-settings-panel/walkthrough-settings-panel';
import { WalkthroughStepCard } from '../components/walkthrough/walkthrough-step-card/walkthrough-step-card';
import { useCharacter } from '../contexts/CharacterContext';
import { useWalkthrough } from '../contexts/WalkthroughContext';
import type { StepLink } from '../types/walkthrough';

export const Route = createFileRoute('/walkthrough')({
  component: WalkthroughPage,
});

function WalkthroughPage() {
  const { activeCharacter, isLoading } = useCharacter();
  const { guide, guideLoading, progress } = useWalkthrough();

  const handleLinkClick = async (link: StepLink) => {
    try {
      await open(link.url);
    } catch (error) {
      console.error('Failed to open link:', error);
    }
  };

  if (isLoading || guideLoading) {
    return (
      <div className="min-h-screen text-stone-50">
        <div className="px-6 py-8">
          <LoadingSpinner className="py-12" />
        </div>
      </div>
    );
  }

  const leftColumn = (
    <>
      {activeCharacter && (
        <>
          <Card title="Display Settings" className="mb-6">
            <WalkthroughSettingsPanel variant="inline" />
          </Card>
          <SectionHeader title="Progress" icon={<ChartBarIcon className="w-4 h-4" />} />
          {progress?.is_completed ? (
            <CampaignCompleteCard
              lastUpdated={progress.last_updated}
              character={activeCharacter}
              className="mb-6"
            />
          ) : (
            progress && (
              <WalkthroughStepCard
                key={`${progress.current_step_id}-${progress.last_updated}`}
                variant="active"
                onLinkClick={handleLinkClick}
                className="mb-6"
              />
            )
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
        <Card title="Campaign Progress">
          <EmptyState
            icon={<BookOpenIcon className="h-12 w-12" />}
            title="No Active Character"
            description="Please select an active character to view walkthrough progress."
          />
        </Card>
      )}
      {activeCharacter && guide && (
        <>
          <CampaignInsights guide={guide} className="mb-6" />
          <ActDistributionChart character={activeCharacter} />
        </>
      )}
    </>
  );

  return <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} showCharacterCard />;
}
