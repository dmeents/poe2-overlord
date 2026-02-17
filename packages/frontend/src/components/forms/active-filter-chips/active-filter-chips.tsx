import { memo } from 'react';
import type { ActiveChip } from '../../../hooks/useListControls';
import { Button } from '../../ui/button/button';
import {
  chipClasses,
  chipContainerClasses,
  chipRemoveButtonClasses,
  clearAllButtonClasses,
} from './active-filter-chips.styles';

interface ActiveFilterChipsProps {
  chips: ActiveChip[];
  onClearAll?: () => void;
}

/**
 * Renders active filter chips with remove buttons
 *
 * @example
 * <ActiveFilterChips
 *   chips={[
 *     { key: 'league', label: 'League: Standard', onRemove: () => {} },
 *     { key: 'class', label: 'Class: Warrior', onRemove: () => {} },
 *   ]}
 *   onClearAll={() => {}}
 * />
 */
export const ActiveFilterChips = memo(function ActiveFilterChips({
  chips,
  onClearAll,
}: ActiveFilterChipsProps) {
  if (chips.length === 0) {
    return null;
  }

  return (
    <div className={chipContainerClasses}>
      {chips.map(chip => (
        <span key={chip.key} className={chipClasses}>
          {chip.label}
          <button
            type="button"
            onClick={chip.onRemove}
            className={chipRemoveButtonClasses}
            aria-label={`Remove ${chip.label} filter`}>
            ×
          </button>
        </span>
      ))}
      {onClearAll && (
        <Button onClick={onClearAll} variant="ghost" size="sm" className={clearAllButtonClasses}>
          Clear all
        </Button>
      )}
    </div>
  );
});
