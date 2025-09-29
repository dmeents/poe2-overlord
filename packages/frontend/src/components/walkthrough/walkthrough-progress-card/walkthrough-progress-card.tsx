import {
  ArrowLeftIcon,
  BookOpenIcon,
  CheckCircleIcon,
  ClockIcon,
  PlayCircleIcon,
} from '@heroicons/react/24/outline';
import React from 'react';
import { Button } from '../';
import type {
  WalkthroughProgress,
  WalkthroughStepResult,
} from '../../../types/walkthrough';

interface WalkthroughProgressCardProps {
  progress: WalkthroughProgress;
  currentStep?: WalkthroughStepResult;
  previousStep?: WalkthroughStepResult;
  onAdvanceStep?: () => void;
  onPreviousStep?: () => void;
  onViewGuide?: () => void;
  className?: string;
}

export const WalkthroughProgressCard: React.FC<
  WalkthroughProgressCardProps
> = ({
  progress,
  currentStep,
  previousStep,
  onAdvanceStep,
  onPreviousStep,
  onViewGuide,
  className = '',
}) => {
  // Check if previous step is available
  const hasPreviousStep = !!previousStep;
  const formatLastUpdated = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

  const getProgressStatus = () => {
    if (progress.is_completed) {
      return { text: 'Campaign Completed', variant: 'success' as const };
    }
    if (progress.current_step_id) {
      return { text: 'In Progress', variant: 'warning' as const };
    }
    return { text: 'Not Started', variant: 'secondary' as const };
  };

  const status = getProgressStatus();

  return (
    <div
      className={`bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm p-6 ${className}`}
    >
      <div className='flex items-center justify-between mb-4'>
        <h3 className='text-lg font-semibold text-gray-900 dark:text-white'>
          Walkthrough Progress
        </h3>
        <span
          className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
            status.variant === 'success'
              ? 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
              : status.variant === 'warning'
                ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400'
                : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
          }`}
        >
          {status.text}
        </span>
      </div>

      {progress.is_completed ? (
        <div className='text-center py-8'>
          <CheckCircleIcon className='w-16 h-16 text-green-500 mx-auto mb-4' />
          <h4 className='text-xl font-semibold text-gray-900 dark:text-white mb-2'>
            Campaign Complete!
          </h4>
          <p className='text-gray-600 dark:text-gray-400'>
            Congratulations on completing the Path of Exile 2 campaign.
          </p>
        </div>
      ) : currentStep ? (
        <div className='space-y-4'>
          <div>
            <h4 className='text-md font-medium text-gray-900 dark:text-white mb-2'>
              Current Step: {currentStep.step.title}
            </h4>
            <p className='text-sm text-gray-600 dark:text-gray-400 mb-3'>
              {currentStep.step.description}
            </p>
          </div>

          <div className='bg-gray-50 dark:bg-gray-800 rounded-lg p-4'>
            <div className='flex items-center gap-2 mb-2'>
              <PlayCircleIcon className='w-4 h-4 text-blue-500' />
              <span className='text-sm font-medium text-gray-900 dark:text-white'>
                Current Zone: {currentStep.step.current_zone}
              </span>
            </div>
            <div className='flex items-center gap-2'>
              <CheckCircleIcon className='w-4 h-4 text-green-500' />
              <span className='text-sm font-medium text-gray-900 dark:text-white'>
                Complete in: {currentStep.step.completion_zone}
              </span>
            </div>
          </div>

          {currentStep.step.objectives.length > 0 && (
            <div>
              <h5 className='text-sm font-medium text-gray-900 dark:text-white mb-2'>
                Objectives:
              </h5>
              <ul className='space-y-3'>
                {currentStep.step.objectives.map((objective, index) => (
                  <li key={index} className='text-gray-600 dark:text-gray-400'>
                    <div className='flex items-start gap-2'>
                      <div className='w-2 h-2 rounded-full mt-2 flex-shrink-0 bg-gray-300' />
                      <div className='flex-1 space-y-1'>
                        <div className='text-sm font-medium'>
                          {objective.text}
                        </div>
                        {objective.details && (
                          <div className='text-xs text-gray-500 dark:text-gray-400 pl-2 border-l-2 border-gray-200 dark:border-gray-600'>
                            {objective.details}
                          </div>
                        )}
                        {objective.notes && (
                          <div className='text-xs text-blue-600 dark:text-blue-400 pl-2 italic'>
                            Note: {objective.notes}
                          </div>
                        )}
                        {objective.rewards && objective.rewards.length > 0 && (
                          <div className='text-xs text-purple-600 dark:text-purple-400 pl-2'>
                            Rewards: {objective.rewards.join(', ')}
                          </div>
                        )}
                        {objective.required !== undefined && (
                          <div className='text-xs text-orange-600 dark:text-orange-400 pl-2'>
                            {objective.required ? 'Required' : 'Optional'}
                          </div>
                        )}
                      </div>
                    </div>
                  </li>
                ))}
              </ul>
            </div>
          )}

          <div className='flex gap-2 pt-2'>
            {hasPreviousStep && onPreviousStep && (
              <Button
                onClick={onPreviousStep}
                variant='outline'
                size='sm'
                className='flex-1'
              >
                <ArrowLeftIcon className='w-4 h-4 mr-2' />
                Previous Step
              </Button>
            )}
            {onAdvanceStep && (
              <Button
                onClick={onAdvanceStep}
                variant='primary'
                size='sm'
                className='flex-1'
              >
                Advance Step
              </Button>
            )}
            {onViewGuide && (
              <Button
                onClick={onViewGuide}
                variant='secondary'
                size='sm'
                className='flex-1'
              >
                <BookOpenIcon className='w-4 h-4 mr-2' />
                View Guide
              </Button>
            )}
          </div>
        </div>
      ) : (
        <div className='text-center py-8'>
          <BookOpenIcon className='w-16 h-16 text-gray-400 mx-auto mb-4' />
          <h4 className='text-lg font-semibold text-gray-900 dark:text-white mb-2'>
            No Active Step
          </h4>
          <p className='text-gray-600 dark:text-gray-400 mb-4'>
            Start your walkthrough to begin tracking progress.
          </p>
          {onViewGuide && (
            <Button onClick={onViewGuide} variant='primary'>
              <BookOpenIcon className='w-4 h-4 mr-2' />
              View Walkthrough Guide
            </Button>
          )}
        </div>
      )}

      <div className='mt-4 pt-4 border-t border-gray-200 dark:border-gray-700'>
        <div className='flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400'>
          <ClockIcon className='w-3 h-3' />
          Last updated: {formatLastUpdated(progress.last_updated)}
        </div>
      </div>
    </div>
  );
};
