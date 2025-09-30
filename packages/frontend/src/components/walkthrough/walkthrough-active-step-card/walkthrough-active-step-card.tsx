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
import React from 'react';
import type {
  WalkthroughProgress,
  WalkthroughStepResult,
} from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';

interface WalkthroughActiveStepCardProps {
  progress: WalkthroughProgress;
  currentStep?: WalkthroughStepResult;
  previousStep?: WalkthroughStepResult;
  onAdvanceStep?: () => void;
  onPreviousStep?: () => void;
  onViewGuide?: () => void;
  onWikiClick: (itemName: string) => void;
  className?: string;
}

export const WalkthroughActiveStepCard: React.FC<
  WalkthroughActiveStepCardProps
> = ({
  progress,
  currentStep,
  previousStep,
  onAdvanceStep,
  onPreviousStep,
  onViewGuide,
  onWikiClick,
  className = '',
}) => {
  // Debug logging
  console.log('WalkthroughActiveStepCard render:', {
    progress: progress
      ? {
          current_step_id: progress.current_step_id,
          is_completed: progress.is_completed,
          last_updated: progress.last_updated,
        }
      : null,
    currentStep: currentStep
      ? {
          step_id: currentStep.step.id,
          title: currentStep.step.title,
        }
      : null,
    previousStep: previousStep
      ? {
          step_id: previousStep.step.id,
          title: previousStep.step.title,
        }
      : null,
  });

  // Check if previous step is available
  const hasPreviousStep = !!previousStep;

  const formatLastUpdated = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

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
              {currentStep.step.current_zone}
            </div>
          </div>

          <p className='text-sm text-zinc-300 mb-3'>
            <ParsedText
              text={currentStep.step.description}
              wikiItems={currentStep.step.wiki_items}
              onWikiClick={onWikiClick}
            />
          </p>

          <div className='space-y-6 mb-8'>
            <DataItem
              label={
                <span className='text-zinc-300 font-medium'>
                  Enter{' '}
                  <ParsedText
                    text={currentStep.step.completion_zone}
                    wikiItems={currentStep.step.wiki_items}
                    onWikiClick={onWikiClick}
                  />
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
                              wikiItems={currentStep.step.wiki_items}
                              onWikiClick={onWikiClick}
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
                                    wikiItems={currentStep.step.wiki_items}
                                    onWikiClick={onWikiClick}
                                  />
                                </div>
                              )}
                              {objective.notes && (
                                <div className='text-xs text-blue-400 italic'>
                                  Note:{' '}
                                  <ParsedText
                                    text={objective.notes}
                                    wikiItems={currentStep.step.wiki_items}
                                    onWikiClick={onWikiClick}
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
                                      wikiItems={currentStep.step.wiki_items}
                                      onWikiClick={onWikiClick}
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
          {hasPreviousStep && onPreviousStep && (
            <button
              onClick={onPreviousStep}
              className='inline-flex items-center gap-1 px-1.5 py-0.5 text-xs font-medium text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 rounded transition-colors cursor-pointer'
              title='Previous step'
            >
              <ArrowLeftIcon className='w-3 h-3' />
              Previous
            </button>
          )}
          {onAdvanceStep && (
            <button
              onClick={onAdvanceStep}
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
