import type { WalkthroughGuide, WalkthroughStepResult } from '../types/walkthrough';

/**
 * Finds a step by its ID across all acts.
 * Returns the step with its act context if found, null otherwise.
 */
export function findStepInGuide(
  guide: WalkthroughGuide,
  stepId: string,
): WalkthroughStepResult | null {
  for (let actIdx = 0; actIdx < guide.acts.length; actIdx++) {
    const act = guide.acts[actIdx];
    const stepIdx = act.steps.findIndex(step => step.id === stepId);
    if (stepIdx !== -1) {
      return {
        step: act.steps[stepIdx],
        act_name: act.act_name,
        act_number: actIdx + 1, // 1-based index
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

/**
 * Gets the ID of the next step after the given step ID.
 * Returns null if the step is not found or is the last step in the guide.
 */
export function getNextStepId(guide: WalkthroughGuide, stepId: string): string | null {
  for (let actIdx = 0; actIdx < guide.acts.length; actIdx++) {
    const act = guide.acts[actIdx];
    const stepIdx = act.steps.findIndex(step => step.id === stepId);

    if (stepIdx !== -1) {
      // Try next step in current act
      if (stepIdx + 1 < act.steps.length) {
        return act.steps[stepIdx + 1].id;
      }

      // Try first step of next act
      if (actIdx + 1 < guide.acts.length && guide.acts[actIdx + 1].steps.length > 0) {
        return guide.acts[actIdx + 1].steps[0].id;
      }

      return null; // Last step
    }
  }

  return null; // Step not found
}

/**
 * Gets the ID of the previous step before the given step ID.
 * Returns null if the step is not found or is the first step in the guide.
 */
export function getPreviousStepId(guide: WalkthroughGuide, stepId: string): string | null {
  for (let actIdx = 0; actIdx < guide.acts.length; actIdx++) {
    const act = guide.acts[actIdx];
    const stepIdx = act.steps.findIndex(step => step.id === stepId);

    if (stepIdx !== -1) {
      // Try previous step in current act
      if (stepIdx > 0) {
        return act.steps[stepIdx - 1].id;
      }

      // Try last step of previous act
      if (actIdx > 0) {
        const prevAct = guide.acts[actIdx - 1];
        if (prevAct.steps.length > 0) {
          return prevAct.steps[prevAct.steps.length - 1].id;
        }
      }

      return null; // First step
    }
  }

  return null; // Step not found
}
