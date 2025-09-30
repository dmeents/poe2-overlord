import { BookOpenIcon, ChartBarIcon } from '@heroicons/react/24/outline';
import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect } from 'react';
import {
  ActDistributionChart,
  CampaignInsights,
  CharacterStatusCard,
  EmptyState,
  LoadingSpinner,
  PageLayout,
  SectionHeader,
  WalkthroughActiveStepCard,
  WalkthroughGuide,
} from '../components';
import {
  useCharacterManagement,
  useWalkthroughEvents,
  useWalkthroughGuide,
} from '../hooks';
import type { CharacterWalkthroughProgress } from '../types/walkthrough';
import { WalkthroughService } from '../utils/walkthrough';
import { handleWikiClick } from '../utils/wiki-utils';

export const Route = createFileRoute('/walkthrough')({
  component: WalkthroughPage,
});

function WalkthroughPage() {
  const { activeCharacter, isLoading } = useCharacterManagement();
  const { guide, loading: guideLoading } = useWalkthroughGuide();

  // Use the walkthrough events hook for real-time updates
  const {
    progress,
    currentStep,
    previousStep,
    setProgress,
    setCurrentStep,
    setPreviousStep,
  } = useWalkthroughEvents(activeCharacter?.id || '', guide);

  // Load initial walkthrough progress data
  const loadWalkthroughData = useCallback(async () => {
    if (!activeCharacter?.id || !guide) return;

    try {
      // Load initial progress
      const characterProgressResponse =
        await invoke<CharacterWalkthroughProgress>(
          'get_character_walkthrough_progress',
          { characterId: activeCharacter.id }
        );

      setProgress(characterProgressResponse.progress);

      // Get step details from the guide using step IDs
      const steps = WalkthroughService.getStepsFromGuide(
        guide,
        characterProgressResponse.progress.current_step_id,
        characterProgressResponse.next_step_id,
        characterProgressResponse.previous_step_id
      );

      // Set current step if available
      if (steps.currentStep) {
        setCurrentStep(steps.currentStep);
      }

      // Set previous step if available
      if (steps.previousStep) {
        setPreviousStep(steps.previousStep);
      }
    } catch (err) {
      console.error('Failed to load walkthrough progress:', err);
    }
  }, [
    activeCharacter?.id,
    guide,
    setProgress,
    setCurrentStep,
    setPreviousStep,
  ]);

  // Load initial data when character and guide are available
  useEffect(() => {
    loadWalkthroughData();
  }, [loadWalkthroughData]);

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
              progress={progress}
              currentStep={currentStep ?? undefined}
              previousStep={previousStep ?? undefined}
              onAdvanceStep={
                progress.is_completed ? undefined : handleAdvanceStep
              }
              onPreviousStep={previousStep ? handlePreviousStep : undefined}
              onWikiClick={handleWikiClick}
              className='mb-6'
            />
          )}

          {guide && (
            <WalkthroughGuide
              guide={guide}
              currentStepId={undefined} // Will be handled by the dashboard
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
