import { Link } from '@tanstack/react-router';
import { useCharacterManagement } from '../../hooks';
import { Button } from '../button';
import { CharacterCard } from '../character-card';
import { characterStatusCardStyles } from './character-status-card.styles';

interface CharacterStatusCardProps {
  className?: string;
}

export function CharacterStatusCard({
  className = '',
}: CharacterStatusCardProps) {
  const { activeCharacter, isLoading } = useCharacterManagement();

  if (isLoading) {
    return (
      <div className={`${characterStatusCardStyles.container} ${className}`}>
        <div className={characterStatusCardStyles.loadingContainer}>
          <div className={characterStatusCardStyles.loadingTitle}></div>
          <div className={characterStatusCardStyles.loadingSubtitle}></div>
          <div className={characterStatusCardStyles.loadingText}></div>
        </div>
      </div>
    );
  }

  if (!activeCharacter) {
    return (
      <div className={`${characterStatusCardStyles.container} ${className}`}>
        <h3 className={characterStatusCardStyles.title}>Active Character</h3>
        <div className={characterStatusCardStyles.emptyState}>
          <p>No active character selected</p>
          <p className={characterStatusCardStyles.emptyStateSubtext}>
            Create or select a character to start tracking
          </p>
          <Link to='/characters'>
            <Button variant='primary' size='sm'>
              Manage Characters
            </Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className={className}>
      <CharacterCard
        character={activeCharacter}
        isActive={true}
        onSelect={() => {}} // No-op since it's already the active character
        onEdit={() => {}} // No-op since interactive is false
        onDelete={() => {}} // No-op since interactive is false
        interactive={false}
      />
    </div>
  );
}
