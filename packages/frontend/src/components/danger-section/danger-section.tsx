import { Button } from '@/components/button';
import { useCharacterTimeTracking } from '@/hooks/useCharacterTimeTracking';
import { useState } from 'react';
import { dangerSectionStyles } from './danger-section.styles';

export function DangerSection() {
  const { activeCharacter, clearAllData } = useCharacterTimeTracking();
  const [isClearing, setIsClearing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const handleClearAllData = async () => {
    if (!activeCharacter) {
      setError(
        'No active character selected. Please select a character first.'
      );
      setTimeout(() => setError(null), 5000);
      return;
    }

    if (
      !confirm(
        `Are you sure you want to clear all time tracking data for ${activeCharacter.name}? This action cannot be undone and will permanently delete all time tracking history, active sessions, and statistics for this character.`
      )
    ) {
      return;
    }

    // Double confirmation for extra safety
    if (
      !confirm(
        'This is your final warning. All time tracking data will be permanently deleted. Type "DELETE" in the next prompt to confirm.'
      )
    ) {
      return;
    }

    const confirmation = prompt(
      'Type "DELETE" to permanently clear all time tracking data:'
    );

    if (confirmation !== 'DELETE') {
      setError(
        'Operation cancelled. You must type "DELETE" exactly to confirm.'
      );
      setTimeout(() => setError(null), 5000);
      return;
    }

    try {
      setIsClearing(true);
      setError(null);
      setSuccess(null);

      await clearAllData();
      setSuccess(
        `All time tracking data for ${activeCharacter.name} has been cleared successfully.`
      );

      // Clear success message after 5 seconds
      setTimeout(() => setSuccess(null), 5000);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to clear time tracking data'
      );
      setTimeout(() => setError(null), 5000);
    } finally {
      setIsClearing(false);
    }
  };

  return (
    <div className={dangerSectionStyles.container}>
      {/* Error and Success Messages */}
      {error && (
        <div className={dangerSectionStyles.errorMessage}>
          <strong>Error:</strong> {error}
        </div>
      )}

      {success && (
        <div className={dangerSectionStyles.successMessage}>
          <strong>Success:</strong> {success}
        </div>
      )}

      {/* Clear All Data Section */}
      <div className={dangerSectionStyles.clearDataSection}>
        <div className={dangerSectionStyles.sectionHeader}>
          <div className={dangerSectionStyles.sectionContent}>
            <h3 className={dangerSectionStyles.title}>
              Clear Character Time Tracking Data
            </h3>
            <p className={dangerSectionStyles.description}>
              Permanently delete all time tracking data for the active character
              including:
            </p>
            <ul className={dangerSectionStyles.list}>
              <li>All active and completed sessions</li>
              <li>Location statistics and play time history</li>
              <li>Session summaries and analytics</li>
              <li>All historical tracking data</li>
            </ul>
            <p className={dangerSectionStyles.warning}>
              ⚠️ This action cannot be undone!
            </p>
          </div>
          <div className={dangerSectionStyles.buttonContainer}>
            <Button
              onClick={handleClearAllData}
              disabled={isClearing || !activeCharacter}
              variant='outline'
              size='sm'
              className={dangerSectionStyles.clearButton}
            >
              {isClearing ? 'Clearing...' : 'Clear Character Data'}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
