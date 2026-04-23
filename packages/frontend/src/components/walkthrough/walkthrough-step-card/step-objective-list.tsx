import { GiftIcon, StarIcon } from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import type { Objective, StepLink } from '../../../types/walkthrough';
import { ParsedText } from '../../../utils/text-parser';
import { stepObjectiveListStyles as s } from './step-objective-list.styles';

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
    <div className={s.container}>
      <h5 className={s.heading}>{countLabel}</h5>
      <ul className={s.list}>
        {filteredObjectives.map((objective, objectiveIndex) => (
          <li key={`objective-${objectiveIndex}-${objective.text.slice(0, 20)}`} className={s.item}>
            <div className={s.itemRow}>
              <StarIcon
                className={
                  objective.required !== undefined && objective.required
                    ? s.starRequired
                    : s.starOptional
                }
                title={
                  objective.required !== undefined && objective.required ? 'Required' : 'Optional'
                }
              />
              <div className={s.contentWrapper}>
                <div className={s.mainText}>
                  <ParsedText text={objective.text} links={links} onLinkClick={onLinkClick} />
                  {objective.leagueStart && (
                    <span
                      className={s.leagueStartBadge}
                      title="Only needs to be completed on your first character per league">
                      LEAGUE START
                    </span>
                  )}
                </div>
                {(!hideObjectiveDescriptions ||
                  (objective.rewards && objective.rewards.length > 0)) && (
                  <div className={s.detailsBlock}>
                    {!hideObjectiveDescriptions && objective.details && (
                      <div className={s.details}>
                        <ParsedText
                          text={objective.details}
                          links={links}
                          onLinkClick={onLinkClick}
                        />
                      </div>
                    )}
                    {objective.rewards && objective.rewards.length > 0 && (
                      <div className={s.rewardsRow}>
                        <GiftIcon className={s.rewardsIcon} title="Rewards" />
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
