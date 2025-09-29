import {
  ArrowPathIcon,
  BookOpenIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
} from '@heroicons/react/24/outline';
import { invoke } from '@tauri-apps/api/core';
import React, { useCallback, useEffect, useState } from 'react';
import { useWalkthroughEvents } from '../../hooks/useWalkthroughEvents';
import type {
  WalkthroughGuide,
  WalkthroughProgress,
  WalkthroughStepResult,
} from '../../types/walkthrough';
import { Button } from '../button/button';
import { WalkthroughGuideViewer } from '../walkthrough-guide-viewer';
import { WalkthroughProgressCard } from '../walkthrough-progress-card';

interface WalkthroughDashboardProps {
  characterId: string;
  className?: string;
}

export const WalkthroughDashboard: React.FC<WalkthroughDashboardProps> = ({
  characterId,
  className = '',
}) => {
  const [guide, setGuide] = useState<WalkthroughGuide | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showGuide, setShowGuide] = useState(false);

  // Use the walkthrough events hook for real-time updates
  const {
    progress,
    currentStep,
    previousStep,
    isListening,
    setProgress,
    setCurrentStep,
    setPreviousStep,
  } = useWalkthroughEvents(characterId);

  // Load walkthrough guide (progress is handled by events)
  const loadWalkthroughData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      // Load guide
      const guideResponse = await invoke<WalkthroughGuide>(
        'get_walkthrough_guide'
      );
      setGuide(guideResponse);

      // Load initial progress (events will handle updates)
      const characterProgressResponse = await invoke<{
        progress: WalkthroughProgress;
        current_step: WalkthroughStepResult | null;
        next_step: WalkthroughStepResult | null;
        previous_step: WalkthroughStepResult | null;
      }>('get_character_walkthrough_progress', {
        characterId,
      });

      setProgress(characterProgressResponse.progress);

      // Set current step if available
      if (characterProgressResponse.current_step) {
        setCurrentStep(characterProgressResponse.current_step);
      }

      // Set previous step if available
      if (characterProgressResponse.previous_step) {
        setPreviousStep(characterProgressResponse.previous_step);
      }
    } catch (err) {
      console.error('Failed to load walkthrough data:', err);
      setError('Failed to load walkthrough data. Please try again.');
    } finally {
      setLoading(false);
    }
  }, [characterId, setProgress, setCurrentStep, setPreviousStep]);

  // Advance to next step
  const handleAdvanceStep = async () => {
    try {
      await invoke('advance_character_walkthrough_step', {
        characterId,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to advance step:', err);
      setError('Failed to advance step. Please try again.');
    }
  };

  // Go to previous step
  const handlePreviousStep = async () => {
    if (!previousStep) return;

    try {
      await invoke('move_character_to_walkthrough_step', {
        characterId,
        stepId: previousStep.step.id,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to go to previous step:', err);
      setError('Failed to go to previous step. Please try again.');
    }
  };

  // Mark campaign as completed
  const handleCompleteCampaign = async () => {
    try {
      await invoke('mark_character_campaign_completed', {
        characterId,
      });
      // Events will handle the UI update
    } catch (err) {
      console.error('Failed to complete campaign:', err);
      setError('Failed to complete campaign. Please try again.');
    }
  };

  // Load data on mount
  useEffect(() => {
    loadWalkthroughData();
  }, [loadWalkthroughData]);

  if (loading) {
    return (
      <div className={`flex items-center justify-center p-8 ${className}`}>
        <div className='text-center'>
          <ArrowPathIcon className='w-8 h-8 animate-spin text-blue-500 mx-auto mb-4' />
          <p className='text-gray-600 dark:text-gray-400'>
            Loading walkthrough data...
          </p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className={`p-6 ${className}`}>
        <div className='bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-6'>
          <div className='flex items-center gap-3 mb-4'>
            <ExclamationTriangleIcon className='w-6 h-6 text-red-500' />
            <h3 className='text-lg font-semibold text-red-900 dark:text-red-100'>
              Error Loading Walkthrough
            </h3>
          </div>
          <p className='text-red-700 dark:text-red-300 mb-4'>{error}</p>
          <Button onClick={loadWalkthroughData} variant='primary'>
            <ArrowPathIcon className='w-4 h-4 mr-2' />
            Try Again
          </Button>
        </div>
      </div>
    );
  }

  if (!guide) {
    return (
      <div className={`p-6 ${className}`}>
        <div className='bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm p-6 text-center'>
          <BookOpenIcon className='w-16 h-16 text-gray-400 mx-auto mb-4' />
          <h3 className='text-lg font-semibold text-gray-900 dark:text-white mb-2'>
            No Walkthrough Guide Available
          </h3>
          <p className='text-gray-600 dark:text-gray-400 mb-4'>
            The walkthrough guide could not be loaded. Please check your
            configuration.
          </p>
          <Button onClick={loadWalkthroughData} variant='primary'>
            <ArrowPathIcon className='w-4 h-4 mr-2' />
            Retry
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className='flex items-center justify-between'>
        <div>
          <h2 className='text-2xl font-bold text-gray-900 dark:text-white'>
            Walkthrough Guide
          </h2>
          <p className='text-gray-600 dark:text-gray-400'>
            Track your progress through the Path of Exile 2 campaign
            {isListening && (
              <span className='ml-2 inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'>
                Live Updates
              </span>
            )}
          </p>
        </div>
        <div className='flex gap-2'>
          <Button
            onClick={() => setShowGuide(!showGuide)}
            variant={showGuide ? 'secondary' : 'primary'}
          >
            <BookOpenIcon className='w-4 h-4 mr-2' />
            {showGuide ? 'Hide Guide' : 'View Guide'}
          </Button>
          <Button onClick={loadWalkthroughData} variant='outline'>
            <ArrowPathIcon className='w-4 h-4' />
          </Button>
        </div>
      </div>

      {/* Progress Card */}
      {progress && (
        <WalkthroughProgressCard
          progress={progress}
          currentStep={currentStep ?? undefined}
          previousStep={previousStep ?? undefined}
          onAdvanceStep={progress.is_completed ? undefined : handleAdvanceStep}
          onPreviousStep={previousStep ? handlePreviousStep : undefined}
          onViewGuide={() => setShowGuide(true)}
        />
      )}

      {/* Quick Actions */}
      {progress && !progress.is_completed && (
        <div className='bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm p-4'>
          <div className='flex items-center justify-between'>
            <div>
              <h3 className='font-medium text-gray-900 dark:text-white'>
                Quick Actions
              </h3>
              <p className='text-sm text-gray-600 dark:text-gray-400'>
                Manage your walkthrough progress
              </p>
            </div>
            <div className='flex gap-2'>
              <Button onClick={handleCompleteCampaign} variant='outline'>
                <CheckCircleIcon className='w-4 h-4 mr-2' />
                Mark Complete
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Guide Viewer */}
      {showGuide && (
        <WalkthroughGuideViewer
          guide={guide}
          currentStepId={progress?.current_step_id || undefined}
          onStepSelect={stepId => {
            // Could implement step selection logic here
            console.log('Selected step:', stepId);
          }}
        />
      )}

      {/* Stats */}
      {guide && (
        <div className='bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm p-6'>
          <h3 className='text-lg font-semibold text-gray-900 dark:text-white mb-4'>
            Guide Statistics
          </h3>
          <div className='grid grid-cols-2 md:grid-cols-4 gap-4'>
            <div className='text-center'>
              <div className='text-2xl font-bold text-blue-600 dark:text-blue-400'>
                {Object.keys(guide.acts).length}
              </div>
              <div className='text-sm text-gray-600 dark:text-gray-400'>
                Acts
              </div>
            </div>
            <div className='text-center'>
              <div className='text-2xl font-bold text-green-600 dark:text-green-400'>
                {Object.values(guide.acts).reduce(
                  (total, act) => total + Object.keys(act.steps).length,
                  0
                )}
              </div>
              <div className='text-sm text-gray-600 dark:text-gray-400'>
                Total Steps
              </div>
            </div>
            <div className='text-center'>
              <div className='text-2xl font-bold text-purple-600 dark:text-purple-400'>
                {Object.values(guide.acts).reduce(
                  (total, act) =>
                    total +
                    Object.values(act.steps).reduce(
                      (stepTotal, step) => stepTotal + step.objectives.length,
                      0
                    ),
                  0
                )}
              </div>
              <div className='text-sm text-gray-600 dark:text-gray-400'>
                Objectives
              </div>
            </div>
            <div className='text-center'>
              <div className='text-2xl font-bold text-orange-600 dark:text-orange-400'>
                {Object.values(guide.acts).reduce(
                  (total, act) =>
                    total +
                    Object.values(act.steps).reduce(
                      (stepTotal, step) => stepTotal + step.wiki_items.length,
                      0
                    ),
                  0
                )}
              </div>
              <div className='text-sm text-gray-600 dark:text-gray-400'>
                Wiki Links
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
