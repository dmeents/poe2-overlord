import { MagnifyingGlassIcon } from '@heroicons/react/24/outline';

interface FilteredEmptyStateProps {
  /**
   * The type of items being filtered (e.g., "characters", "zones")
   */
  itemType: string;
  /**
   * Callback when the "Clear All Filters" button is clicked
   */
  onClearFilters: () => void;
}

/**
 * Empty state component shown when filters result in no matches.
 * Displays a search icon, message, and "Clear All Filters" button.
 */
export function FilteredEmptyState({ itemType, onClearFilters }: FilteredEmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center py-16 px-6 text-center">
      <div className="w-16 h-16 bg-stone-800/50 flex items-center justify-center mb-4">
        <MagnifyingGlassIcon className="w-8 h-8 text-stone-500" />
      </div>
      <h3 className="text-lg font-medium text-stone-300 mb-2">No {itemType} found</h3>
      <p className="text-stone-500 mb-4 max-w-md">
        No {itemType} match your current search and filter criteria. Try adjusting your filters or
        search terms.
      </p>
      <button
        type="button"
        onClick={onClearFilters}
        className="px-4 py-2 text-sm font-medium text-ember-400 hover:text-ember-300 bg-ember-500/10 hover:bg-ember-500/20 border border-ember-500/30 transition-colors">
        Clear All Filters
      </button>
    </div>
  );
}
