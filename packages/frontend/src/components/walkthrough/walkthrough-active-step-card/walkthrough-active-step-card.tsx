import {
  ArrowLeftIcon,
  ArrowRightIcon,
  BookOpenIcon,
  CheckCircleIcon,
  ClockIcon,
  GiftIcon,
  MapPinIcon,
  StarIcon,
} from '@heroicons/react/24/outline';
import { invoke } from '@tauri-apps/api/core';
import React, { useMemo } from 'react';
import { useCharacter } from '../../../contexts/CharacterContext';
import { useWalkthrough } from '../../../contexts/WalkthroughContext';
import { useZone } from '../../../contexts/ZoneContext';
import { ParsedText } from '../../../utils/text-parser';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';

interface WalkthroughActiveStepCardProps {
  onWikiClick: (itemName: string) => void;
  onViewGuide?: () => void;
  className?: string;
}

export const WalkthroughActiveStepCard: React.FC<
  WalkthroughActiveStepCardProps
> = ({ onWikiClick, onViewGuide, className = '' }) => {
  // Get data from contexts
  const { activeCharacter } = useCharacter();
  const { progress, currentStep, previousStep } = useWalkthrough();
  const { openZone } = useZone();

  // Check if previous step is available
  const hasPreviousStep = !!previousStep;

  const formatLastUpdated = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

  // Advance to next step
  const handleAdvanceStep = async () => {
    if (!currentStep || !progress || !activeCharacter) return;

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
        characterId: activeCharacter.id,
        progress: newProgress,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to advance step:', err);
    }
  };

  // Go to previous step
  const handlePreviousStep = async () => {
    if (!previousStep || !progress || !activeCharacter) return;

    try {
      // Create new progress with previous step
      const newProgress = {
        ...progress,
        current_step_id: previousStep.step.id,
        is_completed: false,
        last_updated: new Date().toISOString(),
      };

      await invoke('update_character_walkthrough_progress', {
        characterId: activeCharacter.id,
        progress: newProgress,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to go to previous step:', err);
    }
  };

  // Filter out zone names from wiki items so they don't become wiki links
  const filteredWikiItems = useMemo(() => {
    if (!currentStep) return [];

    const zoneNames = [
      currentStep.step.current_zone,
      currentStep.step.completion_zone,
    ].filter(Boolean);

    return currentStep.step.wiki_items.filter(
      item => !zoneNames.includes(item)
    );
  }, [currentStep]);

  // Custom handler that checks if clicked item is a zone
  const handleItemClick = (itemName: string) => {
    if (!currentStep) return;

    const zoneNames = [
      currentStep.step.current_zone,
      currentStep.step.completion_zone,
    ];

    if (zoneNames.includes(itemName)) {
      openZone(itemName);
    } else {
      onWikiClick(itemName);
    }
  };

  // Early return if no progress available
  if (!progress) {
    return null;
  }

  return (
    <Card
      className={`${className} ${
        progress.is_completed
          ? 'border-green-500 bg-green-500/10'
          : 'border-blue-500 bg-blue-500/10'
      } pb-1`}
    >
      {progress.is_completed ? (
        <div className='text-center py-8'>
          <CheckCircleIcon className='w-16 h-16 text-green-500 mx-auto mb-4' />
          <h4 className='text-xl font-semibold text-white mb-2'>
            Campaign Complete!
          </h4>
          <p className='text-zinc-300'>
            Congratulations on completing the Path of Exile 2 campaign.
          </p>
        </div>
      ) : currentStep ? (
        <div className='space-y-4'>
          <div className='flex items-start justify-between mb-2'>
            <div className='flex items-center gap-2'>
              <h4 className='font-medium text-white'>
                {currentStep.step.title}
              </h4>
            </div>
            <div className='flex items-center gap-1 text-sm text-zinc-400'>
              <MapPinIcon className='w-3 h-3' />
              <button
                onClick={() => openZone(currentStep.step.current_zone)}
                className='hover:text-zinc-200 hover:underline cursor-pointer transition-colors'
              >
                {currentStep.step.current_zone}
              </button>
            </div>
          </div>

          <p className='text-sm text-zinc-300 mb-3'>
            <ParsedText
              text={currentStep.step.description}
              wikiItems={filteredWikiItems}
              onWikiClick={handleItemClick}
            />
          </p>

          <div className='space-y-6 mb-8'>
            <DataItem
              label={
                <span className='text-zinc-300 font-medium'>
                  Enter{' '}
                  <button
                    onClick={() => openZone(currentStep.step.completion_zone)}
                    className='text-zinc-300 hover:text-zinc-200 underline decoration-blue-400 hover:decoration-blue-300 cursor-pointer font-medium'
                  >
                    {currentStep.step.completion_zone}
                  </button>
                </span>
              }
              value=''
              icon={<MapPinIcon className='w-4 h-4 text-blue-400' />}
              className='rounded-none'
            />

            {currentStep.step.objectives.length > 0 && (
              <div>
                <h5 className='text-xs font-medium text-zinc-300 mb-2'>
                  Objectives ({currentStep.step.objectives.length}):
                </h5>
                <ul className='space-y-2'>
                  {currentStep.step.objectives.map((objective, index) => (
                    <li key={index} className='text-xs'>
                      <div className='flex items-start gap-2'>
                        <div className='w-1.5 h-1.5 rounded-full bg-zinc-400 mt-1.5 flex-shrink-0' />
                        <div className='flex-1 space-y-1'>
                          <div className='font-medium text-zinc-300 flex items-center gap-2'>
                            {objective.required !== undefined && (
                              <StarIcon
                                className={`w-3 h-3 ${
                                  objective.required
                                    ? 'text-orange-400'
                                    : 'text-zinc-400'
                                }`}
                                title={
                                  objective.required ? 'Required' : 'Optional'
                                }
                              />
                            )}
                            <ParsedText
                              text={objective.text}
                              wikiItems={filteredWikiItems}
                              onWikiClick={handleItemClick}
                            />
                          </div>
                          {(objective.details ||
                            objective.notes ||
                            (objective.rewards &&
                              objective.rewards.length > 0)) && (
                            <div className='border-l-2 border-zinc-600 pl-2 ml-1.5 space-y-1'>
                              {objective.details && (
                                <div className='text-xs text-zinc-500'>
                                  <ParsedText
                                    text={objective.details}
                                    wikiItems={filteredWikiItems}
                                    onWikiClick={handleItemClick}
                                  />
                                </div>
                              )}
                              {objective.notes && (
                                <div className='text-xs text-blue-400 italic'>
                                  Note:{' '}
                                  <ParsedText
                                    text={objective.notes}
                                    wikiItems={filteredWikiItems}
                                    onWikiClick={handleItemClick}
                                  />
                                </div>
                              )}
                              {objective.rewards &&
                                objective.rewards.length > 0 && (
                                  <div className='text-xs flex items-center gap-1'>
                                    <GiftIcon
                                      className='w-3 h-3 text-purple-400'
                                      title='Rewards'
                                    />
                                    <ParsedText
                                      text={objective.rewards.join(', ')}
                                      wikiItems={filteredWikiItems}
                                      onWikiClick={handleItemClick}
                                    />
                                  </div>
                                )}
                            </div>
                          )}
                        </div>
                      </div>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        </div>
      ) : (
        <div className='text-center py-8'>
          <BookOpenIcon className='w-16 h-16 text-zinc-400 mx-auto mb-4' />
          <h4 className='text-lg font-semibold text-white mb-2'>
            No Active Step
          </h4>
          <p className='text-zinc-300 mb-4'>
            Start your walkthrough to begin tracking progress.
          </p>
          {onViewGuide && (
            <button
              onClick={onViewGuide}
              className='inline-flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors'
            >
              <BookOpenIcon className='w-4 h-4' />
              View Walkthrough Guide
            </button>
          )}
        </div>
      )}

      {/* Footer with navigation buttons and last updated */}
      <div className='flex justify-between items-center pt-1 border-t border-zinc-700/30'>
        <div className='flex items-center gap-2 text-xs text-zinc-500'>
          <ClockIcon className='w-3 h-3' />
          Last updated: {formatLastUpdated(progress.last_updated)}
        </div>

        <div className='flex gap-2'>
          {hasPreviousStep && (
            <button
              onClick={handlePreviousStep}
              className='inline-flex items-center gap-1 px-1.5 py-0.5 text-xs font-medium text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 rounded transition-colors cursor-pointer'
              title='Previous step'
            >
              <ArrowLeftIcon className='w-3 h-3' />
              Previous
            </button>
          )}
          {!progress.is_completed && (
            <button
              onClick={handleAdvanceStep}
              className='inline-flex items-center gap-1 px-1.5 py-0.5 text-xs font-medium text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 rounded transition-colors cursor-pointer'
              title='Next step'
            >
              Next
              <ArrowRightIcon className='w-3 h-3' />
            </button>
          )}
          {onViewGuide && (
            <button
              onClick={onViewGuide}
              className='inline-flex items-center gap-1 px-1.5 py-0.5 text-xs font-medium text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 rounded transition-colors cursor-pointer'
              title='View guide'
            >
              <BookOpenIcon className='w-3 h-3' />
              Guide
            </button>
          )}
        </div>
      </div>
    </Card>
  );
};
