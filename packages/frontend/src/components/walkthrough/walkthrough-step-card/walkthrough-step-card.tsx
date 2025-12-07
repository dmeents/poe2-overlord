import {
  ArrowRightIcon,
  GiftIcon,
  MapPinIcon,
  StarIcon,
} from '@heroicons/react/24/outline';
import React, { useMemo } from 'react';
import type { WalkthroughStep } from '../../../types/walkthrough';
import { useZone } from '../../../contexts/ZoneContext';
import { ParsedText } from '../../../utils/text-parser';
import { Card } from '../../ui/card/card';
import { DataItem } from '../../ui/data-item/data-item';

interface WalkthroughStepCardProps {
  step: WalkthroughStep;
  isCurrent: boolean;
  onWikiClick: (itemName: string) => void;
  onSkipToStep?: (stepId: string) => void;
}

export const WalkthroughStepCard: React.FC<WalkthroughStepCardProps> = ({
  step,
  isCurrent,
  onWikiClick,
  onSkipToStep,
}) => {
  const { openZone } = useZone();
  // Filter out zone names from wiki items so they don't become wiki links
  const filteredWikiItems = useMemo(() => {
    const zoneNames = [step.current_zone, step.completion_zone].filter(Boolean);

    return step.wiki_items.filter(item => !zoneNames.includes(item));
  }, [step]);

  // Custom handler that checks if clicked item is a zone
  const handleItemClick = (itemName: string) => {
    const zoneNames = [step.current_zone, step.completion_zone];

    if (zoneNames.includes(itemName)) {
      openZone(itemName);
    } else {
      onWikiClick(itemName);
    }
  };

  return (
    <Card
      className={`${isCurrent ? 'border-blue-500 bg-blue-500/10' : ''} pb-1`}
    >
      <div className='flex items-start justify-between mb-2'>
        <div className='flex items-center gap-2'>
          <h4 className='font-medium text-white'>{step.title}</h4>
          {isCurrent && (
            <span className='inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-500/20 text-blue-400'>
              Current
            </span>
          )}
        </div>
        <div className='flex items-center gap-1 text-sm text-zinc-400'>
          <MapPinIcon className='w-3 h-3' />
          <button
            onClick={() => openZone(step.current_zone)}
            className='hover:text-zinc-200 hover:underline cursor-pointer transition-colors'
          >
            {step.current_zone}
          </button>
        </div>
      </div>

      <p className='text-sm text-zinc-300 mb-3'>
        <ParsedText
          text={step.description}
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
                onClick={() => openZone(step.completion_zone)}
                className='text-zinc-300 hover:text-zinc-200 underline decoration-blue-400 hover:decoration-blue-300 cursor-pointer font-medium'
              >
                {step.completion_zone}
              </button>
            </span>
          }
          value=''
          icon={<MapPinIcon className='w-4 h-4 text-blue-400' />}
          className='rounded-none'
        />

        {step.objectives.length > 0 && (
          <div>
            <h5 className='text-xs font-medium text-zinc-300 mb-2'>
              Objectives ({step.objectives.length}):
            </h5>
            <ul className='space-y-2'>
              {step.objectives.map((objective, index) => (
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
                            title={objective.required ? 'Required' : 'Optional'}
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

      {/* Skip Button */}
      {onSkipToStep && !isCurrent && (
        <div className='flex justify-end pt-1 border-t border-zinc-700/30'>
          <button
            onClick={e => {
              e.stopPropagation();
              onSkipToStep(step.id);
            }}
            className='inline-flex items-center gap-1 px-1.5 py-0.5 text-xs font-medium text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 rounded transition-colors cursor-pointer'
            title='Go to this step'
          >
            <ArrowRightIcon className='w-3 h-3' />
            Go Here
          </button>
        </div>
      )}
    </Card>
  );
};
