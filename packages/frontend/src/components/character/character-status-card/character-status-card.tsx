import { Link } from '@tanstack/react-router';
import { useCharacterManagement } from '../../../hooks/useCharacterManagement';
import { EmptyState } from '../../ui/empty-state/empty-state';
import { LoadingSpinner } from '../../ui/loading-spinner/loading-spinner';
import { Button } from '../../ui/button/button';
import { CharacterCard } from '../character-card/character-card';
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
        <h3 className={characterStatusCardStyles.title}>Active Character</h3>
        <div className='flex items-center justify-center py-8'>
          <LoadingSpinner message='Loading character data...' />
        </div>
      </div>
    );
  }

  if (!activeCharacter) {
    return (
      <div className={`${characterStatusCardStyles.container} ${className}`}>
        <h3 className={characterStatusCardStyles.title}>Active Character</h3>
        <EmptyState
          icon={
            <div className='h-12 w-12 rounded-full bg-zinc-700 flex items-center justify-center text-zinc-400 text-xl'>
              👤
            </div>
          }
          title='No Active Character'
          description='Create or select a character to start tracking'
          action={
            <Link to='/characters'>
              <Button variant='primary' size='sm'>
                Manage Characters
              </Button>
            </Link>
          }
        />
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
        showDetails={false}
      />
    </div>
  );
}
