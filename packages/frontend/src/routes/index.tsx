import { createFileRoute } from '@tanstack/react-router';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import { MapPinIcon } from '@heroicons/react/24/outline';
import { ActDistributionChart } from '../components/charts/act-distribution-chart/act-distribution-chart';
import { CharacterStatusCard } from '../components/character/character-status-card/character-status-card';
import { DashboardInsights } from '../components/insights/dashboard-insights/dashboard-insights';
import { ZoneCard } from '../components/zones/zone-card/zone-card';
import { PageLayout } from '../components/layout/page-layout/page-layout';
import { WalkthroughActiveStepCard } from '../components/walkthrough/walkthrough-active-step-card/walkthrough-active-step-card';
import { ZoneDetailsModal } from '../components/zones/zone-details-modal/zone-details-modal';
import { Card } from '../components/ui/card/card';
import { useCharacterManagement } from '../hooks/useCharacterManagement';
import { useWalkthroughEvents } from '../hooks/useWalkthroughEvents';
import { useWalkthroughGuide } from '../hooks/useWalkthroughGuide';
import type { ZoneStats } from '../types/character';
import type { CharacterWalkthroughProgress } from '../types/walkthrough';
import { WalkthroughService } from '../utils/walkthrough';
import { handleWikiClick } from '../utils/wiki-utils';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { activeCharacter } = useCharacterManagement();
  const { guide } = useWalkthroughGuide();

  // Zone modal state
  const [selectedZone, setSelectedZone] = useState<ZoneStats | null>(null);
  const [isZoneModalOpen, setIsZoneModalOpen] = useState(false);

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

  // Find the current active zone
  const activeZone = activeCharacter?.zones?.find(zone => zone.is_active);

  const leftColumn = (
    <>
      <CharacterStatusCard />
      {activeCharacter && (
        <>
          {activeZone ? (
            <ZoneCard
              zone={activeZone}
              allZones={activeCharacter.zones || []}
              className='mt-6'
            />
          ) : (
            <Card
              title='Current Location'
              icon={<MapPinIcon className='w-5 h-5' />}
              className='mt-6'
            >
              <div className='flex flex-col items-center justify-center py-8 text-zinc-400'>
                <MapPinIcon className='w-12 h-12 mb-3 opacity-50' />
                <p className='text-sm'>No active zone</p>
                <p className='text-xs text-zinc-500 mt-1'>
                  Start playing to track your location
                </p>
              </div>
            </Card>
          )}
        </>
      )}
      {activeCharacter && progress && (
        <WalkthroughActiveStepCard
          key={`${progress.current_step_id}-${progress.last_updated}`}
          progress={progress}
          currentStep={currentStep ?? undefined}
          previousStep={previousStep ?? undefined}
          onAdvanceStep={progress.is_completed ? undefined : handleAdvanceStep}
          onPreviousStep={previousStep ? handlePreviousStep : undefined}
          onWikiClick={handleWikiClick}
          onZoneClick={handleZoneClick}
          className='mt-6'
        />
      )}
    </>
  );

  const rightColumn = (
    <>
      <DashboardInsights />
      {activeCharacter && <ActDistributionChart character={activeCharacter} />}
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
