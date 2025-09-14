import { Button } from '@/components/button';
import { useCharacterTimeTracking } from '@/hooks/useCharacterTimeTracking';
import { useState } from 'react';

export function DangerSection() {
  const { activeCharacter, clearAllData } = useCharacterTimeTracking();
  const [isClearing, setIsClearing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const handleClearAllData = async () => {
    if (!activeCharacter) {
      setError('No active character selected. Please select a character first.');
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
      setSuccess(`All time tracking data for ${activeCharacter.name} has been cleared successfully.`);

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
    <div className='space-y-4'>
      {/* Error and Success Messages */}
      {error && (
        <div className='bg-red-900/20 border border-red-800 text-red-300 px-4 py-3 rounded-lg'>
          <strong>Error:</strong> {error}
        </div>
      )}

      {success && (
        <div className='bg-green-900/20 border border-green-800 text-green-300 px-4 py-3 rounded-lg'>
          <strong>Success:</strong> {success}
        </div>
      )}

      {/* Clear All Data Section */}
      <div className='border border-red-800 rounded-lg p-4 bg-red-950/10'>
        <div className='flex items-start justify-between'>
          <div className='flex-1'>
            <h3 className='text-lg font-medium text-red-400 mb-2'>
              Clear Character Time Tracking Data
            </h3>
            <p className='text-zinc-400 text-sm mb-3'>
              Permanently delete all time tracking data for the active character including:
            </p>
            <ul className='text-zinc-400 text-sm list-disc list-inside space-y-1 mb-4'>
              <li>All active and completed sessions</li>
              <li>Location statistics and play time history</li>
              <li>Session summaries and analytics</li>
              <li>All historical tracking data</li>
            </ul>
            <p className='text-red-300 text-sm font-medium'>
              ⚠️ This action cannot be undone!
            </p>
          </div>
          <div className='ml-6'>
            <Button
              onClick={handleClearAllData}
              disabled={isClearing || !activeCharacter}
              variant='outline'
              size='sm'
              className='text-red-400 hover:text-red-300 border-red-600 hover:border-red-500 hover:bg-red-950/20'
            >
              {isClearing ? 'Clearing...' : 'Clear Character Data'}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
