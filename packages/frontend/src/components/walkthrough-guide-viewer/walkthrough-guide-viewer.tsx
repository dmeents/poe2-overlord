import {
  ArrowTopRightOnSquareIcon,
  BookOpenIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  CircleStackIcon,
  PlayCircleIcon,
  TagIcon,
} from '@heroicons/react/24/outline';
import { open } from '@tauri-apps/plugin-shell';
import React, { useState } from 'react';
import type {
  WalkthroughAct,
  WalkthroughGuide,
  WalkthroughStep,
} from '../../types/walkthrough';
import { Modal } from '../modal/modal';

interface WalkthroughGuideViewerProps {
  guide: WalkthroughGuide;
  currentStepId?: string;
  onStepSelect?: (stepId: string) => void;
  className?: string;
}

export const WalkthroughGuideViewer: React.FC<WalkthroughGuideViewerProps> = ({
  guide,
  currentStepId,
  onStepSelect,
  className = '',
}) => {
  const [expandedActs, setExpandedActs] = useState<Set<string>>(new Set());
  const [selectedStep, setSelectedStep] = useState<WalkthroughStep | null>(
    null
  );

  const getWikiUrl = (itemName: string) => {
    // Convert item name to capitalized snake case for wiki URL
    const capitalizedSnakeCase = itemName
      .replace(/\s+/g, '_') // Replace spaces with underscores
      .replace(/[^a-zA-Z0-9_]/g, '') // Remove special characters except underscores
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join('_');

    return `https://www.poe2wiki.net/wiki/${capitalizedSnakeCase}`;
  };

  const handleWikiClick = async (itemName: string) => {
    const url = getWikiUrl(itemName);
    try {
      await open(url);
    } catch (error) {
      console.error('Failed to open wiki link:', error);
    }
  };

  const toggleAct = (actId: string) => {
    const newExpanded = new Set(expandedActs);
    if (newExpanded.has(actId)) {
      newExpanded.delete(actId);
    } else {
      newExpanded.add(actId);
    }
    setExpandedActs(newExpanded);
  };

  const handleStepClick = (step: WalkthroughStep) => {
    setSelectedStep(step);
    if (onStepSelect) {
      onStepSelect(step.id);
    }
  };

  const isCurrentStep = (stepId: string) => currentStepId === stepId;

  const renderStep = (step: WalkthroughStep) => {
    const isCurrent = isCurrentStep(step.id);

    return (
      <div
        key={step.id}
        className={`p-4 border rounded-lg cursor-pointer transition-colors ${
          isCurrent
            ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
            : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
        }`}
        onClick={() => handleStepClick(step)}
      >
        <div className='flex items-start justify-between mb-2'>
          <div className='flex items-center gap-2'>
            {isCurrent ? (
              <PlayCircleIcon className='w-5 h-5 text-blue-500' />
            ) : (
              <CircleStackIcon className='w-5 h-5 text-gray-400' />
            )}
            <h4 className='font-medium text-gray-900 dark:text-white'>
              {step.title}
            </h4>
            {isCurrent && (
              <span className='inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400'>
                Current
              </span>
            )}
          </div>
        </div>

        <p className='text-sm text-gray-600 dark:text-gray-400 mb-3'>
          {step.description}
        </p>

        <div className='space-y-2'>
          <div className='flex items-center gap-2 text-xs'>
            <TagIcon className='w-3 h-3 text-gray-500' />
            <span className='text-gray-500 dark:text-gray-400'>
              Complete in:{' '}
              <span className='font-medium'>{step.completion_zone}</span>
            </span>
          </div>

          {step.objectives.length > 0 && (
            <div>
              <h5 className='text-xs font-medium text-gray-700 dark:text-gray-300 mb-1'>
                Objectives ({step.objectives.length}):
              </h5>
              <ul className='space-y-1'>
                {step.objectives.slice(0, 3).map((objective, index) => (
                  <li
                    key={index}
                    className='text-xs text-gray-600 dark:text-gray-400'
                  >
                    <div className='flex items-start gap-2'>
                      <div className='w-1.5 h-1.5 rounded-full bg-gray-400 mt-1.5 flex-shrink-0' />
                      <div className='flex-1'>
                        <div>{objective.text}</div>
                        {objective.details && (
                          <div className='text-xs text-gray-500 dark:text-gray-500 mt-0.5 pl-1'>
                            {objective.details}
                          </div>
                        )}
                      </div>
                    </div>
                  </li>
                ))}
                {step.objectives.length > 3 && (
                  <li className='text-xs text-gray-500 dark:text-gray-500'>
                    +{step.objectives.length - 3} more objectives
                  </li>
                )}
              </ul>
            </div>
          )}

          {step.wiki_items.length > 0 && (
            <div className='flex flex-wrap gap-1'>
              {step.wiki_items.slice(0, 2).map((item, index) => (
                <button
                  key={index}
                  onClick={() => handleWikiClick(item)}
                  className='inline-flex items-center gap-1 text-xs text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline'
                >
                  <ArrowTopRightOnSquareIcon className='w-3 h-3' />
                  {item}
                </button>
              ))}
              {step.wiki_items.length > 2 && (
                <span className='text-xs text-gray-500 dark:text-gray-500'>
                  +{step.wiki_items.length - 2} more
                </span>
              )}
            </div>
          )}
        </div>
      </div>
    );
  };

  const renderAct = (act: WalkthroughAct, actKey: string) => {
    const isExpanded = expandedActs.has(actKey);

    return (
      <div
        key={actKey}
        className='bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm mb-4'
      >
        <div className='p-4 cursor-pointer' onClick={() => toggleAct(actKey)}>
          <div className='flex items-center justify-between'>
            <div className='flex items-center gap-3'>
              {isExpanded ? (
                <ChevronDownIcon className='w-5 h-5 text-gray-500' />
              ) : (
                <ChevronRightIcon className='w-5 h-5 text-gray-500' />
              )}
              <h3 className='text-lg font-semibold text-gray-900 dark:text-white'>
                {act.act_name}
              </h3>
              <span className='inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400'>
                {Object.keys(act.steps).length} steps
              </span>
            </div>
          </div>
          <p className='text-sm text-gray-600 dark:text-gray-400 mt-1 ml-8'>
            Act {act.act_number}
          </p>
        </div>

        {isExpanded && (
          <div className='px-4 pb-4 space-y-3'>
            {Object.values(act.steps)
              .sort((a, b) => {
                const aStepNum = parseInt(a.id.split('_').pop() || '0');
                const bStepNum = parseInt(b.id.split('_').pop() || '0');
                return aStepNum - bStepNum;
              })
              .map(step => renderStep(step))}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className={`space-y-4 ${className}`}>
      <div className='bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm p-6'>
        <div className='flex items-center gap-3 mb-4'>
          <BookOpenIcon className='w-6 h-6 text-blue-500' />
          <div>
            <h2 className='text-xl font-semibold text-gray-900 dark:text-white'>
              Walkthrough Guide
            </h2>
            <p className='text-sm text-gray-600 dark:text-gray-400'>
              Track your progress through the Path of Exile 2 campaign
            </p>
          </div>
        </div>

        <div className='grid grid-cols-1 md:grid-cols-3 gap-4 text-sm'>
          <div className='text-center p-3 bg-gray-50 dark:bg-gray-800 rounded-lg'>
            <div className='font-semibold text-gray-900 dark:text-white'>
              {Object.keys(guide.acts).length}
            </div>
            <div className='text-gray-600 dark:text-gray-400'>Acts</div>
          </div>
          <div className='text-center p-3 bg-gray-50 dark:bg-gray-800 rounded-lg'>
            <div className='font-semibold text-gray-900 dark:text-white'>
              {Object.values(guide.acts).reduce(
                (total, act) => total + Object.keys(act.steps).length,
                0
              )}
            </div>
            <div className='text-gray-600 dark:text-gray-400'>Total Steps</div>
          </div>
          <div className='text-center p-3 bg-gray-50 dark:bg-gray-800 rounded-lg'>
            <div className='font-semibold text-gray-900 dark:text-white'>
              {currentStepId ? 'In Progress' : 'Not Started'}
            </div>
            <div className='text-gray-600 dark:text-gray-400'>Status</div>
          </div>
        </div>
      </div>

      <div className='space-y-4'>
        {Object.entries(guide.acts)
          .sort(([, a], [, b]) => a.act_number - b.act_number)
          .map(([actKey, act]) => renderAct(act, actKey))}
      </div>

      {selectedStep && (
        <Modal
          isOpen={true}
          onClose={() => setSelectedStep(null)}
          title={selectedStep.title}
        >
          <div className='space-y-4'>
            <p className='text-gray-600 dark:text-gray-400'>
              {selectedStep.description}
            </p>

            <div className='grid grid-cols-1 md:grid-cols-2 gap-4'>
              <div>
                <h4 className='font-medium text-gray-900 dark:text-white mb-2'>
                  Current Zone
                </h4>
                <p className='text-sm text-gray-600 dark:text-gray-400'>
                  {selectedStep.current_zone}
                </p>
              </div>
              <div>
                <h4 className='font-medium text-gray-900 dark:text-white mb-2'>
                  Complete In
                </h4>
                <p className='text-sm text-gray-600 dark:text-gray-400'>
                  {selectedStep.completion_zone}
                </p>
              </div>
            </div>

            {selectedStep.objectives.length > 0 && (
              <div>
                <h4 className='font-medium text-gray-900 dark:text-white mb-2'>
                  Objectives
                </h4>
                <ul className='space-y-3'>
                  {selectedStep.objectives.map((objective, index) => (
                    <li key={index} className='text-sm'>
                      <div className='flex items-start gap-2'>
                        <div className='w-2 h-2 rounded-full bg-gray-400 mt-2 flex-shrink-0' />
                        <div className='flex-1 space-y-1'>
                          <div className='font-medium text-gray-900 dark:text-white'>
                            {objective.text}
                          </div>
                          {objective.details && (
                            <div className='text-sm text-gray-600 dark:text-gray-400 pl-2 border-l-2 border-gray-200 dark:border-gray-600'>
                              {objective.details}
                            </div>
                          )}
                          {objective.notes && (
                            <div className='text-sm text-blue-600 dark:text-blue-400 pl-2 italic'>
                              Note: {objective.notes}
                            </div>
                          )}
                          {objective.rewards &&
                            objective.rewards.length > 0 && (
                              <div className='text-sm text-purple-600 dark:text-purple-400 pl-2'>
                                Rewards: {objective.rewards.join(', ')}
                              </div>
                            )}
                          {objective.required !== undefined && (
                            <div className='text-sm text-orange-600 dark:text-orange-400 pl-2'>
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

            {selectedStep.wiki_items.length > 0 && (
              <div>
                <h4 className='font-medium text-gray-900 dark:text-white mb-2'>
                  Related Resources
                </h4>
                <div className='space-y-2'>
                  {selectedStep.wiki_items.map((item, index) => (
                    <button
                      key={index}
                      onClick={() => handleWikiClick(item)}
                      className='flex items-center gap-2 text-sm text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 hover:underline'
                    >
                      <ArrowTopRightOnSquareIcon className='w-4 h-4' />
                      {item}
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>
        </Modal>
      )}
    </div>
  );
};
