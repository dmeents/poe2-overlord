import type { WalkthroughGuide, WalkthroughStepResult } from '../types/walkthrough';

export function findStepInGuide(
  guide: WalkthroughGuide,
  stepId: string,
): WalkthroughStepResult | null {
  for (const act of Object.values(guide.acts)) {
    if (act.steps[stepId]) {
      return {
        step: act.steps[stepId],
        act_name: act.act_name,
        act_number: act.act_number,
      };
    }
  }
  return null;
}

/**
 * Get step details from guide using step ID
 */
export function getStepFromGuide(
  guide: WalkthroughGuide,
  stepId: string | null,
): WalkthroughStepResult | null {
  if (!stepId) return null;
  return findStepInGuide(guide, stepId);
}
