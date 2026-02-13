import { ArrowRightIcon } from '@heroicons/react/24/outline';

interface StepDestinationProps {
  completionZone: string;
  onZoneClick: (zoneName: string) => void;
}

/**
 * Renders the destination banner showing where the player needs to go
 */
export function StepDestination({
  completionZone,
  onZoneClick,
}: StepDestinationProps): React.JSX.Element {
  return (
    <div className="flex items-center gap-2 text-sm">
      <ArrowRightIcon className="w-4 h-4 text-ember-400 flex-shrink-0" />
      <span className="text-stone-400">Head to</span>
      <button
        type="button"
        onClick={() => onZoneClick(completionZone)}
        className="text-stone-200 font-semibold hover:text-stone-100 underline decoration-ember-500/50 hover:decoration-ember-400 cursor-pointer">
        {completionZone}
      </button>
    </div>
  );
}
