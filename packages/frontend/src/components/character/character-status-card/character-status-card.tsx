import { Link } from '@tanstack/react-router';
import { useCharacter } from '../../../contexts/CharacterContext';
import { Button } from '../../ui/button/button';
import { Card } from '../../ui/card/card';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { LoadingSpinner } from '../../ui/loading-spinner/loading-spinner';
import { CharacterCard } from '../character-card/character-card';

interface CharacterStatusCardProps {
  className?: string;
}

export function CharacterStatusCard({ className = '' }: CharacterStatusCardProps) {
  const { activeCharacter, isLoading } = useCharacter();

  if (isLoading) {
    return (
      <Card title="Active Character" className={className}>
        <LoadingSpinner message="Loading character data..." className="py-8" />
      </Card>
    );
  }

  if (!activeCharacter) {
    return (
      <Card title="Active Character" className={className}>
        <EmptyState
          icon={
            <div className="h-12 w-12 rounded-full bg-stone-700 flex items-center justify-center text-stone-400 text-xl">
              👤
            </div>
          }
          title="No Active Character"
          description="Create or select a character to start tracking"
          action={
            <Link to="/characters">
              <Button variant="primary" size="sm">
                Manage Characters
              </Button>
            </Link>
          }
        />
      </Card>
    );
  }

  const { zones: _zones, ...summary } = activeCharacter;

  return (
    <div className={className}>
      <CharacterCard
        character={{ ...summary, is_active: true }}
        isActive={true}
        interactive={false}
      />
    </div>
  );
}
