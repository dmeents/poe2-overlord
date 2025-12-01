import { BookOpenIcon, ChartBarIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useState } from 'react';
import { ActDistributionChart } from '../components/charts/act-distribution-chart/act-distribution-chart';
import { CampaignInsights } from '../components/insights/campaign-insights/campaign-insights';
import { CharacterStatusCard } from '../components/character/character-status-card/character-status-card';
import { EmptyState } from '../components/ui/empty-state/empty-state';
import { LoadingSpinner } from '../components/ui/loading-spinner/loading-spinner';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { SectionHeader } from '../components/ui/section-header/section-header';
import { WalkthroughActiveStepCard } from '../components/walkthrough/walkthrough-active-step-card/walkthrough-active-step-card';
import { WalkthroughGuide } from '../components/walkthrough/walkthrough-guide/walkthrough-guide';
import { ZoneDetailsModal } from '../components/zones/zone-details-modal/zone-details-modal';
import { useCharacter } from '../contexts/CharacterContext';
import { useWalkthrough } from '../contexts/WalkthroughContext';
import type { ZoneStats } from '../types/character';
import { handleWikiClick } from '../utils/wiki-utils';

export const Route = createFileRoute('/walkthrough')({
  component: WalkthroughPage,
});

function WalkthroughPage() {
  const { activeCharacter, isLoading } = useCharacter();
  const { guide, guideLoading, progress, currentStep, previousStep } =
    useWalkthrough();

  console.log(activeCharacter);

  // Zone modal state
  const [selectedZone, setSelectedZone] = useState<ZoneStats | null>(null);
  const [isZoneModalOpen, setIsZoneModalOpen] = useState(false);

  // Advance to next step
  const handleAdvanceStep = async () => {
    if (!currentStep || !progress) return;

    try {
      // Get the next step ID from the current step
      const nextStepId = currentStep.step.next_step_id;
      if (!nextStepId) {
        console.error('No next step available. Campaign may be completed.');
        return;
      }

      // Create new progress with next step
      const newProgress = {
        ...progress,
        current_step_id: nextStepId,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId: activeCharacter?.id,
        progress: newProgress,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to advance step:', err);
    }
  };

  // Go to previous step
  const handlePreviousStep = async () => {
    if (!previousStep || !progress) return;

    try {
      // Create new progress with previous step
      const newProgress = {
        ...progress,
        current_step_id: previousStep.step.id,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId: activeCharacter?.id,
        progress: newProgress,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to go to previous step:', err);
    }
  };

  // Handle zone click from walkthrough cards
  const handleZoneClick = useCallback(
    (zoneName: string) => {
      if (!activeCharacter?.zones) return;

      // Find the zone in the active character's zones
      const zone = activeCharacter.zones.find(z => z.zone_name === zoneName);

      if (zone) {
        // Zone found in character's visited zones
        setSelectedZone(zone);
      } else {
        // Zone not visited yet - create a placeholder
        const placeholderZone: ZoneStats = {
          zone_name: zoneName,
          duration: 0,
          deaths: 0,
          visits: 0,
          first_visited: new Date().toISOString(),
          last_visited: new Date().toISOString(),
          is_active: false,
          entry_timestamp: undefined,
          area_id: undefined,
          act: undefined,
          area_level: undefined,
          is_town: false,
          has_waypoint: false,
          bosses: [],
          monsters: [],
          npcs: [],
          connected_zones: [],
          description: undefined,
          points_of_interest: [],
          image_url: undefined,
          wiki_url: undefined,
          last_updated: undefined,
        };
        setSelectedZone(placeholderZone);
      }

      setIsZoneModalOpen(true);
    },
    [activeCharacter?.zones]
  );

  const handleZoneModalClose = useCallback(() => {
    setIsZoneModalOpen(false);
  }, []);

  const handleZoneChange = useCallback((zone: ZoneStats | null) => {
    setSelectedZone(zone);
  }, []);

  if (isLoading || guideLoading) {
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
        <>
          <SectionHeader
            title='Progress'
            icon={<ChartBarIcon className='w-4 h-4' />}
          />
          {/* Active Step Card */}
          {progress && (
            <WalkthroughActiveStepCard
              key={`${progress.current_step_id}-${progress.last_updated}`}
              progress={progress}
              currentStep={currentStep ?? undefined}
              previousStep={previousStep ?? undefined}
              onAdvanceStep={
                progress.is_completed ? undefined : handleAdvanceStep
              }
              onPreviousStep={previousStep ? handlePreviousStep : undefined}
              onWikiClick={handleWikiClick}
              onZoneClick={handleZoneClick}
              className='mb-6'
            />
          )}

          {guide && (
            <WalkthroughGuide
              guide={guide}
              currentStepId={progress?.current_step_id || undefined}
              characterId={activeCharacter.id}
              onZoneClick={handleZoneClick}
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

  return (
    <>
      <PageLayout leftColumn={leftColumn} rightColumn={rightColumn} />
      <ZoneDetailsModal
        zone={selectedZone}
        isOpen={isZoneModalOpen}
        onClose={handleZoneModalClose}
        allZones={activeCharacter?.zones || []}
        onZoneChange={handleZoneChange}
      />
    </>
  );
}
