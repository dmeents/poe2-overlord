import { GiftIcon, StarIcon } from '@heroicons/react/24/outline';
import type { WalkthroughObjective } from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';

interface StepObjectiveListProps {
  objectives: WalkthroughObjective[];
  wikiItems: string[];
  onWikiClick: (itemName: string) => void;
}

/**
 * Renders the objectives section with bullets, stars, details, notes, and rewards
 */
export function StepObjectiveList({
  objectives,
  wikiItems,
  onWikiClick,
}: StepObjectiveListProps): React.JSX.Element | null {
  if (objectives.length === 0) return null;

  return (
    <div className="space-y-3 px-4">
      <h5 className="text-sm font-semibold text-stone-200">Objectives ({objectives.length}):</h5>
      <ul className="space-y-4">
        {objectives.map((objective, objectiveIndex) => (
          <li
            key={`objective-${objectiveIndex}-${objective.text.slice(0, 20)}`}
            className="text-sm">
            <div className="flex items-start gap-2.5">
              {/* Star icon as leading indicator */}
              <StarIcon
                className={
                  objective.required !== undefined && objective.required
                    ? 'w-4 h-4 text-ember-400 flex-shrink-0 mt-0.5'
                    : 'w-4 h-4 text-stone-400 flex-shrink-0 mt-0.5'
                }
                title={
                  objective.required !== undefined && objective.required ? 'Required' : 'Optional'
                }
              />

              {/* Objective content */}
              <div className="flex-1 space-y-1.5">
                {/* Main text */}
                <div className="font-medium text-stone-200">
                  <ParsedText
                    text={objective.text}
                    wikiItems={wikiItems}
                    onWikiClick={onWikiClick}
                  />
                </div>

                {/* Details, notes, rewards */}
                {(objective.details ||
                  objective.notes ||
                  (objective.rewards && objective.rewards.length > 0)) && (
                  <div className="border-l-2 border-ember-500/30 pl-3 space-y-1.5">
                    {objective.details && (
                      <div className="text-xs text-stone-400">
                        <ParsedText
                          text={objective.details}
                          wikiItems={wikiItems}
                          onWikiClick={onWikiClick}
                        />
                      </div>
                    )}
                    {objective.notes && (
                      <div className="text-xs text-stone-300 italic">
                        Note:{' '}
                        <ParsedText
                          text={objective.notes}
                          wikiItems={wikiItems}
                          onWikiClick={onWikiClick}
                        />
                      </div>
                    )}
                    {objective.rewards && objective.rewards.length > 0 && (
                      <div className="text-xs flex items-center gap-1 text-stone-400">
                        <GiftIcon className="w-3.5 h-3.5 text-molten-400" title="Rewards" />
                        <ParsedText
                          text={objective.rewards.join(', ')}
                          wikiItems={wikiItems}
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
  );
}
