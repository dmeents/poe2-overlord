import { GiftIcon, StarIcon } from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import type { Objective, StepLink } from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';

interface StepObjectiveListProps {
  objectives: Objective[];
  links: StepLink[];
  onLinkClick: (link: StepLink) => void;
  hideOptionalObjectives?: boolean;
  hideLeagueStartObjectives?: boolean;
  hideObjectiveDescriptions?: boolean;
}

/**
 * Renders the objectives section with bullets, stars, details, and rewards
 */
export function StepObjectiveList({
  objectives,
  links,
  onLinkClick,
  hideOptionalObjectives,
  hideLeagueStartObjectives,
  hideObjectiveDescriptions,
}: StepObjectiveListProps): React.JSX.Element | null {
  const filteredObjectives = useMemo(() => {
    return objectives.filter(obj => {
      if (hideOptionalObjectives && !obj.required && !obj.leagueStart) return false;
      if (hideLeagueStartObjectives && obj.leagueStart) return false;
      return true;
    });
  }, [objectives, hideOptionalObjectives, hideLeagueStartObjectives]);

  if (objectives.length === 0) return null;

  const isFiltered = filteredObjectives.length !== objectives.length;
  const countLabel = isFiltered
    ? `Objectives (${filteredObjectives.length}/${objectives.length}):`
    : `Objectives (${objectives.length}):`;

  return (
    <div className="space-y-3 px-4">
      <h5 className="text-sm font-semibold text-stone-200">{countLabel}</h5>
      <ul className="space-y-4">
        {filteredObjectives.map((objective, objectiveIndex) => (
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
                <div className="font-medium text-stone-200 flex items-center gap-2 flex-wrap">
                  <ParsedText text={objective.text} links={links} onLinkClick={onLinkClick} />
                  {objective.leagueStart && (
                    <span
                      className="text-[10px] font-semibold text-molten-400"
                      title="Only needs to be completed on your first character per league">
                      LEAGUE START
                    </span>
                  )}
                </div>

                {/* Details and rewards */}
                {(!hideObjectiveDescriptions ||
                  (objective.rewards && objective.rewards.length > 0)) && (
                  <div className="border-l-2 border-ember-500/30 pl-3 space-y-1.5">
                    {!hideObjectiveDescriptions && objective.details && (
                      <div className="text-xs text-stone-400 whitespace-pre-wrap">
                        <ParsedText
                          text={objective.details}
                          links={links}
                          onLinkClick={onLinkClick}
                        />
                      </div>
                    )}
                    {objective.rewards && objective.rewards.length > 0 && (
                      <div className="text-xs flex items-center gap-1 text-stone-400">
                        <GiftIcon className="w-3.5 h-3.5 text-molten-400" title="Rewards" />
                        <ParsedText
                          text={objective.rewards.join(', ')}
                          links={links}
                          onLinkClick={onLinkClick}
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
